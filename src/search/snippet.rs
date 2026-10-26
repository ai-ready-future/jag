//! Results with the lines around a match (STORY-14, CRITERION-21).
//!
//! `plan_windows` joins the matches of each file into windows. It reads no
//! file. `read_snippets` reads each file of the kept windows one time and
//! gives the lines of each window.

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub use super::Source;
use super::walk::{FileMatches, WalkWarning, WarningKind};

/// The lines before and after a match.
pub const CONTEXT_LINES: u64 = 3;
/// The most lines in one result.
pub const MAX_LINES: u64 = 12;

/// The most characters of a line in a request and in the output (decision 21).
pub const MAX_LINE_CHARS: usize = 300;

/// One line of a result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnippetLine {
    /// The number of the line. The first line is 1.
    pub number: u64,
    /// The bytes of the line that jag keeps, with no `\n`. A `\r` at the end
    /// stays. A line of more than `MAX_LINE_CHARS` characters is cut.
    pub text: Vec<u8>,
    pub is_match: bool,
    /// The count of characters cut from the end. 0 for no cut.
    pub cut: u64,
}

impl SnippetLine {
    /// A line from its full bytes. The line is cut at `MAX_LINE_CHARS`.
    pub fn new(number: u64, line: &[u8], is_match: bool) -> SnippetLine {
        let (kept, cut) = cut_line(line);
        SnippetLine {
            number,
            text: kept.to_vec(),
            is_match,
            cut,
        }
    }
}

/// Keep the first `MAX_LINE_CHARS` characters of a line. Gives the kept bytes
/// and the count of the other characters. Each bad sequence of bytes counts as
/// one character, as in the request (`from_utf8_lossy` gives one U+FFFD for
/// it).
pub fn cut_line(bytes: &[u8]) -> (&[u8], u64) {
    let mut count = 0usize;
    let mut offset = 0usize;
    let mut kept = 0usize;
    for width in char_widths(bytes) {
        offset += width;
        count += 1;
        if count == MAX_LINE_CHARS {
            kept = offset;
        }
    }
    if count <= MAX_LINE_CHARS {
        (bytes, 0)
    } else {
        (&bytes[..kept], (count - MAX_LINE_CHARS) as u64)
    }
}

/// The width in bytes of each character of `String::from_utf8_lossy(bytes)`.
/// Each bad sequence of bytes is one character. The cut and the count of
/// characters use this one rule.
fn char_widths(bytes: &[u8]) -> impl Iterator<Item = usize> + '_ {
    bytes.utf8_chunks().flat_map(|chunk| {
        let bad = chunk.invalid().len();
        chunk
            .valid()
            .chars()
            .map(char::len_utf8)
            .chain((bad > 0).then_some(bad))
    })
}

/// The first `chars` characters of `bytes`: a prefix that ends at a character,
/// never inside one.
pub fn char_prefix(bytes: &[u8], chars: usize) -> &[u8] {
    let end: usize = char_widths(bytes).take(chars).sum();
    &bytes[..end]
}

/// The count of characters of `String::from_utf8_lossy(bytes)`, with no copy
/// (the unit of the size limit, decision 2 of FEATURE-09).
pub fn char_count(bytes: &[u8]) -> usize {
    char_widths(bytes).count()
}

/// One result: a match with the lines around it (IF-6).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snippet {
    pub source: Source,
    pub lines: Vec<SnippetLine>,
}

impl Snippet {
    /// The number of the first line. 0 when the result has no line.
    pub fn first_line(&self) -> u64 {
        self.lines.first().map_or(0, |line| line.number)
    }

    /// The number of the last line. 0 when the result has no line.
    pub fn last_line(&self) -> u64 {
        self.lines.last().map_or(0, |line| line.number)
    }
}

/// The lines around each match (FEATURE-13, decisions 13 to 17).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContextLines {
    pub before: u64,
    pub after: u64,
    /// The most lines of one result. `None`: no limit (plain grep).
    pub max_lines: Option<u64>,
}

impl ContextLines {
    /// 3 lines before, 3 lines after, and at most 12 lines (FEATURE-04 spec
    /// decision 4).
    pub const DEFAULT: ContextLines = ContextLines {
        before: CONTEXT_LINES,
        after: CONTEXT_LINES,
        max_lines: Some(MAX_LINES),
    };
}

/// The plan of one result. The plan uses only line numbers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Window {
    pub source: Source,
    pub start: u64,
    pub end: u64,
    /// The numbers of the matching lines in the window, in line order.
    pub matches: Vec<u64>,
}

/// Join the matches of each file into windows of `context.max_lines` lines
/// or less, with `context.before` and `context.after` lines around each
/// match. With no limit, windows that touch or overlap always join.
///
/// The windows are in the order of `files`, then in line order.
pub fn plan_windows(files: &[FileMatches], context: ContextLines) -> Vec<Window> {
    // The last line that a window from `start` can hold.
    let last_line = |start: u64| {
        context
            .max_lines
            .map_or(u64::MAX, |max| start.saturating_add(max.saturating_sub(1)))
    };
    let mut windows = Vec::new();
    for file in files {
        let mut current: Option<Window> = None;
        for line in &file.lines {
            let m = line.line_number;
            current = Some(match current.take() {
                None => open_window(&file.source, m, 1, context),
                Some(mut window) => {
                    if m <= last_line(window.start)
                        && m.saturating_sub(context.before) <= window.end.saturating_add(1)
                    {
                        window.matches.push(m);
                        window.end = window
                            .end
                            .max(m.saturating_add(context.after))
                            .min(last_line(window.start));
                        window
                    } else {
                        let first_free = window.end.saturating_add(1);
                        windows.push(window);
                        open_window(&file.source, m, first_free, context)
                    }
                }
            });
        }
        windows.extend(current);
    }
    windows
}

/// A new window for the match `m`. No line before `first_free` is in it.
fn open_window(source: &Source, m: u64, first_free: u64, context: ContextLines) -> Window {
    Window {
        source: source.clone(),
        start: m.saturating_sub(context.before).max(first_free),
        end: m.saturating_add(context.after),
        matches: vec![m],
    }
}

/// One window for each matching line, with no line around it (FEATURE-12,
/// decision 6). The windows are in the order of `files`, then in line order.
pub fn line_windows(files: &[FileMatches]) -> Vec<Window> {
    files
        .iter()
        .flat_map(|file| {
            file.lines.iter().map(|line| Window {
                source: file.source.clone(),
                start: line.line_number,
                end: line.line_number,
                matches: vec![line.line_number],
            })
        })
        .collect()
}

/// The text of the matching lines of each window, in the order of `windows`.
///
/// `windows` must be the output of `plan_windows(files, context)`. That function puts
/// each matching line of `files` in exactly one window, in order. So window
/// k holds the next `windows[k].matches.len()` matching lines. This reads no
/// file.
pub fn match_texts<'a>(files: &'a [FileMatches], windows: &[Window]) -> Vec<Vec<&'a [u8]>> {
    let mut lines = files.iter().flat_map(|file| file.lines.iter());
    windows
        .iter()
        .map(|window| {
            window
                .matches
                .iter()
                .map(|number| {
                    let line = lines.next().expect("a matching line for each window line");
                    assert_eq!(
                        line.line_number, *number,
                        "windows must come from plan_windows"
                    );
                    line.text.as_slice()
                })
                .collect()
        })
        .collect()
}

