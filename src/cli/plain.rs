//! C-cli: a search with no `--ask` and no `--json` (FEATURE-23, decisions 7, 8,
//! 12, 13, 14, 15, and 19).
//!
//! The flow is the flow of `search` in rg 15.2.0 (`crates/core/main.rs`) with
//! one thread: the haystacks come in order, `grep-searcher` searches each one,
//! and `grep-printer` writes the result of each one to standard output. The
//! settings are the settings of rg (`crates/core/flags/hiargs.rs`). The flow
//! uses no model, no key, no cost line, and no progress line.

use std::io::{self, IsTerminal, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use grep_matcher::Matcher;
use grep_printer::{ColorSpecs, Standard, StandardBuilder, Summary, SummaryBuilder, SummaryKind};
use grep_searcher::{BinaryDetection, Searcher, SearcherBuilder, Sink};
use termcolor::{Buffer, BufferWriter, ColorChoice, ColorSpec, WriteColor};

use super::grep::{MatchOptions, Report, build_matcher, context_lines, file_filter, words};
use super::rg_text::{NO_PATTERN, NOTHING_SEARCHED, line};
use super::{Cli, colored, size_option};
use crate::config::{Env, plain_size_limit};
use crate::failure::Failure;
use crate::output::{SizeReport, size_notice};
use crate::search::filter::current_dir;
use crate::search::sequence::{Haystack, Item, Places, Sequence};
use crate::search::snippet::{char_count, char_prefix};

/// The bytes that the printer holds before they go to standard output.
const FLUSH_AT: usize = 64 * 1024;

/// Run the search and give the exit code (decision 12).
pub(super) fn run(cli: &Cli, env: &Env) -> ExitCode {
    // jag has no handler: a closed pipe is a write error, and SIGINT ends jag
    // by its default action, with no message, as it ends rg (decision 13).
    match search(cli, env) {
        Ok(code) => ExitCode::from(code),
        Err(text) => {
            let mut stderr = io::stderr();
            let _ = stderr.write_all(text.as_bytes());
            ExitCode::from(2)
        }
    }
}

/// The text of a failure with the form of jag (decision 19).
fn failure_text(failure: Failure) -> String {
    failure.render()
}

/// The size limit: the option, else `JAG_MAX_CHARS`, else none (decision 19).
fn size_limit(cli: &Cli, env: &Env) -> Result<Option<usize>, String> {
    let option = size_option(cli).map_err(|error| failure_text(Failure::from(error)))?;
    plain_size_limit(env, option).map_err(|error| failure_text(Failure::from(error)))
}

/// What the options say about the layout of the output.
struct Layout {
    with_filename: bool,
    heading: bool,
    line_number: bool,
}

impl Layout {
    /// The defaults of rg: the path when more than one file may be searched,
    /// the heading and the line numbers in a terminal.
    fn of(cli: &Cli, report: Report, places: &Places, stdout_is_terminal: bool) -> Layout {
        let with_filename = if cli.with_filename {
            true
        } else if cli.no_filename {
            false
        } else {
            !places.one_file
        };
        let heading = if cli.heading {
            true
        } else if cli.no_heading {
            false
        } else {
            stdout_is_terminal
        };
        let line_number = if cli.line_number {
            true
        } else if cli.no_line_number || report != Report::Lines {
            false
        } else {
            stdout_is_terminal && !places.only_stdin()
        };
        Layout {
            with_filename,
            heading,
            line_number,
        }
    }
}

/// The writer of the printer. It holds the bytes of the result of one
/// haystack, and gives them to standard output when it has many, or when the
/// search of the haystack ends. With a size limit it holds all of them, and the
/// caller takes them.
struct Out<'a> {
    buffer: Buffer,
    writer: &'a BufferWriter,
    collect: bool,
}

impl Out<'_> {
    /// Write the held bytes to standard output.
    fn flush_out(&mut self) -> io::Result<()> {
        self.writer.print(&self.buffer)?;
        self.buffer.clear();
        io::stdout().flush()
    }

    /// The held bytes, which the caller owns after this call.
    fn take(&mut self) -> Vec<u8> {
        let bytes = self.buffer.as_slice().to_vec();
        self.buffer.clear();
        bytes
    }
}

