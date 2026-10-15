//! C-cli: the grep options of FEATURE-13 and FEATURE-23. The error of an
//! unknown option, the words with `-e`, the matcher, the lines around a match,
//! the checks of `-l`, `-c`, `-q`, and the layout options, the color, and the
//! globs.

use std::ffi::OsStr;
use std::path::Path;

use clap::error::{ContextKind, ContextValue};
use grep_regex::{RegexMatcher, RegexMatcherBuilder};

use super::{Cli, usage_failure};
use crate::failure::Failure;
use crate::search::FileFilter;
use crate::search::snippet::{ContextLines, MAX_LINES};

/// The most lines on each side of a match with `--ask` (decision 14).
const MAX_CONTEXT_WITH_ASK: u64 = 12;

/// The longest name of an option that an error shows, after `--` (decision 18).
const MAX_OPTION_NAME: usize = 30;

/// The failure of an unknown option: its name, when the text that clap gives
/// has the form of an option. `None` for each other text: the caller then
/// gives the failure of each argument error (decision 18).
pub(super) fn unknown_option_failure(error: &clap::Error) -> Option<Failure> {
    match error.get(ContextKind::InvalidArg) {
        Some(ContextValue::String(name)) if is_option_form(name) => Some(usage_failure(
            &format!("the option {name} is not valid."),
            "Run jag --help to see the options.",
        )),
        _ => None,
    }
}

/// True for `-` and one ASCII letter or digit, or for `--` and 1 to 30 ASCII
/// letters, digits, or `-`.
fn is_option_form(text: &str) -> bool {
    match text.strip_prefix("--") {
        Some(long) => {
            (1..=MAX_OPTION_NAME).contains(&long.len())
                && long
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
        }
        None => text
            .strip_prefix('-')
            .is_some_and(|short| short.len() == 1 && short.as_bytes()[0].is_ascii_alphanumeric()),
    }
}

/// The words after the options: with no `-e`, the term and the paths. With
/// `-e`, each word is a path (decision 10).
pub(super) fn words(cli: &Cli) -> Vec<&OsStr> {
    cli.term
        .iter()
        .map(|term| term.as_os_str())
        .chain(cli.paths.iter().map(|path| path.as_os_str()))
        .collect()
}

/// The options that change the matcher (decisions 7 to 9).
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct MatchOptions {
    pub ignore_case: bool,
    pub word: bool,
    pub fixed: bool,
}

impl MatchOptions {
    pub(super) fn of(cli: &Cli) -> MatchOptions {
        MatchOptions {
            ignore_case: cli.ignore_case,
            word: cli.word,
            fixed: cli.fixed,
        }
    }
}

/// The matcher of the terms, with -i, -w, and -F. The settings are the
/// settings of `matcher_rust` in rg 15.2.0 (FEATURE-23, decision 2). A term
/// that is two times in the list counts one time, as in rg.
pub(super) fn build_matcher(
    terms: &[&str],
    options: MatchOptions,
) -> Result<RegexMatcher, grep_regex::Error> {
    let mut patterns: Vec<&str> = Vec::with_capacity(terms.len());
    for term in terms {
        if !patterns.contains(term) {
            patterns.push(term);
        }
    }
    RegexMatcherBuilder::new()
        .multi_line(true)
        .unicode(true)
        .octal(false)
        .fixed_strings(options.fixed)
        .case_insensitive(options.ignore_case)
        .word(options.word)
        .line_terminator(Some(b'\n'))
        .dot_matches_new_line(false)
        .ban_byte(Some(b'\0'))
        .build_many(&patterns)
}

/// The failure of a term that is not a regular expression, in a search with
/// `--ask` or `--json`.
pub(super) fn matcher_failure() -> Failure {
    usage_failure(
        "the term is not a valid regular expression.",
        "Check the term. It uses the regular expression rules of ripgrep.",
    )
}