/// The most bytes that jag reads from a file at one time.
const READ_PART: usize = 64 * 1024;

/// The most bytes of `MAX_LINE_CHARS` characters: a character has at most 4
/// bytes, and a bad sequence has at most 3 bytes.
const MAX_KEPT_BYTES: usize = 4 * MAX_LINE_CHARS;

/// Read each file of the windows one time and give the lines of each window.
/// A window of standard input takes its lines from `input`, the piped text
/// that the scan searched (decision 13 of FEATURE-14).
///
/// A file that jag cannot read gives a warning and no result. A path that is
/// not a regular file (for example a device such as `/dev/zero`) gives the
/// same warning, and jag reads no byte of it. The file can change after the
/// scan, so the end of a window stops at the last line, and a match past the
/// last line is dropped.
///
/// jag reads all of each file in parts, but it keeps only the lines inside a
/// window, and of each line only the bytes that the result shows.
pub fn read_snippets(windows: &[Window], input: Option<&[u8]>) -> (Vec<Snippet>, Vec<WalkWarning>) {
    let mut snippets = Vec::new();
    let mut warnings = Vec::new();
    let mut buffer = vec![0; READ_PART];
    let mut index = 0;
    while index < windows.len() {
        let source = &windows[index].source;
        let group_end = windows[index..]
            .iter()
            .position(|window| &window.source != source)
            .map_or(windows.len(), |offset| index + offset);
        let group = &windows[index..group_end];
        let lines = match source {
            Source::File(path) => {
                read_file(path, group, &mut buffer).map_err(|error| WalkWarning {
                    path: Some(path.clone()),
                    kind: WarningKind::Read(error.kind()),
                })
            }
            Source::StandardInput => {
                let mut lines = LineSplitter::new(group);
                for part in input.unwrap_or_default().chunks(READ_PART) {
                    lines.push(part);
                }
                Ok(lines.finish())
            }
        };
        match lines {
            Err(warning) => warnings.push(warning),
            Ok(lines) => {
                for window in group {
                    snippets.extend(snippet_of(window, &lines));
                }
            }
        }
        index = group_end;
    }
    (snippets, warnings)
}

/// Read the lines of `windows` from the file at `path`. jag reads no byte of
/// a path that is not a regular file, because a device or a pipe can have no
/// end. A folder gives the error that a read of a folder gives.
fn read_file(path: &Path, windows: &[Window], buffer: &mut [u8]) -> io::Result<KeptLines> {
    let file = File::open(path)?;
    let file_type = file.metadata()?.file_type();
    if file_type.is_dir() {
        return Err(io::ErrorKind::IsADirectory.into());
    }
    if !file_type.is_file() {
        return Err(io::ErrorKind::InvalidInput.into());
    }
    read_lines(file, windows, buffer)
}

/// Read all of `reader` in parts of the size of `buffer`, and keep the lines
/// of `windows`. jag reads to the end also after the last line of a window,
/// so that a read error at any point gives the error, as a read of all of the
/// file does.
fn read_lines(
    mut reader: impl Read,
    windows: &[Window],
    buffer: &mut [u8],
) -> io::Result<KeptLines> {
    let mut lines = LineSplitter::new(windows);
    loop {
        match reader.read(buffer) {
            Ok(0) => return Ok(lines.finish()),
            Ok(count) => lines.push(&buffer[..count]),
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(error) => return Err(error),
        }
    }
}

/// One line inside a window: its number, the bytes that jag keeps, and the
/// count of characters cut from the end (as `cut_line` gives them).
struct KeptLine {
    number: u64,
    text: Vec<u8>,
    cut: u64,
}

/// The lines of one file that its windows hold, and the count of lines of
/// the file.
struct KeptLines {
    /// In line order.
    lines: Vec<KeptLine>,
    count: u64,
}

impl KeptLines {
    fn line(&self, number: u64) -> &KeptLine {
        let index = self
            .lines
            .binary_search_by_key(&number, |line| line.number)
            .expect("each line of a window is kept");
        &self.lines[index]
    }
}

/// Splits the text of one file into lines, part after part, and keeps only
/// the lines inside the windows. The lines and the line numbers are the same
/// as in the scan: a byte order mark of UTF-16 means the text is changed to
/// UTF-8, a mark of UTF-8 is removed, and a text that ends with `\n` has no
/// empty last line.
struct LineSplitter {
    /// The `(start, end)` of each window, in order of `start`.
    spans: Vec<(u64, u64)>,
    /// The first span that does not end before the current line.
    span: usize,
    /// The first bytes of the text, until there are 3 of them: they tell the
    /// byte order mark. `None` after that.
    head: Option<Vec<u8>>,
    /// After a mark of UTF-16: the state of the change to UTF-8.
    utf16: Option<Utf16>,
    /// The count of `\n` in the text so far.
    newlines: u64,
    /// The current line has at least one byte.
    open: bool,
    /// The bytes of the current line, when a window holds it.
    line: Option<LineText>,
    kept: Vec<KeptLine>,
}

impl LineSplitter {
    fn new(windows: &[Window]) -> LineSplitter {
        let mut spans: Vec<(u64, u64)> = windows
            .iter()
            .map(|window| (window.start, window.end))
            .collect();
        spans.sort_unstable();
        LineSplitter {
            spans,
            span: 0,
            head: Some(Vec::new()),
            utf16: None,
            newlines: 0,
            open: false,
            line: None,
            kept: Vec::new(),
        }
    }

    /// Add the next bytes of the file.
    fn push(&mut self, mut part: &[u8]) {
        if let Some(head) = self.head.as_mut() {
            let count = (3 - head.len()).min(part.len());
            head.extend_from_slice(&part[..count]);
            part = &part[count..];
            if head.len() < 3 {
                return;
            }
        }
        if let Some(head) = self.head.take() {
            self.start(&head);
        }
        self.push_body(part);
    }

    /// The kept lines, after the last byte of the file.
    fn finish(mut self) -> KeptLines {
        if let Some(head) = self.head.take() {
            self.start(&head);
        }
        if let Some(text) = self.utf16.as_mut().map(Utf16::finish) {
            self.push_text(text.as_bytes());
        }
        let mut count = self.newlines;
        if self.open {
            count += 1;
            self.end_line(count);
        }
        KeptLines {
            lines: self.kept,
            count,
        }
    }

    /// Use the byte order mark at the start of `head`, then add the bytes
    /// after it.
    fn start(&mut self, head: &[u8]) {
        let little = head.starts_with(&[0xFF, 0xFE]);
        if little || head.starts_with(&[0xFE, 0xFF]) {
            self.utf16 = Some(Utf16 {
                little,
                odd: None,
                high: None,
            });
            self.push_body(&head[2..]);
        } else {
            self.push_text(head.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(head));
        }
    }