impl Write for Out<'_> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let count = self.buffer.write(bytes)?;
        if !self.collect && self.buffer.len() >= FLUSH_AT {
            self.flush_out()?;
        }
        Ok(count)
    }

    fn flush(&mut self) -> io::Result<()> {
        if self.collect {
            return Ok(());
        }
        self.flush_out()
    }
}

impl WriteColor for Out<'_> {
    fn supports_color(&self) -> bool {
        self.buffer.supports_color()
    }

    fn set_color(&mut self, spec: &ColorSpec) -> io::Result<()> {
        self.buffer.set_color(spec)
    }

    fn reset(&mut self) -> io::Result<()> {
        self.buffer.reset()
    }

    fn is_synchronous(&self) -> bool {
        false
    }
}

/// The printer of the report.
enum Printer<'a> {
    Standard(Standard<Out<'a>>),
    Summary(Summary<Out<'a>>),
}

impl<'a> Printer<'a> {
    fn out(&mut self) -> &mut Out<'a> {
        match self {
            Printer::Standard(printer) => printer.get_mut(),
            Printer::Summary(printer) => printer.get_mut(),
        }
    }
}

/// The separator between two files, as `file_separator` of rg: an empty line
/// with a heading, else `--` when lines around a match are on.
fn file_separator(layout: &Layout, report: Report, around: bool) -> Option<Vec<u8>> {
    if report != Report::Lines {
        return None;
    }
    if layout.heading {
        Some(Vec::new())
    } else if around {
        Some(b"--".to_vec())
    } else {
        None
    }
}

fn build_printer<'a>(
    report: Report,
    layout: &Layout,
    around: bool,
    writer: &'a BufferWriter,
    collect: bool,
) -> Printer<'a> {
    let specs = ColorSpecs::new(&grep_printer::default_color_specs());
    let out = Out {
        buffer: writer.buffer(),
        writer,
        collect,
    };
    let kind = match report {
        Report::Lines => {
            return Printer::Standard(
                StandardBuilder::new()
                    .color_specs(specs)
                    .heading(layout.heading)
                    .path(layout.with_filename)
                    .per_match_one_line(true)
                    .separator_context(Some(b"--".to_vec()))
                    .separator_field_context(b"-".to_vec())
                    .separator_field_match(b":".to_vec())
                    .separator_search(file_separator(layout, report, around))
                    .build(out),
            );
        }
        Report::Files => SummaryKind::PathWithMatch,
        Report::Counts => SummaryKind::Count,
        Report::Quiet => SummaryKind::QuietWithMatch,
    };
    Printer::Summary(
        SummaryBuilder::new()
            .color_specs(specs)
            .exclude_zero(true)
            .kind(kind)
            .path(layout.with_filename)
            .separator_field(b":".to_vec())
            .build(out),
    )
}

/// Search one haystack with `sink`.
fn search_haystack<M: Matcher, S: Sink<Error = io::Error>>(
    searcher: &mut Searcher,
    matcher: &M,
    haystack: &Haystack,
    sink: &mut S,
) -> io::Result<()> {
    if haystack.stdin {
        searcher.search_reader(matcher, io::stdin().lock(), sink)
    } else {
        searcher.search_path(matcher, &haystack.path, sink)
    }
}

/// What the search of one haystack found.
fn search_one<M: Matcher>(
    printer: &mut Printer,
    searcher: &mut Searcher,
    matcher: &M,
    haystack: &Haystack,
) -> io::Result<bool> {
    match printer {
        Printer::Standard(printer) => {
            let mut sink = printer.sink_with_path(matcher, &haystack.path);
            search_haystack(searcher, matcher, haystack, &mut sink)?;
            Ok(sink.has_match())
        }
        Printer::Summary(printer) => {
            let mut sink = printer.sink_with_path(matcher, &haystack.path);
            search_haystack(searcher, matcher, haystack, &mut sink)?;
            Ok(sink.has_match())
        }
    }
}