/// What plain grep writes (decisions 19 to 21).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Report {
    /// The matching lines (FEATURE-12).
    Lines,
    /// `-l`: the paths of the files with a match.
    Files,
    /// `-c`: the count of matching lines of each file.
    Counts,
    /// `-q`: nothing, only the exit code.
    Quiet,
}

impl Report {
    /// The report of the options: `-q` wins, then `-l`, then `-c` (rg).
    pub(super) fn of(cli: &Cli) -> Report {
        if cli.quiet {
            Report::Quiet
        } else if cli.files_with_matches {
            Report::Files
        } else if cli.count {
            Report::Counts
        } else {
            Report::Lines
        }
    }
}

/// The options that work only in a search with no `--ask` and no `--json`:
/// `-l`, `-c`, `-q` (decisions 19 to 22 of FEATURE-13), and the layout options
/// of FEATURE-23 (decision 23). The first one in a fixed order gives the
/// failure, and `--json` is checked first.
pub(super) fn refuse_plain_options(cli: &Cli, question: bool) -> Result<(), Failure> {
    let options = [
        (cli.quiet, "-q"),
        (cli.files_with_matches, "-l"),
        (cli.count, "-c"),
        (cli.line_number, "-n"),
        (cli.no_line_number, "-N"),
        (cli.with_filename, "-H"),
        (cli.no_filename, "-I"),
        (cli.heading, "--heading"),
        (cli.no_heading, "--no-heading"),
    ];
    let Some((_, name)) = options.into_iter().find(|(given, _)| *given) else {
        return Ok(());
    };
    if cli.json {
        return Err(usage_failure(
            &format!("the option {name} does not work with --json."),
            &format!("Remove --json, or remove {name}."),
        ));
    }
    if question {
        return Err(usage_failure(
            &format!("the option {name} does not work with --ask."),
            &format!("Remove --ask, or remove {name}."),
        ));
    }
    Ok(())
}

/// The lines around a match (decisions 12 to 15). `None` when no option of
/// the three is given. `question` is true in a question search (with --ask).
pub(super) fn context_lines(cli: &Cli, question: bool) -> Result<Option<ContextLines>, Failure> {
    let after = count_of_lines(cli.after.as_deref(), "-A")?;
    let before = count_of_lines(cli.before.as_deref(), "-B")?;
    let both = count_of_lines(cli.context.as_deref(), "-C")?;
    if after.is_none() && before.is_none() && both.is_none() {
        return Ok(None);
    }
    // -A and -B win over -C for their side (decision 13).
    let before = before.or(both).unwrap_or((0, "-B"));
    let after = after.or(both).unwrap_or((0, "-A"));
    if !question {
        return Ok(Some(ContextLines {
            before: before.0,
            after: after.0,
            max_lines: None,
        }));
    }
    for (value, name) in [before, after] {
        if value > MAX_CONTEXT_WITH_ASK {
            return Err(usage_failure(
                &format!("the value of {name} is more than 12 with --ask."),
                "Give a number from 0 to 12.",
            ));
        }
    }
    let lines = before.0 + 1 + after.0;
    Ok(Some(ContextLines {
        before: before.0,
        after: after.0,
        max_lines: Some(lines.max(MAX_LINES)),
    }))
}

/// The value of `-A`, `-B`, or `-C` with the name of the option: ASCII digits
/// only, after a trim of white space. A number that is too large counts as
/// `u64::MAX`, as grep takes a very large number (decision 12). The message
/// never shows the value.
fn count_of_lines(
    text: Option<&str>,
    name: &'static str,
) -> Result<Option<(u64, &'static str)>, Failure> {
    let Some(text) = text else {
        return Ok(None);
    };
    let digits = text.trim();
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(usage_failure(
            &format!("the value of {name} is not a whole number of 0 or more."),
            "Give a number of lines, for example 3.",
        ));
    }
    Ok(Some((digits.parse().unwrap_or(u64::MAX), name)))
}