    /// Add bytes that come after the byte order mark.
    fn push_body(&mut self, bytes: &[u8]) {
        match self.utf16.as_mut() {
            Some(utf16) => {
                let text = utf16.push(bytes);
                self.push_text(text.as_bytes());
            }
            None => self.push_text(bytes),
        }
    }

    /// Add text that has no byte order mark, and split it on `\n`.
    fn push_text(&mut self, mut text: &[u8]) {
        while !text.is_empty() {
            let number = self.newlines + 1;
            let kept = self.holds(number);
            if !kept && self.span == self.spans.len() {
                // No window holds a later line: count the lines only.
                self.newlines += text.iter().filter(|byte| **byte == b'\n').count() as u64;
                self.open = !text.ends_with(b"\n");
                return;
            }
            let (body, ended) = match text.iter().position(|byte| *byte == b'\n') {
                Some(end) => {
                    let body = &text[..end];
                    text = &text[end + 1..];
                    (body, true)
                }
                None => (std::mem::take(&mut text), false),
            };
            if kept {
                self.line.get_or_insert_default().push(body);
            }
            if ended {
                self.end_line(number);
                self.newlines += 1;
            }
            self.open = !ended;
        }
    }

    fn end_line(&mut self, number: u64) {
        if let Some(line) = self.line.take() {
            let (text, cut) = line.finish();
            self.kept.push(KeptLine { number, text, cut });
        }
    }

    /// A window holds line `number`. Each call must give a number that is not
    /// less than the number of the call before it.
    fn holds(&mut self, number: u64) -> bool {
        while self
            .spans
            .get(self.span)
            .is_some_and(|span| span.1 < number)
        {
            self.span += 1;
        }
        self.spans
            .get(self.span)
            .is_some_and(|span| span.0 <= number)
    }
}

/// The state of the change of UTF-16 to UTF-8 between two parts. The text is
/// the same as `String::from_utf16_lossy` gives for all of the units, with a
/// last unit of U+FFFD for an odd last byte.
struct Utf16 {
    little: bool,
    /// The first byte of a unit whose second byte is in the next part.
    odd: Option<u8>,
    /// A high surrogate at the end of the units so far: the next unit can
    /// join it.
    high: Option<u16>,
}

impl Utf16 {
    /// The UTF-8 text of the next bytes.
    fn push(&mut self, bytes: &[u8]) -> String {
        let mut joined = Vec::with_capacity(bytes.len() + 1);
        joined.extend(self.odd.take());
        joined.extend_from_slice(bytes);
        let (pairs, rest) = joined.as_chunks::<2>();
        self.odd = rest.first().copied();
        let little = self.little;
        let mut units: Vec<u16> = self
            .high
            .take()
            .into_iter()
            .chain(pairs.iter().map(|pair| {
                if little {
                    u16::from_le_bytes(*pair)
                } else {
                    u16::from_be_bytes(*pair)
                }
            }))
            .collect();
        if units
            .last()
            .is_some_and(|unit| (0xD800..0xDC00).contains(unit))
        {
            self.high = units.pop();
        }
        String::from_utf16_lossy(&units)
    }

    /// The UTF-8 text of the units that are left after the last byte.
    fn finish(&mut self) -> String {
        let mut units: Vec<u16> = self.high.take().into_iter().collect();
        if self.odd.take().is_some() {
            units.push(0xFFFD);
        }
        String::from_utf16_lossy(&units)
    }
}

/// The bytes of one kept line, added part after part. While the line has at
/// most `MAX_KEPT_BYTES` bytes, `text` holds all of it. After that, `text`
/// holds only the first `MAX_LINE_CHARS` characters, and `rest` counts the
/// other characters. So a long line uses little memory, and the kept bytes
/// and the cut are the same as `cut_line` gives for all of the line.
#[derive(Default)]
struct LineText {
    text: Vec<u8>,
    rest: Option<CharCount>,
}

impl LineText {
    fn push(&mut self, mut bytes: &[u8]) {
        if self.rest.is_none() {
            let count = (MAX_KEPT_BYTES + 1 - self.text.len()).min(bytes.len());
            self.text.extend_from_slice(&bytes[..count]);
            bytes = &bytes[count..];
            if self.text.len() <= MAX_KEPT_BYTES {
                return;
            }
            // More than `MAX_KEPT_BYTES` bytes hold more than `MAX_LINE_CHARS`
            // characters. Each of the first `MAX_LINE_CHARS` characters
            // starts at least 4 bytes before the end of `text`, so a later
            // byte cannot change it.
            let kept = char_prefix(&self.text, MAX_LINE_CHARS).len();
            let mut rest = CharCount::default();
            rest.push(&self.text[kept..]);
            self.text.truncate(kept);
            self.rest = Some(rest);
        }
        if let Some(rest) = self.rest.as_mut() {
            rest.push(bytes);
        }
    }

    /// The kept bytes and the count of characters cut from the end.
    fn finish(self) -> (Vec<u8>, u64) {
        match self.rest {
            None => {
                let (kept, cut) = cut_line(&self.text);
                (kept.to_vec(), cut)
            }
            Some(rest) => (self.text, rest.finish()),
        }
    }
}

/// Counts the characters of bytes that come in parts, with the rule of
/// `char_widths`: each bad sequence of bytes is one character.
#[derive(Default)]
struct CharCount {
    count: u64,
    /// The last bytes, from the start of a character that the next bytes can
    /// change. Fewer than 4 bytes after each `push`.
    pending: Vec<u8>,
}

impl CharCount {
    fn push(&mut self, bytes: &[u8]) {
        self.pending.extend_from_slice(bytes);
        let mut start = 0;
        for width in char_widths(&self.pending) {
            // Only the first 4 bytes from its start tell a character. When
            // they are all here, a later byte cannot change it.
            if start + 4 > self.pending.len() {
                break;
            }
            start += width;
            self.count += 1;
        }
        self.pending.drain(..start);
    }

    /// The count of characters, after the last byte.
    fn finish(self) -> u64 {
        self.count + char_count(&self.pending) as u64
    }
}