fn search(cli: &Cli, env: &Env) -> Result<u8, String> {
    let limit = size_limit(cli, env)?;
    let all_words = words(cli);
    // With -e, each word is a path. With no -e, the first word is the term.
    let (terms, path_words): (Vec<&str>, Vec<&std::ffi::OsStr>) = if cli.terms.is_empty() {
        let Some((term, rest)) = all_words.split_first() else {
            return Err(line(NO_PATTERN));
        };
        let Some(term) = term.to_str() else {
            return Err(line("pattern given is not valid UTF-8"));
        };
        (vec![term], rest.to_vec())
    } else {
        (cli.terms.iter().map(String::as_str).collect(), all_words)
    };
    let context = context_lines(cli, false)
        .map_err(failure_text)?
        .map_or((0, 0), |lines| (lines.before, lines.after));
    let cwd = current_dir();
    let files = file_filter(&cli.globs, &cwd).map_err(failure_text)?;
    let matcher =
        build_matcher(&terms, MatchOptions::of(cli)).map_err(|error| line(&error.to_string()))?;

    let places = Places::new(path_words.into_iter().map(PathBuf::from).collect());
    let report = Report::of(cli);
    let stdout_is_terminal = io::stdout().is_terminal();
    let layout = Layout::of(cli, report, &places, stdout_is_terminal);
    let choice = if colored(cli, env, stdout_is_terminal) {
        ColorChoice::Always
    } else {
        ColorChoice::Never
    };
    let writer = BufferWriter::stdout(choice);
    let around = context.0 > 0 || context.1 > 0;
    let mut printer = build_printer(report, &layout, around, &writer, limit.is_some());
    let mut searcher = SearcherBuilder::new()
        .line_number(layout.line_number)
        .before_context(usize::try_from(context.0).unwrap_or(usize::MAX))
        .after_context(usize::try_from(context.1).unwrap_or(usize::MAX))
        .build();

    let mut sizes = limit.map(|max| {
        Sizes::new(
            max,
            report == Report::Lines && around,
            layout.heading && report == Report::Lines,
        )
    });
    let mut matched = false;
    let mut searched = false;
    let mut errored = false;
    for item in Sequence::new(&places, &files, &cwd) {
        let haystack = match item {
            Item::Error(text) => {
                say(&text);
                errored = true;
                continue;
            }
            Item::IgnoreMessage(text) => {
                say(&text);
                continue;
            }
            Item::Haystack(haystack) => haystack,
        };
        searched = true;
        searcher.set_binary_detection(if haystack.explicit {
            BinaryDetection::convert(b'\0')
        } else {
            BinaryDetection::quit(b'\0')
        });
        let result = search_one(&mut printer, &mut searcher, &matcher, &haystack);
        let result = result.and_then(|found| {
            deliver(&mut printer, sizes.as_mut())?;
            Ok(found)
        });
        match result {
            Ok(found) => matched = matched || found,
            // A closed pipe is a graceful end.
            Err(error) if error.kind() == io::ErrorKind::BrokenPipe => break,
            Err(error) => {
                say(&format!("{}: {error}", haystack.path.display()));
                errored = true;
                continue;
            }
        }
        if matched && report == Report::Quiet {
            break;
        }
    }
    if places.implicit && !searched {
        say(NOTHING_SEARCHED);
        errored = true;
    }
    let mut limited = false;
    if let Some(sizes) = &sizes
        && let Some(max) = limit
    {
        let size = sizes.report();
        if let Some(notice) = size_notice(&size, Some(max)) {
            say_plain(&notice);
        }
        limited = size.limited();
    }
    Ok(if matched && (report == Report::Quiet || !errored) {
        if limited { 8 } else { 0 }
    } else if errored {
        2
    } else {
        1
    })
}

/// Give the result of one haystack to standard output: all of it, or with a
/// size limit the part that fits.
fn deliver(printer: &mut Printer, sizes: Option<&mut Sizes>) -> io::Result<()> {
    match sizes {
        None => printer.out().flush_out(),
        Some(sizes) => {
            let bytes = printer.out().take();
            sizes.deliver(&bytes)
        }
    }
}

/// An error line: `jag: ` and the text.
fn say(text: &str) {
    let _ = io::stderr().write_all(line(text).as_bytes());
}