/// The filter of `-g` and `--glob` (decision 20). The message never shows the
/// glob.
pub(super) fn file_filter(globs: &[String], cwd: &Path) -> Result<FileFilter, Failure> {
    FileFilter::new(globs, cwd).map_err(|_| {
        usage_failure(
            "the glob of -g is not valid.",
            "Check the glob. jag uses the glob rules of ripgrep.",
        )
    })
}

/// The values of `--color` (rg 15.2.0). `ansi` is `always` on Unix.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ColorWhen {
    Never,
    Always,
    Auto,
}

impl ColorWhen {
    /// The value of the last `--color`, or `Auto` when no option is given.
    /// `None` for a value that rg does not know.
    pub(super) fn parse(text: Option<&str>) -> Option<ColorWhen> {
        match text {
            None | Some("auto") => Some(ColorWhen::Auto),
            Some("never") => Some(ColorWhen::Never),
            Some("always" | "ansi") => Some(ColorWhen::Always),
            Some(_) => None,
        }
    }
}

/// The checks of `--color` and `--sort` for a search with `--ask` or `--json`.
/// The messages never show the value.
pub(super) fn check_color_and_sort(cli: &Cli) -> Result<ColorWhen, Failure> {
    let when = ColorWhen::parse(cli.color.as_deref()).ok_or_else(|| {
        usage_failure(
            "the value of --color is not valid.",
            "Use never, auto, always, or ansi.",
        )
    })?;
    if let Some(sort) = cli.sort.as_deref()
        && !matches!(sort, "path" | "none")
    {
        return Err(usage_failure(
            "the value of --sort is not valid.",
            "Use path or none.",
        ));
    }
    Ok(when)
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;

    use grep_matcher::Matcher;

    use super::*;
    use crate::cli::{SearchArgs, parse_from, search_args};

    fn parsed(list: &[&str]) -> Cli {
        let mut all = vec!["jag"];
        all.extend_from_slice(list);
        parse_from(all.iter().map(OsString::from)).expect("parse")
    }

    fn refused(list: &[&str]) -> clap::Error {
        let mut all = vec!["jag"];
        all.extend_from_slice(list);
        parse_from(all.iter().map(OsString::from)).expect_err("an error")
    }

    #[test]
    fn each_short_and_long_name_parses_to_its_field() {
        for list in [
            &["-i", "-w", "-F", "-l", "-c", "-q", "t", "p"][..],
            &[
                "--ignore-case",
                "--word-regexp",
                "--fixed-strings",
                "--files-with-matches",
                "--count",
                "--quiet",
                "t",
                "p",
            ],
            &["-iwFlcq", "t", "p"],
        ] {
            let cli = parsed(list);
            assert!(cli.ignore_case && cli.word && cli.fixed, "{list:?}");
            assert!(cli.files_with_matches && cli.count && cli.quiet, "{list:?}");
        }
        assert!(parsed(&["--silent", "t", "p"]).quiet);
        let cli = parsed(&["-iA2", "t", "p"]);
        assert!(cli.ignore_case);
        assert_eq!(cli.after.as_deref(), Some("2"));
        for list in [
            &["-A", "2", "-B", "3", "-C", "4", "t", "p"][..],
            &["-A2", "-B3", "-C4", "t", "p"],
            &[
                "--after-context=2",
                "--before-context",
                "3",
                "--context=4",
                "t",
                "p",
            ],
        ] {
            let cli = parsed(list);
            let values = (cli.after, cli.before, cli.context);
            let want = (Some("2".into()), Some("3".into()), Some("4".into()));
            assert_eq!(values, want, "{list:?}");
        }
        assert_eq!(
            parsed(&["-A", "2", "-A", "5", "t", "p"]).after.as_deref(),
            Some("5")
        );
        assert_eq!(parsed(&["-A", "-1", "t", "p"]).after.as_deref(), Some("-1"));
    }

    #[test]
    fn e_takes_a_term_that_starts_with_a_dash() {
        assert_eq!(parsed(&["-e", "-x", "p"]).terms, ["-x"]);
        assert_eq!(parsed(&["-e-x", "p"]).terms, ["-x"]);
        assert_eq!(parsed(&["--regexp=a", "-e", "b", "p"]).terms, ["a", "b"]);
    }

    // Decision 37: a short option reads one `=` after its letter as the
    // separator, as rg does. GNU grep reads it as a part of the value.
    #[test]
    fn a_short_option_reads_an_equals_sign_as_the_separator() {
        assert_eq!(parsed(&["-e=foo", "p"]).terms, ["foo"]);
        let cli = parsed(&["-ie=FOO", "p"]);
        assert!(cli.ignore_case);
        assert_eq!(cli.terms, ["FOO"]);
        assert_eq!(parsed(&["-e==foo", "p"]).terms, ["=foo"]);
        assert_eq!(parsed(&["-e=", "p"]).terms, [""]);
        assert_eq!(parsed(&["-e", "-e=foo", "p"]).terms, ["-e=foo"]);
        assert_eq!(parsed(&["-A=2", "t", "p"]).after.as_deref(), Some("2"));
        assert_eq!(parsed(&["-B=2", "t", "p"]).before.as_deref(), Some("2"));
        assert_eq!(parsed(&["-C=2", "t", "p"]).context.as_deref(), Some("2"));
    }

    #[test]
    fn the_globs_keep_the_order_of_the_command_line() {
        let cli = parsed(&[
            "--exclude=*.md",
            "--include",
            "*.rs",
            "--ask",
            "q",
            "needle",
            "dir",
            "--exclude=c.*",
        ]);
        assert_eq!(
            cli.globs,
            [
                (GlobKind::Exclude, "*.md".to_owned()),
                (GlobKind::Include, "*.rs".to_owned()),
                (GlobKind::Exclude, "c.*".to_owned()),
            ]
        );
    }

    #[test]
    fn each_option_conflicts_with_check_setup_and_forget_key() {
        let options: [&[&str]; 12] = [
            &["-i"],
            &["-w"],
            &["-F"],
            &["-e", "x"],
            &["-A", "1"],
            &["-B", "1"],
            &["-C", "1"],
            &["-l"],
            &["-c"],
            &["-q"],
            &["--include=*.rs"],
            &["--exclude=*.rs"],
        ];
        for mode in ["--check", "--setup", "--forget-key"] {
            for option in options {
                let mut list = vec![mode];
                list.extend_from_slice(option);
                let error = refused(&list);
                assert_eq!(
                    error.kind(),
                    clap::error::ErrorKind::ArgumentConflict,
                    "{list:?}"
                );
            }
        }
    }

    fn paths(args: &SearchArgs) -> Vec<String> {
        args.paths
            .iter()
            .map(|path| path.display().to_string())
            .collect()
    }

    #[test]
    fn with_e_each_word_is_a_path() {
        let cli = parsed(&["-e", "needle", "a.txt", "b.txt"]);
        let args = search_args(&cli).expect("args");
        assert_eq!(args.terms, ["needle"]);
        assert_eq!(paths(&args), ["a.txt", "b.txt"]);

        let cli = parsed(&["-e", "--ask", "q", "needle"]);
        let args = search_args(&cli).expect("args");
        assert_eq!(
            (args.terms.as_slice(), args.question),
            (&["--ask"][..], None)
        );
        assert_eq!(paths(&args), ["q", "needle"]);

        let cli = parsed(&["-e", "needle"]);
        let args = search_args(&cli).expect("args");
        assert_eq!(args.terms, ["needle"]);
        assert!(args.paths.is_empty());
    }

    #[test]
    fn a_word_that_is_not_utf8_is_a_path_only_with_e() {
        let bad = OsString::from_vec(b"a\xff".to_vec());
        let with_e = parse_from([OsString::from("jag"), "-e".into(), "x".into(), bad.clone()])
            .expect("parse");
        let args = search_args(&with_e).expect("args");
        assert_eq!(args.paths, [std::path::PathBuf::from(bad.clone())]);

        let no_e = parse_from([OsString::from("jag"), bad, "p".into()]).expect("parse");
        let failure = search_args(&no_e).expect_err("a failure");
        assert_eq!(failure.cause, "the command line is not valid.");
    }

    fn matches(terms: &[&str], options: MatchOptions, line: &str) -> bool {
        let matcher = build_matcher(terms, options).expect("a matcher");
        matcher.is_match(line.as_bytes()).expect("no error")
    }

    #[test]
    fn the_matcher_follows_i_w_f_and_e() {
        let plain = MatchOptions::default();
        let case = MatchOptions {
            ignore_case: true,
            ..plain
        };
        let word = MatchOptions {
            word: true,
            ..plain
        };
        let fixed = MatchOptions {
            fixed: true,
            ..plain
        };
        assert!(!matches(&["needle"], plain, "Needle"));
        assert!(matches(&["needle"], case, "Needle"));
        assert!(matches(&["needle"], word, "the needle."));
        assert!(!matches(&["needle"], word, "needles"));
        assert!(!matches(&["needle"], word, "needle_x"));
        assert!(matches(&["-2"], word, "foo -2 bar"));
        assert!(matches(&["one", "three"], plain, "one"));
        assert!(matches(&["one", "three"], plain, "three"));
        assert!(!matches(&["one", "three"], plain, "two"));
        assert!(matches(&["needle."], fixed, "the needle."));
        assert!(!matches(&["needle."], fixed, "needles"));
        assert!(matches(&["a("], fixed, "x a( y"));
        let failure = build_matcher(&["a("], plain).expect_err("a failure");
        assert_eq!(failure.cause, "the term is not a valid regular expression.");
    }

    #[test]
    fn the_option_form_has_a_dash_and_a_short_name_or_two_dashes_and_30_characters() {
        for text in [
            "-Z",
            "-1",
            "--nope",
            "--a-b",
            &format!("--{}", "a".repeat(30)),
        ] {
            assert!(is_option_form(text), "{text}");
        }
        for text in [
            "-",
            "--",
            "-é",
            "-ab",
            "--a_b",
            "--a=b",
            &format!("--{}", "a".repeat(31)),
            "SENTINEL",
        ] {
            assert!(!is_option_form(text), "{text}");
        }
    }

    #[test]
    fn an_unknown_option_gives_its_name_and_never_its_value() {
        for (list, name) in [
            (&["--nope"][..], "--nope"),
            (&["--nope=SENTINEL", "t", "p"], "--nope"),
            (&["-iZx", "t", "p"], "-Z"),
            (&["-r", "t", "p"], "-r"),
        ] {
            let failure = unknown_option_failure(&refused(list)).expect("a name");
            assert_eq!(failure.cause, format!("the option {name} is not valid."));
            assert_eq!(failure.fix, "Run jag --help to see the options.");
            assert_eq!(failure.exit_code, 2);
        }
        let long = format!("--{}", "SENTINEL-".repeat(4));
        assert!(unknown_option_failure(&refused(&[&long])).is_none());
    }

    fn context(list: &[&str], question: bool) -> Result<Option<ContextLines>, Failure> {
        let mut all = list.to_vec();
        all.extend_from_slice(&["t", "p"]);
        context_lines(&parsed(&all), question)
    }

    fn sides(list: &[&str]) -> (u64, u64) {
        let lines = context(list, false).expect("valid").expect("an option");
        (lines.before, lines.after)
    }

    #[test]
    fn a_and_b_win_over_c_and_the_last_value_wins() {
        assert_eq!(context(&[], true).expect("valid"), None);
        assert_eq!(context(&[], false).expect("valid"), None);
        assert_eq!(sides(&["-C1", "-A3"]), (1, 3));
        assert_eq!(sides(&["-A3", "-C1"]), (1, 3));
        assert_eq!(sides(&["-A", "5"]), (0, 5));
        assert_eq!(sides(&["-B", "5"]), (5, 0));
        assert_eq!(sides(&["-C", "2", "-A", "1", "-B", "0"]), (0, 1));
        assert_eq!(sides(&["-A", " 1 "]), (0, 1));
        assert_eq!(sides(&["-C", "99999999999999999999"]), (u64::MAX, u64::MAX));
        let plain = context(&["-C", "100"], false)
            .expect("valid")
            .expect("an option");
        assert_eq!(plain.max_lines, None);
    }

    #[test]
    fn with_ask_each_side_takes_0_to_12_lines() {
        let lines = context(&["-C", "12"], true)
            .expect("valid")
            .expect("an option");
        assert_eq!(lines.max_lines, Some(25));
        let lines = context(&["-A", "2"], true)
            .expect("valid")
            .expect("an option");
        assert_eq!(
            (lines.before, lines.after, lines.max_lines),
            (0, 2, Some(12))
        );
        for (list, name) in [
            (&["-C", "13"][..], "-C"),
            (&["-A", "13"], "-A"),
            (&["-B", "13", "-A", "1"], "-B"),
            (&["-C", "13", "-A", "1"], "-C"),
        ] {
            let failure = context(list, true).expect_err("a failure");
            assert_eq!(
                failure.cause,
                format!("the value of {name} is more than 12 with --ask.")
            );
            assert_eq!(failure.fix, "Give a number from 0 to 12.");
        }
    }

    #[test]
    fn a_value_that_is_not_a_whole_number_is_refused_and_not_shown() {
        for value in ["abc", "-1", "+1", "", "1.5", "SENTINEL"] {
            for name in ["-A", "-B", "-C"] {
                let failure = context(&[name, value], false).expect_err("a failure");
                assert_eq!(
                    failure.cause,
                    format!("the value of {name} is not a whole number of 0 or more.")
                );
                assert_eq!(failure.fix, "Give a number of lines, for example 3.");
                assert!(!failure.cause.contains("SENTINEL"));
            }
        }
    }

    fn report_of(list: &[&str]) -> Result<Report, Failure> {
        let mut all = list.to_vec();
        all.extend_from_slice(&["t", "p"]);
        let cli = parsed(&all);
        report(&cli, cli.ask.is_some())
    }

    #[test]
    fn q_wins_then_l_then_c_and_none_works_with_json_or_ask() {
        assert_eq!(report_of(&[]).expect("valid"), Report::Lines);
        assert_eq!(report_of(&["-c"]).expect("valid"), Report::Counts);
        assert_eq!(report_of(&["-c", "-l"]).expect("valid"), Report::Files);
        assert_eq!(
            report_of(&["-l", "-c", "-q"]).expect("valid"),
            Report::Quiet
        );
        let failure = report_of(&["-l", "--json", "--ask", "q"]).expect_err("a failure");
        assert_eq!(failure.cause, "the option -l does not work with --json.");
        assert_eq!(failure.fix, "Remove --json, or remove -l.");
        let failure = report_of(&["-c", "-q", "--ask", "q"]).expect_err("a failure");
        assert_eq!(failure.cause, "the option -q does not work with --ask.");
        assert_eq!(failure.fix, "Remove --ask, or remove -q.");
    }

    #[test]
    fn a_glob_that_does_not_parse_names_its_option_and_not_the_glob() {
        let failure =
            file_filter(&[(GlobKind::Exclude, "SENTINEL[".to_owned())]).expect_err("a failure");
        assert_eq!(failure.cause, "the glob of --exclude is not valid.");
        assert_eq!(
            failure.fix,
            "Check the glob. jag uses the glob rules of ripgrep."
        );
    }
}