fn snippet_of(window: &Window, lines: &KeptLines) -> Option<Snippet> {
    let last = lines.count;
    let matches: Vec<u64> = window
        .matches
        .iter()
        .copied()
        .filter(|m| *m <= last)
        .collect();
    if matches.is_empty() {
        return None;
    }
    let end = window.end.min(last);
    let lines = (window.start..=end)
        .map(|number| {
            let line = lines.line(number);
            SnippetLine {
                number,
                text: line.text.clone(),
                is_match: matches.binary_search(&number).is_ok(),
                cut: line.cut,
            }
        })
        .collect();
    Some(Snippet {
        source: window.source.clone(),
        lines,
    })
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use std::fs;
    use std::io;
    use std::path::{Path, PathBuf};

    use tempfile::TempDir;

    use super::*;
    use crate::search::walk::MatchLine;

    fn file(path: &str, numbers: &[u64]) -> FileMatches {
        FileMatches {
            source: Source::File(PathBuf::from(path)),
            lines: numbers
                .iter()
                .map(|number| MatchLine {
                    line_number: *number,
                    text: b"x".to_vec(),
                })
                .collect(),
        }
    }

    #[test]
    fn line_windows_gives_one_window_for_each_matching_line() {
        let windows = line_windows(&[file("a", &[1, 3]), file("b", &[2])]);
        let got: Vec<_> = windows
            .iter()
            .map(|window| {
                (
                    window.source.path().and_then(Path::to_str).expect("text"),
                    window.start,
                    window.end,
                    window.matches.clone(),
                )
            })
            .collect();
        assert_eq!(
            got,
            [
                ("a", 1, 1, vec![1]),
                ("a", 3, 3, vec![3]),
                ("b", 2, 2, vec![2])
            ]
        );
    }

    #[test]
    fn line_windows_read_as_one_line_snippets_that_keep_a_carriage_return() {
        let dir = tempfile::tempdir().expect("folder");
        fs::write(dir.path().join("a.txt"), b"one\r\ntwo\nthree\n").expect("write");
        let path = dir.path().join("a.txt");
        let files = [FileMatches {
            source: Source::File(path),
            lines: vec![
                MatchLine {
                    line_number: 1,
                    text: b"one".to_vec(),
                },
                MatchLine {
                    line_number: 3,
                    text: b"three".to_vec(),
                },
            ],
        }];
        let (snippets, warnings) = read_snippets(&line_windows(&files), None);
        assert!(warnings.is_empty());
        assert_eq!(snippets.len(), 2);
        assert_eq!(snippets[0].lines.len(), 1);
        assert_eq!(snippets[0].lines[0].text, b"one\r");
        assert!(snippets[0].lines[0].is_match);
        assert_eq!(snippets[1].lines[0].number, 3);
    }

    /// The `(start, end)` of each window for one file.
    fn spans(numbers: &[u64]) -> Vec<(u64, u64)> {
        plan_windows(&[file("a", numbers)], ContextLines::DEFAULT)
            .iter()
            .map(|window| (window.start, window.end))
            .collect()
    }

    /// The `(start, end)` of each window for one file, with `context`.
    fn spans_with(
        numbers: &[u64],
        before: u64,
        after: u64,
        max_lines: Option<u64>,
    ) -> Vec<(u64, u64)> {
        let context = ContextLines {
            before,
            after,
            max_lines,
        };
        plan_windows(&[file("a", numbers)], context)
            .iter()
            .map(|window| (window.start, window.end))
            .collect()
    }

    #[test]
    fn only_lines_after_give_no_line_before() {
        assert_eq!(spans_with(&[5], 0, 2, Some(12)), [(5, 7)]);
    }

    #[test]
    fn twelve_lines_on_each_side_give_a_window_of_25_lines() {
        assert_eq!(spans_with(&[30], 12, 12, Some(25)), [(18, 42)]);
    }

    #[test]
    fn with_no_limit_groups_that_touch_join_and_others_do_not() {
        assert_eq!(spans_with(&[4, 6], 1, 1, None), [(3, 7)]);
        assert_eq!(spans_with(&[4, 10], 1, 1, None), [(3, 5), (9, 11)]);
        // Lines 3 to 5 and 6 to 8 touch, so they join. Line 6 is a gap
        // between 3 to 5 and 7 to 9.
        assert_eq!(spans_with(&[4, 7], 1, 1, None), [(3, 8)]);
        assert_eq!(spans_with(&[4, 8], 1, 1, None), [(3, 5), (7, 9)]);
        assert_eq!(spans_with(&[4, 5], 0, 0, None), [(4, 5)]);
        assert_eq!(spans_with(&[4, 10], 0, 0, None), [(4, 4), (10, 10)]);
        let many: Vec<u64> = (1..=100).step_by(2).collect();
        assert_eq!(spans_with(&many, 1, 1, None), [(1, 100)]);
    }

    #[test]
    fn a_very_large_value_does_not_overflow() {
        assert_eq!(spans_with(&[5, 9], 0, u64::MAX, None), [(5, u64::MAX)]);
        assert_eq!(
            spans_with(&[5], u64::MAX, u64::MAX, Some(u64::MAX)),
            [(1, u64::MAX)]
        );
        assert_eq!(
            spans_with(&[u64::MAX], 3, 3, Some(12)),
            [(u64::MAX - 3, u64::MAX)]
        );
    }

    #[test]
    fn the_default_context_gives_the_windows_of_feature_04() {
        assert_eq!(spans_with(&[10, 16], 3, 3, Some(12)), spans(&[10, 16]));
        assert_eq!(ContextLines::DEFAULT.max_lines, Some(MAX_LINES));
    }

    fn cut(bytes: &[u8]) -> (usize, u64) {
        let (kept, cut) = cut_line(bytes);
        (kept.len(), cut)
    }

    #[test]
    fn a_line_of_300_characters_is_not_cut() {
        assert_eq!(cut(&[b'a'; 300]), (300, 0));
        assert_eq!(cut(b""), (0, 0));
    }

    #[test]
    fn a_line_of_more_than_300_characters_is_cut() {
        assert_eq!(cut(&[b'a'; 301]), (300, 1));
        assert_eq!(cut(&[b'a'; 1000]), (300, 700));
    }

    #[test]
    fn a_multibyte_character_counts_as_one_character() {
        let line = "漢".repeat(400);
        assert_eq!(cut(line.as_bytes()), (900, 100));
    }

    #[test]
    fn a_byte_that_is_not_utf8_counts_as_one_character() {
        assert_eq!(cut(&[0xFF; 400]), (300, 100));
    }

    #[test]
    fn a_bad_sequence_counts_as_one_character() {
        let mut line = vec![b'a'; 299];
        line.extend([0xE2, 0x82]);
        line.extend([b'b'; 10]);
        assert_eq!(cut(&line), (301, 10));
    }

    #[test]
    fn a_carriage_return_counts_as_one_character() {
        let mut line = vec![b'a'; 300];
        line.push(b'\r');
        assert_eq!(cut(&line), (300, 1));
    }

    #[test]
    fn the_kept_bytes_of_valid_text_are_valid_text() {
        for line in ["é".repeat(500), "漢".repeat(500), "😀".repeat(500)] {
            let (kept, cut) = cut_line(line.as_bytes());
            assert!(std::str::from_utf8(kept).is_ok());
            assert_eq!(
                std::str::from_utf8(kept).expect("text").chars().count(),
                300
            );
            assert_eq!(cut, 200);
        }
    }

    #[test]
    fn a_new_line_is_cut_at_300_characters() {
        let line = SnippetLine::new(4, &[b'x'; 1000], true);
        assert_eq!((line.number, line.text.len(), line.cut), (4, 300, 700));
        assert!(line.is_match);
    }

    #[test]
    fn a_file_line_of_1000_characters_is_read_as_300_and_a_cut_of_700() {
        let (_dir, path) = folder_with("a.txt", format!("{}\n", "x".repeat(1000)).as_bytes());
        let (snippets, _) = read_snippets(&[window(&path, 1, 1, &[1])], None);
        assert_eq!(snippets[0].lines[0].text.len(), 300);
        assert_eq!(snippets[0].lines[0].cut, 700);
    }

    #[test]
    fn a_result_gives_its_first_and_last_line() {
        let snippet = Snippet {
            source: Source::StandardInput,
            lines: (7..=17)
                .map(|n| SnippetLine::new(n, b"x", n == 10))
                .collect(),
        };
        assert_eq!((snippet.first_line(), snippet.last_line()), (7, 17));
        let empty = Snippet {
            source: Source::StandardInput,
            lines: Vec::new(),
        };
        assert_eq!((empty.first_line(), empty.last_line()), (0, 0));
    }

    #[test]
    fn the_label_of_standard_input_is_fixed() {
        assert_eq!(Source::StandardInput.label(), b"<stdin>");
        assert_eq!(Source::File(PathBuf::from("a/b")).label(), b"a/b");
    }

    #[test]
    fn near_matches_join_into_one_window() {
        assert_eq!(spans(&[10, 14]), [(7, 17)]);
    }

    #[test]
    fn a_window_has_at_most_12_lines() {
        assert_eq!(spans(&[10, 16]), [(7, 18)]);
        assert_eq!(spans(&[10, 13, 16]), [(7, 18)]);
    }

    #[test]
    fn a_new_window_does_not_repeat_a_line() {
        assert_eq!(spans(&[1, 4, 7, 9, 11, 14]), [(1, 12), (13, 17)]);
    }

    #[test]
    fn far_matches_give_two_windows() {
        assert_eq!(spans(&[10, 30]), [(7, 13), (27, 33)]);
    }

    #[test]
    fn a_match_on_line_2_starts_at_line_1() {
        assert_eq!(spans(&[2]), [(1, 5)]);
    }

    #[test]
    fn the_matches_of_a_window_are_kept_in_line_order() {
        let windows = plan_windows(&[file("a", &[10, 13, 16, 40])], ContextLines::DEFAULT);
        assert_eq!(windows[0].matches, [10, 13, 16]);
        assert_eq!(windows[1].matches, [40]);
    }

    #[test]
    fn windows_follow_the_order_of_the_files() {
        let windows = plan_windows(&[file("a", &[1]), file("b", &[1])], ContextLines::DEFAULT);
        assert_eq!(windows[0].source, Source::File(PathBuf::from("a")));
        assert_eq!(windows[1].source, Source::File(PathBuf::from("b")));
    }

    #[test]
    fn match_texts_gives_the_matching_lines_of_each_window() {
        let mut first = file("a", &[10, 14, 30]);
        first.lines[0].text = b"ten".to_vec();
        first.lines[1].text = b"fourteen".to_vec();
        first.lines[2].text = b"thirty".to_vec();
        let mut second = file("b", &[2]);
        second.lines[0].text = b"two".to_vec();
        let files = [first, second];
        let windows = plan_windows(&files, ContextLines::DEFAULT);
        assert_eq!(windows.len(), 3);
        let texts = match_texts(&files, &windows);
        assert_eq!(
            texts,
            [
                vec![&b"ten"[..], &b"fourteen"[..]],
                vec![&b"thirty"[..]],
                vec![&b"two"[..]]
            ]
        );
    }

    #[test]
    #[should_panic(expected = "windows must come from plan_windows")]
    fn match_texts_refuses_windows_that_do_not_match_the_files() {
        let files = [file("a", &[10])];
        let windows = [Window {
            source: Source::File(PathBuf::from("a")),
            start: 1,
            end: 5,
            matches: vec![3],
        }];
        let _ = match_texts(&files, &windows);
    }

    fn folder_with(name: &str, text: &[u8]) -> (TempDir, PathBuf) {
        let dir = TempDir::new().expect("temporary folder");
        let path = dir.path().join(name);
        fs::write(&path, text).expect("write file");
        (dir, path)
    }

    fn window(path: &Path, start: u64, end: u64, matches: &[u64]) -> Window {
        Window {
            source: Source::File(path.to_path_buf()),
            start,
            end,
            matches: matches.to_vec(),
        }
    }

    fn numbers(snippet: &Snippet) -> Vec<u64> {
        snippet.lines.iter().map(|line| line.number).collect()
    }

    #[test]
    fn a_match_on_the_last_line_ends_at_the_last_line() {
        let (_dir, path) = folder_with("a.txt", b"1\n2\n3\n4\n5\n");
        let (snippets, warnings) = read_snippets(&[window(&path, 2, 8, &[5])], None);
        assert!(warnings.is_empty());
        assert_eq!(numbers(&snippets[0]), [2, 3, 4, 5]);
        let last = snippets[0].lines.last().expect("a last line");
        assert!(last.is_match);
        assert_eq!(last.text, b"5");
    }

    #[test]
    fn a_file_with_no_final_newline_gives_the_same_lines() {
        let (_dir, path) = folder_with("a.txt", b"1\n2\n3\n4\n5");
        let (snippets, _) = read_snippets(&[window(&path, 2, 8, &[5])], None);
        assert_eq!(numbers(&snippets[0]), [2, 3, 4, 5]);
    }

    #[test]
    fn a_match_past_the_last_line_is_dropped() {
        let (_dir, path) = folder_with("a.txt", b"1\n2\n3\n");
        let (snippets, _) = read_snippets(
            &[window(&path, 1, 6, &[2, 9]), window(&path, 20, 26, &[23])],
            None,
        );
        assert_eq!(snippets.len(), 1);
        let marks: Vec<bool> = snippets[0].lines.iter().map(|l| l.is_match).collect();
        assert_eq!(marks, [false, true, false]);
    }

    fn utf16(text: &str, little: bool) -> Vec<u8> {
        let mut bytes = if little {
            vec![0xFF, 0xFE]
        } else {
            vec![0xFE, 0xFF]
        };
        for unit in text.encode_utf16() {
            let pair = if little {
                unit.to_le_bytes()
            } else {
                unit.to_be_bytes()
            };
            bytes.extend(pair);
        }
        bytes
    }

    #[test]
    fn a_utf16_file_with_a_mark_gives_the_lines_as_text() {
        for little in [true, false] {
            let (_dir, path) = folder_with("a.txt", &utf16("one\ntwo é\nthree\n", little));
            let (snippets, warnings) = read_snippets(&[window(&path, 1, 3, &[2])], None);
            assert!(warnings.is_empty());
            let texts: Vec<&[u8]> = snippets[0]
                .lines
                .iter()
                .map(|line| line.text.as_slice())
                .collect();
            assert_eq!(texts, [&b"one"[..], "two é".as_bytes(), &b"three"[..]]);
            assert!(snippets[0].lines[1].is_match);
        }
    }

    #[test]
    fn a_utf8_mark_is_not_part_of_the_first_line() {
        let (_dir, path) = folder_with("a.txt", b"\xEF\xBB\xBFone\ntwo\n");
        let (snippets, _) = read_snippets(&[window(&path, 1, 2, &[1])], None);
        assert_eq!(snippets[0].lines[0].text, b"one");
    }

    #[test]
    fn a_carriage_return_stays_at_the_end_of_a_line() {
        let (_dir, path) = folder_with("a.txt", b"one\r\ntwo\r\n");
        let (snippets, _) = read_snippets(&[window(&path, 1, 2, &[1])], None);
        assert_eq!(snippets[0].lines[0].text, b"one\r");
    }

    #[test]
    fn a_file_that_jag_cannot_read_gives_a_warning_and_no_result() {
        let dir = TempDir::new().expect("temporary folder");
        let missing = dir.path().join("missing.txt");
        let (snippets, warnings) = read_snippets(&[window(&missing, 1, 4, &[1])], None);
        assert!(snippets.is_empty());
        assert_eq!(
            warnings,
            [WalkWarning {
                path: Some(missing),
                kind: WarningKind::Read(io::ErrorKind::NotFound),
            }]
        );
    }

    #[test]
    fn jag_reads_only_the_files_of_the_kept_windows() {
        let dir = TempDir::new().expect("temporary folder");
        let files: Vec<FileMatches> = (0..250)
            .map(|number| {
                let name = format!("f{number:03}.txt");
                if number < 200 {
                    fs::write(dir.path().join(&name), b"needle\n").expect("write file");
                }
                file(&dir.path().join(name).to_string_lossy(), &[1])
            })
            .collect();
        let windows = plan_windows(&files, ContextLines::DEFAULT);
        assert_eq!(windows.len(), 250);
        let (snippets, warnings) = read_snippets(&windows[..200], None);
        assert_eq!(snippets.len(), 200);
        assert!(warnings.is_empty());
    }

    // CRITERION-35: the lines of piped text come from the text that the scan
    // searched, with the label of standard input.
    #[test]
    fn a_window_of_standard_input_reads_the_piped_text() {
        let text = b"a\nb\nc\nd\ne\nf\nneedle\n";
        let windows = [
            Window {
                source: Source::StandardInput,
                start: 4,
                end: 10,
                matches: vec![7],
            },
            Window {
                source: Source::StandardInput,
                start: 1,
                end: 1,
                matches: vec![1],
            },
        ];
        let (snippets, warnings) = read_snippets(&windows, Some(text));
        assert!(warnings.is_empty());
        assert_eq!(snippets.len(), 2);
        assert_eq!(snippets[0].source, Source::StandardInput);
        assert_eq!(numbers(&snippets[0]), [4, 5, 6, 7]);
        let marks: Vec<bool> = snippets[0].lines.iter().map(|l| l.is_match).collect();
        assert_eq!(marks, [false, false, false, true]);
        assert_eq!(snippets[0].lines[3].text, b"needle");
        assert_eq!(snippets[1].lines[0].text, b"a");
        assert_eq!(
            snippets[0].source.label(),
            Source::STANDARD_INPUT_LABEL.as_bytes()
        );
    }

    #[test]
    fn two_windows_of_one_file_read_the_file_one_time() {
        let text: String = (1..=40).map(|number| format!("line {number}\n")).collect();
        let (_dir, path) = folder_with("a.txt", text.as_bytes());
        let (snippets, _) = read_snippets(
            &[window(&path, 7, 13, &[10]), window(&path, 27, 33, &[30])],
            None,
        );
        assert_eq!(snippets.len(), 2);
        assert_eq!(numbers(&snippets[1]).first(), Some(&27));
    }

    #[test]
    fn char_count_counts_characters_and_a_bad_sequence_as_one() {
        assert_eq!(char_count(b""), 0);
        assert_eq!(char_count(b"abc"), 3);
        assert_eq!(char_count("é".as_bytes()), 1);
        assert_eq!(char_count(b"a\xffb"), 3);
        assert_eq!(char_count(b"\xe2\x82"), 1);
        assert_eq!(char_count(b"\xe2\x82\n"), 2);
    }

    #[test]
    fn the_count_of_joined_lines_is_the_sum_of_their_counts() {
        let lines: [&[u8]; 3] = [b"a\xe2\x82\n", b"\xffb\n", "é\n".as_bytes()];
        let joined: Vec<u8> = lines.concat();
        let sum: usize = lines.iter().map(|line| char_count(line)).sum();
        assert_eq!(char_count(&joined), sum);
        assert_eq!(sum, String::from_utf8_lossy(&joined).chars().count());
    }

    // The model for the reader in parts: the reader that read all of a file
    // at one time, decoded all of it, and split all of it.
    fn whole_decode(bytes: &[u8]) -> Cow<'_, [u8]> {
        let little = bytes.starts_with(&[0xFF, 0xFE]);
        if little || bytes.starts_with(&[0xFE, 0xFF]) {
            let body = &bytes[2..];
            let (pairs, _) = body.as_chunks::<2>();
            let mut units: Vec<u16> = pairs
                .iter()
                .map(|pair| {
                    if little {
                        u16::from_le_bytes(*pair)
                    } else {
                        u16::from_be_bytes(*pair)
                    }
                })
                .collect();
            if body.len() % 2 == 1 {
                units.push(0xFFFD);
            }
            return Cow::Owned(String::from_utf16_lossy(&units).into_bytes());
        }
        match bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]) {
            Some(rest) => Cow::Borrowed(rest),
            None => Cow::Borrowed(bytes),
        }
    }

    fn whole_split(bytes: &[u8]) -> Vec<&[u8]> {
        if bytes.is_empty() {
            return Vec::new();
        }
        let mut lines: Vec<&[u8]> = bytes.split(|byte| *byte == b'\n').collect();
        if bytes.ends_with(b"\n") {
            lines.pop();
        }
        lines
    }

    fn whole_count(bytes: &[u8]) -> u64 {
        whole_split(&whole_decode(bytes)).len() as u64
    }

    fn whole_snippets(windows: &[Window], bytes: &[u8]) -> Vec<Snippet> {
        let text = whole_decode(bytes);
        let lines = whole_split(&text);
        let last = lines.len() as u64;
        windows
            .iter()
            .filter_map(|window| {
                let matches: Vec<u64> = window
                    .matches
                    .iter()
                    .copied()
                    .filter(|m| *m <= last)
                    .collect();
                if matches.is_empty() {
                    return None;
                }
                let end = window.end.min(last);
                let lines = (window.start..=end)
                    .map(|number| {
                        SnippetLine::new(
                            number,
                            lines[(number - 1) as usize],
                            matches.binary_search(&number).is_ok(),
                        )
                    })
                    .collect();
                Some(Snippet {
                    source: window.source.clone(),
                    lines,
                })
            })
            .collect()
    }

    /// The results and the count of lines, from a read in parts of `part`
    /// bytes.
    fn snippets_in_parts(windows: &[Window], bytes: &[u8], part: usize) -> (Vec<Snippet>, u64) {
        let mut lines = LineSplitter::new(windows);
        for chunk in bytes.chunks(part) {
            lines.push(chunk);
        }
        let lines = lines.finish();
        let snippets = windows
            .iter()
            .filter_map(|window| snippet_of(window, &lines))
            .collect();
        (snippets, lines.count)
    }

    /// Windows in no order: one for all lines, one for each line (or for
    /// some lines of a long text), and windows past the last line.
    fn windows_for(source: &Source, count: u64) -> Vec<Window> {
        let make = |start: u64, end: u64, matches: Vec<u64>| Window {
            source: source.clone(),
            start,
            end,
            matches,
        };
        let mut windows = vec![make(1, count + 2, (1..=count + 1).collect())];
        let step = 1 + count as usize / 200;
        windows.extend(
            (1..=count + 1)
                .rev()
                .step_by(step)
                .map(|number| make(number, number, vec![number])),
        );
        windows.push(make(2, 4, vec![3]));
        windows.push(make(
            count.saturating_sub(1).max(1),
            count + 5,
            vec![count.max(1)],
        ));
        windows
    }

    struct Random(u64);

    impl Random {
        fn below(&mut self, limit: usize) -> usize {
            let mut x = self.0;
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            self.0 = x;
            (x % limit as u64) as usize
        }
    }

    fn utf16_units(units: &[u16], little: bool, odd: bool) -> Vec<u8> {
        let mut bytes = if little {
            vec![0xFF, 0xFE]
        } else {
            vec![0xFE, 0xFF]
        };
        for unit in units {
            let pair = if little {
                unit.to_le_bytes()
            } else {
                unit.to_be_bytes()
            };
            bytes.extend(pair);
        }
        if odd {
            bytes.push(0x41);
        }
        bytes
    }

    const PIECES: [&[u8]; 16] = [
        b"a",
        b"z",
        b" ",
        b"\n",
        b"\r",
        b"\xC3\xA9",
        b"\xE6\xBC\xA2",
        b"\xF0\x9F\x98\x80",
        b"\xE2\x82",
        b"\xF0\x9F\x98",
        b"\xFF",
        b"\x80",
        b"\xC3",
        b"\xED\xA0\x80",
        b"\xF4\x90\x80\x80",
        b"\xE0\x80",
    ];

    const UNITS: [u16; 12] = [
        0x0041, 0x000D, 0x00E9, 0x6F22, 0xD83D, 0xDE00, 0xD800, 0xDC00, 0xDBFF, 0xDFFF, 0xFFFD,
        0x0000,
    ];

    /// Random text of about `pieces` pieces, with a `\n` at about each
    /// `line` pieces.
    fn random_utf8(random: &mut Random, pieces: usize, line: usize) -> Vec<u8> {
        let mut text = Vec::new();
        for _ in 0..pieces {
            if random.below(line) == 0 {
                text.push(b'\n');
            }
            text.extend_from_slice(PIECES[random.below(PIECES.len())]);
        }
        text
    }

    fn random_utf16(random: &mut Random, units: usize, line: usize) -> Vec<u8> {
        let list: Vec<u16> = (0..units)
            .map(|_| {
                if random.below(line) == 0 {
                    0x000A
                } else {
                    UNITS[random.below(UNITS.len())]
                }
            })
            .collect();
        utf16_units(&list, random.below(2) == 0, random.below(3) == 0)
    }

    /// Texts with each byte order mark, split characters and surrogates, bad
    /// sequences, no final `\n`, and lines near the cut of 300 characters and
    /// near `MAX_KEPT_BYTES`.
    fn corpus() -> Vec<Vec<u8>> {
        let small: [&[u8]; 22] = [
            b"",
            b"\n",
            b"\n\n",
            b"a",
            b"a\n",
            b"a\nb",
            b"\r\n",
            b"one\r\ntwo\r\n",
            b"\xEF\xBB\xBF",
            b"\xEF\xBB",
            b"\xEF",
            b"\xEF\xBB\xBF\n",
            b"\xEF\xBB\xBFone\ntwo",
            b"\xFF",
            b"\xFE",
            b"\xFF\xFE",
            b"\xFE\xFF",
            b"\xFF\xFE\x0A",
            b"\xFF\xFE\x0A\x00",
            b"\xFE\xFF\x00\x0A\x00",
            b"\xFF\xFEa",
            b"\xE2\x82\n\xAC",
        ];
        let mut texts: Vec<Vec<u8>> = small.iter().map(|text| text.to_vec()).collect();
        let unit_cases: [&[u16]; 9] = [
            &[],
            &[0x0A],
            &[0x41, 0x0A, 0x42],
            &[0xD83D, 0xDE00, 0x0A],
            &[0xD83D],
            &[0xDE00],
            &[0xD83D, 0xD83D, 0xDE00],
            &[0xD83D, 0x0A, 0xDE00],
            &[0x41, 0xD800],
        ];
        let emoji: Vec<u16> = "😀"
            .repeat(400)
            .encode_utf16()
            .chain([0x0A, 0x41])
            .collect();
        for units in unit_cases.iter().copied().chain([emoji.as_slice()]) {
            for little in [true, false] {
                for odd in [false, true] {
                    texts.push(utf16_units(units, little, odd));
                }
            }
        }
        let emojis = |count: usize| "😀".repeat(count).into_bytes();
        let heads: [Vec<u8>; 13] = [
            b"a".repeat(298),
            b"a".repeat(299),
            b"a".repeat(300),
            b"a".repeat(1199),
            b"a".repeat(1200),
            emojis(299),
            emojis(300),
            [b"a".to_vec(), emojis(299)].concat(),
            [b"aa".to_vec(), emojis(298)].concat(),
            [b"aaa".to_vec(), emojis(298)].concat(),
            b"\xF0\x9F\x98".repeat(299),
            b"\xF0\x9F\x98".repeat(400),
            b"\xFF".repeat(1199),
        ];
        let tails: [&[u8]; 13] = [
            b"",
            b"\r",
            b"\xF0\x9F\x98\x80",
            b"\xF0\x9F\x98",
            b"\xF0\x9F",
            b"\xF0",
            b"\xE2\x82\xAC",
            b"\xE2\x82",
            b"\xC3\xA9",
            b"\xC3",
            b"\xFF",
            b"\x80\x80",
            b"\xED\xA0\x80",
        ];
        let fillers: [Vec<u8>; 6] = [
            Vec::new(),
            b"b".to_vec(),
            b"bbb".to_vec(),
            b"\x80".to_vec(),
            b"\xA9b".to_vec(),
            emojis(250),
        ];
        for head in &heads {
            let mut text = Vec::new();
            for tail in tails {
                for filler in &fillers {
                    text.extend_from_slice(head);
                    text.extend_from_slice(tail);
                    text.extend_from_slice(filler);
                    text.push(b'\n');
                }
            }
            texts.push(text[..text.len() - 1].to_vec());
            texts.push(text);
        }
        let mut random = Random(0x9E37_79B9_7F4A_7C15);
        for _ in 0..40 {
            let mut text = Vec::new();
            if random.below(4) == 0 {
                text.extend_from_slice(b"\xEF\xBB\xBF");
            }
            let pieces = random.below(3000);
            let line = [8, 200, 900, 5000][random.below(4)];
            text.extend(random_utf8(&mut random, pieces, line));
            texts.push(text);
        }
        for _ in 0..30 {
            let units = random.below(3000);
            let line = [8, 400, 5000][random.below(3)];
            texts.push(random_utf16(&mut random, units, line));
        }
        // Texts of more than one part of `READ_PART` bytes.
        texts.push(random_utf8(&mut random, 150_000, 60));
        texts.push(random_utf16(&mut random, 150_000, 60));
        texts
    }

    #[test]
    fn reading_in_parts_gives_the_lines_of_a_read_of_all_of_the_text() {
        let source = Source::StandardInput;
        for text in corpus() {
            let count = whole_count(&text);
            // The second windows end before the last line, so jag only counts
            // the lines after them.
            let early = vec![
                Window {
                    source: source.clone(),
                    start: 1,
                    end: 1,
                    matches: vec![1],
                },
                Window {
                    source: source.clone(),
                    start: 2,
                    end: 4,
                    matches: vec![3],
                },
            ];
            for windows in [windows_for(&source, count), early] {
                let expected = (whole_snippets(&windows, &text), count);
                for part in [1, 2, 3, 4, 5, 7, 13, 64, 1201, 4096, READ_PART] {
                    if part < 7 && text.len() > 200_000 {
                        continue;
                    }
                    assert!(
                        snippets_in_parts(&windows, &text, part) == expected,
                        "part {part}, text {:?}",
                        String::from_utf8_lossy(&text[..text.len().min(80)])
                    );
                }
            }
        }
    }

    #[test]
    fn a_file_and_standard_input_give_the_lines_of_a_read_of_all_of_the_text() {
        let dir = TempDir::new().expect("temporary folder");
        let path = dir.path().join("a.txt");
        let file = Source::File(path.clone());
        for text in corpus() {
            fs::write(&path, &text).expect("write file");
            let count = whole_count(&text);
            let windows = windows_for(&file, count);
            let (snippets, warnings) = read_snippets(&windows, None);
            assert!(warnings.is_empty());
            assert!(snippets == whole_snippets(&windows, &text));
            let windows = windows_for(&Source::StandardInput, count);
            let (snippets, warnings) = read_snippets(&windows, Some(&text));
            assert!(warnings.is_empty());
            assert!(snippets == whole_snippets(&windows, &text));
        }
    }

    #[test]
    fn jag_keeps_only_the_lines_inside_a_window() {
        let text: String = (1..=1000)
            .map(|number| format!("line {number}\n"))
            .collect();
        let mut lines = LineSplitter::new(&[window(Path::new("a"), 500, 502, &[501])]);
        for part in text.as_bytes().chunks(100) {
            lines.push(part);
        }
        let lines = lines.finish();
        assert_eq!(lines.count, 1000);
        let kept: Vec<(u64, &[u8])> = lines
            .lines
            .iter()
            .map(|line| (line.number, line.text.as_slice()))
            .collect();
        assert_eq!(
            kept,
            [
                (500, &b"line 500"[..]),
                (501, &b"line 501"[..]),
                (502, &b"line 502"[..])
            ]
        );
    }

    /// The bytes that the splitter holds now.
    fn held(lines: &LineSplitter) -> usize {
        let line = lines.line.as_ref().map_or(0, |line| {
            line.text.capacity() + line.rest.as_ref().map_or(0, |rest| rest.pending.capacity())
        });
        let kept: usize = lines.kept.iter().map(|line| line.text.capacity()).sum();
        line + kept
    }

    #[test]
    fn a_long_line_with_no_newline_uses_little_memory_and_gives_the_same_cut() {
        let most = 4 * READ_PART;
        // 16 MiB of one character.
        let part = vec![b'x'; READ_PART];
        let mut lines = LineSplitter::new(&[window(Path::new("a"), 1, 1, &[1])]);
        for _ in 0..256 {
            lines.push(&part);
            assert!(held(&lines) <= most, "holds {} bytes", held(&lines));
        }
        let lines = lines.finish();
        assert_eq!(lines.count, 1);
        assert_eq!(lines.lines[0].text, vec![b'x'; 300]);
        assert_eq!(lines.lines[0].cut, (256 * READ_PART - 300) as u64);

        // Characters of 4 bytes and bad sequences, cut in parts that split
        // them.
        let text = "😀\u{e9}".repeat(400_000).into_bytes();
        let text = [text.as_slice(), b"\xF0\x9F\xFF\xE2\x82"].concat();
        let mut lines = LineSplitter::new(&[window(Path::new("a"), 1, 1, &[1])]);
        for part in text.chunks(READ_PART + 1) {
            lines.push(part);
            assert!(held(&lines) <= most, "holds {} bytes", held(&lines));
        }
        let lines = lines.finish();
        let whole = SnippetLine::new(1, &text, true);
        assert_eq!(lines.lines[0].text, whole.text);
        assert_eq!(lines.lines[0].cut, whole.cut);
    }

    #[cfg(unix)]
    #[test]
    fn a_device_gives_a_warning_and_no_result_and_jag_reads_no_byte() {
        let dir = TempDir::new().expect("temporary folder");
        let link = dir.path().join("swapped.txt");
        std::os::unix::fs::symlink("/dev/zero", &link).expect("symbolic link");
        for path in [PathBuf::from("/dev/zero"), link] {
            for start in [1, 5] {
                let (snippets, warnings) = read_snippets(&[window(&path, start, 8, &[5])], None);
                assert!(snippets.is_empty());
                assert_eq!(
                    warnings,
                    [WalkWarning {
                        path: Some(path.clone()),
                        kind: WarningKind::Read(io::ErrorKind::InvalidInput),
                    }]
                );
            }
        }
    }

    #[cfg(unix)]
    #[test]
    fn a_folder_gives_the_warning_of_a_read_of_a_folder() {
        let dir = TempDir::new().expect("temporary folder");
        let (snippets, warnings) = read_snippets(&[window(dir.path(), 1, 4, &[1])], None);
        assert!(snippets.is_empty());
        assert_eq!(
            warnings,
            [WalkWarning {
                path: Some(dir.path().to_path_buf()),
                kind: WarningKind::Read(io::ErrorKind::IsADirectory),
            }]
        );
    }

    /// A reader that gives an error at each read.
    struct Broken;

    impl Read for Broken {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Err(io::ErrorKind::BrokenPipe.into())
        }
    }

    /// A reader that is stopped by a signal one time, then gives no byte.
    struct Interrupted(bool);

    impl Read for Interrupted {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            if std::mem::replace(&mut self.0, false) {
                Err(io::ErrorKind::Interrupted.into())
            } else {
                Ok(0)
            }
        }
    }

    #[test]
    fn a_read_error_after_the_last_line_of_a_window_gives_the_error() {
        let windows = [window(Path::new("a"), 1, 3, &[2])];
        let text = &b"1\n2\n3\n4\n5\n"[..];
        let mut buffer = [0; 4];
        let lines = read_lines(text, &windows, &mut buffer).expect("no error");
        assert_eq!(lines.count, 5);
        let error = read_lines(text.chain(Broken), &windows, &mut buffer)
            .err()
            .expect("an error");
        assert_eq!(error.kind(), io::ErrorKind::BrokenPipe);
        let lines = read_lines(Interrupted(true).chain(text), &windows, &mut buffer)
            .expect("jag reads again after a signal");
        assert_eq!(lines.count, 5);
    }
}