/// A line that has its own `jag: `.
fn say_plain(text: &str) {
    let _ = writeln!(io::stderr(), "{text}");
}

/// The size limit of a search with no question (decision 19): whole results
/// in the order of the search, while they fit in the limit.
struct Sizes {
    max: usize,
    used: usize,
    report: SizeReport,
    /// A result did not fit: nothing after it goes out.
    full: bool,
    /// The results are groups of lines, with `--` before each group.
    grouped: bool,
    /// The first result of a haystack has the heading line.
    heading: bool,
}

impl Sizes {
    fn new(max: usize, grouped: bool, heading: bool) -> Sizes {
        Sizes {
            max,
            used: 0,
            report: SizeReport::default(),
            full: false,
            grouped,
            heading,
        }
    }

    fn report(&self) -> SizeReport {
        self.report
    }

    /// Write the results of `bytes` that fit. The others only count.
    fn deliver(&mut self, bytes: &[u8]) -> io::Result<()> {
        let mut stdout = io::stdout().lock();
        for result in split_results(bytes, self.grouped, self.heading) {
            if self.full {
                self.report.left_out += 1;
                continue;
            }
            let size = char_count(result);
            if self.used + size <= self.max {
                stdout.write_all(result)?;
                self.used += size;
                self.report.shown += 1;
                continue;
            }
            self.full = true;
            if self.report.shown == 0 {
                let (cut, left) = cut_result(result, self.max);
                stdout.write_all(&cut)?;
                self.report.shown = 1;
                self.report.top_result_cut = left;
            } else {
                self.report.left_out += 1;
            }
        }
        stdout.flush()
    }
}

/// The part of `result` that fits in `max` characters, with the mark
/// ` [+K chars]`, and K, the count of characters that jag cut (DECISION-23).
fn cut_result(result: &[u8], max: usize) -> (Vec<u8>, u64) {
    let text = result.strip_suffix(b"\n").unwrap_or(result);
    let total = char_count(text);
    let mark = |left: usize| format!(" [+{left} chars]");
    let kept = (0..=total.min(max))
        .rev()
        .find(|kept| kept + mark(total - kept).chars().count() < max)
        .unwrap_or(0);
    let mut cut = char_prefix(text, kept).to_vec();
    cut.extend_from_slice(mark(total - kept).as_bytes());
    cut.push(b'\n');
    (cut, (total - kept) as u64)
}

/// A line without the color codes of the printer.
fn without_codes(line: &[u8]) -> Vec<u8> {
    let mut plain = Vec::with_capacity(line.len());
    let mut at = 0;
    while at < line.len() {
        if line[at] == 0x1b && line.get(at + 1) == Some(&b'[') {
            at += 2;
            while at < line.len() && !line[at].is_ascii_alphabetic() {
                at += 1;
            }
            at += 1;
        } else {
            plain.push(line[at]);
            at += 1;
        }
    }
    plain
}

/// The results in the output of one haystack: a line, or with `grouped` a
/// group of lines that starts at a `--` line. With `heading`, the heading line
/// and the empty line before it join the first line.
fn split_results(bytes: &[u8], grouped: bool, heading: bool) -> Vec<&[u8]> {
    let lines: Vec<&[u8]> = bytes.split_inclusive(|byte| *byte == b'\n').collect();
    let mut results = Vec::new();
    let mut start = 0;
    let mut offset = 0;
    let mut pending = 0;
    if heading && !grouped {
        // The first result: an empty line, the heading, and a line.
        let take = if lines.first() == Some(&&b"\n"[..]) {
            3
        } else {
            2
        };
        let take = take.min(lines.len());
        let size: usize = lines[..take].iter().map(|line| line.len()).sum();
        if size > 0 {
            results.push(&bytes[..size]);
        }
        start = size;
        offset = size;
        pending = take;
    }
    for text in &lines[pending..] {
        if grouped && without_codes(text) == b"--\n" && offset > start {
            results.push(&bytes[start..offset]);
            start = offset;
        }
        if !grouped {
            results.push(&bytes[offset..offset + text.len()]);
            start = offset + text.len();
        }
        offset += text.len();
    }
    if grouped && offset > start {
        results.push(&bytes[start..offset]);
    }
    results
}
