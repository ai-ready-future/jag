//! C-cli: the error texts of a search with no `--ask` and no `--json`. They are
//! the texts of rg 15.2.0, with the name `jag` (FEATURE-23, decisions 11, 17,
//! and 18; ADR-0022).
//!
//! The check reads the command line in order, as `parse` of
//! `crates/core/flags/parse.rs` of rg does (it uses `lexopt`). It gives the
//! first error. clap still parses the command line after the check.

use std::collections::BTreeSet;
use std::ffi::{OsStr, OsString};

/// One option that a search with no `--ask` knows.
struct Flag {
    long: &'static str,
    short: Option<char>,
    takes_value: bool,
}

const fn switch(long: &'static str, short: Option<char>) -> Flag {
    Flag {
        long,
        short,
        takes_value: false,
    }
}

const fn value(long: &'static str, short: Option<char>) -> Flag {
    Flag {
        long,
        short,
        takes_value: true,
    }
}

/// The options of card C (the rg form) and the options that only jag has
/// (card A). `-h` and `-V` are not in the list: clap answers them.
const FLAGS: &[Flag] = &[
    switch("ignore-case", Some('i')),
    switch("word-regexp", Some('w')),
    switch("fixed-strings", Some('F')),
    value("regexp", Some('e')),
    value("after-context", Some('A')),
    value("before-context", Some('B')),
    value("context", Some('C')),
    switch("files-with-matches", Some('l')),
    switch("count", Some('c')),
    switch("quiet", Some('q')),
    value("glob", Some('g')),
    switch("line-number", Some('n')),
    switch("no-line-number", Some('N')),
    switch("with-filename", Some('H')),
    switch("no-filename", Some('I')),
    switch("heading", None),
    switch("no-heading", None),
    value("sort", None),
    value("color", None),
    switch("json", None),
    value("ask", None),
    switch("check", None),
    switch("setup", None),
    switch("forget-key", None),
    value("timeout", None),
    value("retry-wait", None),
    value("base-url", None),
    value("model", None),
    value("max-scored", None),
    value("max-chars", None),
    switch("no-max-chars", None),
];

/// The values that `--color` takes (rg 15.2.0 `ColorChoice`).
const COLOR_CHOICES: [&str; 4] = ["never", "auto", "always", "ansi"];

/// The values that `--sort` takes in jag. rg also takes `modified`,
/// `accessed`, and `created` (card C, a recorded difference).
const SORT_CHOICES: [&str; 2] = ["path", "none"];

/// The words that make the other texts: `--ask` or `--json` before the first
/// `--` (decision 17).
const OWN_FORM_OPTIONS: [&str; 2] = ["--json", "--ask"];

/// The modes that are not a search (decision 3).
const MODE_OPTIONS: [&str; 3] = ["--check", "--setup", "--forget-key"];

fn find_long(name: &str) -> Option<&'static Flag> {
    FLAGS.iter().find(|flag| flag.long == name)
}

fn find_short(letter: char) -> Option<&'static Flag> {
    FLAGS.iter().find(|flag| flag.short == Some(letter))
}

/// Check the command line, in order. `Err` holds the text of the first error,
/// with no `jag: ` in front. The check ends with `Ok` at the first `--`, at
/// `-h` and `-V` (clap answers them), and at an option of a mode that is not a
/// search.
pub fn check(args: &[OsString]) -> Result<(), String> {
    let mut words = args.iter().skip(1);
    while let Some(word) = words.next() {
        let Some(text) = word.to_str() else {
            continue;
        };
        if text == "--" {
            return Ok(());
        }
        if let Some(long) = text.strip_prefix("--") {
            let (name, attached) = match long.split_once('=') {
                Some((name, attached)) => (name, Some(attached)),
                None => (long, None),
            };
            if matches!(name, "help" | "version") || MODE_OPTIONS.contains(&format!("--{name}").as_str())
            {
                return Ok(());
            }
            let Some(flag) = find_long(name) else {
                return Err(unrecognized(&format!("--{name}"), Some(name)));
            };
            let shown = format!("--{name}");
            if flag.takes_value {
                let given = match attached {
                    Some(attached) => OsString::from(attached),
                    None => words
                        .next()
                        .cloned()
                        .ok_or_else(|| missing_value(&shown))?,
                };
                check_value(flag, &shown, &given)?;
            } else if let Some(attached) = attached {
                return Err(format!(
                    "invalid CLI arguments: unexpected argument for option '--{name}': {attached:?}"
                ));
            }
        } else if text.len() > 1 && text.starts_with('-') {
            check_short_cluster(&text[1..], &mut words)?;
        }
    }
    Ok(())
}

/// The letters after one `-`. A letter that takes a value takes the rest of
/// the word, without one `=` at its start, or else the next word.
fn check_short_cluster<'a>(
    letters: &str,
    words: &mut impl Iterator<Item = &'a OsString>,
) -> Result<(), String> {
    for (place, letter) in letters.char_indices() {
        if letter == 'h' || letter == 'V' {
            return Ok(());
        }
        let Some(flag) = find_short(letter) else {
            return Err(unrecognized(&format!("-{letter}"), None));
        };
        if !flag.takes_value {
            continue;
        }
        let shown = format!("-{letter}");
        let rest = &letters[place + letter.len_utf8()..];
        let given = if rest.is_empty() {
            words.next().cloned().ok_or_else(|| missing_value(&shown))?
        } else {
            OsString::from(rest.strip_prefix('=').unwrap_or(rest))
        };
        return check_value(flag, &shown, &given);
    }
    Ok(())
}

fn missing_value(shown: &str) -> String {
    format!("missing value for flag {shown}: missing argument for option '{shown}'")
}

fn check_value(flag: &Flag, shown: &str, given: &OsStr) -> Result<(), String> {
    let error = |detail: String| format!("error parsing flag {shown}: {detail}");
    match flag.long {
        "after-context" | "before-context" | "context" => {
            let Some(text) = given.to_str() else {
                return Err(error("value is not valid UTF-8".to_owned()));
            };
            text.parse::<usize>()
                .map(drop)
                .map_err(|why| error(format!("value is not a valid number: {why}")))
        }
        "color" => choice(given, &COLOR_CHOICES).map_err(error),
        "sort" => choice(given, &SORT_CHOICES).map_err(error),
        _ => Ok(()),
    }
}

fn choice(given: &OsStr, choices: &[&str]) -> Result<(), String> {
    let Some(text) = given.to_str() else {
        return Err("value is not valid UTF-8".to_owned());
    };
    if choices.contains(&text) {
        Ok(())
    } else {
        Err(format!("choice '{text}' is unrecognized"))
    }
}

/// `unrecognized flag --X`, and for a long name the similar options of jag
/// (decision 22).
fn unrecognized(shown: &str, long: Option<&str>) -> String {
    let mut text = format!("unrecognized flag {shown}");
    if let Some(long) = long {
        let similar = similar_names(long);
        if !similar.is_empty() {
            let list: Vec<String> = similar.iter().map(|name| format!("--{name}")).collect();
            text.push_str(&format!(
                "\n\nsimilar flags that are available: {}",
                list.join(", ")
            ));
        }
    }
    text
}

/// The long names of jag that are like `unrecognized`: the Jaccard index of
/// the 3-grams is 0.4 or more (`find_similar_names` of rg 15.2.0).
fn similar_names(unrecognized: &str) -> Vec<&'static str> {
    const THRESHOLD: f64 = 0.4;
    let given = ngrams(unrecognized);
    FLAGS
        .iter()
        .map(|flag| flag.long)
        .filter(|name| jaccard_index(&given, &ngrams(name)) >= THRESHOLD)
        .collect()
}

fn jaccard_index(one: &BTreeSet<Vec<u8>>, other: &BTreeSet<Vec<u8>>) -> f64 {
    let union = one.union(other).count();
    let both = one.intersection(other).count();
    both as f64 / union as f64
}

/// All 3-grams of a name. A name of fewer than 3 bytes is padded with `!`.
fn ngrams(name: &str) -> BTreeSet<Vec<u8>> {
    let bytes = name.as_bytes();
    match bytes.len() {
        0 => BTreeSet::from([b"!!!".to_vec()]),
        1 => BTreeSet::from([vec![bytes[0], b'!', b'!']]),
        2 => BTreeSet::from([vec![bytes[0], bytes[1], b'!']]),
        _ => bytes.windows(3).map(<[u8]>::to_vec).collect(),
    }
}

/// True when the command line asks for the text of `--ask` or `--json`: one
/// of these options comes before the first `--`. The words go from left to
/// right, as clap reads them (decision 17).
pub fn own_form_requested(args: impl IntoIterator<Item = OsString>) -> bool {
    super::option_before_double_dash(args, |word| {
        OWN_FORM_OPTIONS
            .iter()
            .any(|name| word == *name || word.strip_prefix(name).is_some_and(|rest| rest.starts_with('=')))
    })
}

/// The line of an error: `jag: ` and the text.
pub fn line(text: &str) -> String {
    format!("jag: {text}\n")
}

/// No term and no `-e` (D17).
pub const NO_PATTERN: &str = "jag requires at least one pattern to execute a search";

/// A search of the current folder that searched nothing (D18). Card E option 1
/// drops the line about `--debug`.
pub const NOTHING_SEARCHED: &str = "No files were searched, which means jag probably applied a filter you didn't expect.";

#[cfg(test)]
mod tests {
    use super::*;

    fn run(list: &[&str]) -> Result<(), String> {
        let mut all = vec![OsString::from("jag")];
        all.extend(list.iter().map(OsString::from));
        check(&all)
    }

    #[test]
    fn a_good_command_line_passes() {
        for list in [
            &["foo", "a.txt"][..],
            &["-iA2", "foo"],
            &["-A=2", "foo"],
            &["--sort", "path", "-g", "*.rs", "foo"],
            &["--color=always", "foo"],
            &["-e", "-x", "--", "--nope"],
            &["--", "--nope"],
            &["-h"],
            &["--setup", "--nope"],
        ] {
            assert_eq!(run(list), Ok(()), "{list:?}");
        }
    }

    #[test]
    fn each_error_has_the_text_of_rg() {
        let cases: [(&[&str], &str); 9] = [
            (&["-A", "x", "foo"], "error parsing flag -A: value is not a valid number: invalid digit found in string"),
            (&["-A", "-1", "foo"], "error parsing flag -A: value is not a valid number: invalid digit found in string"),
            (&["--context=x", "foo"], "error parsing flag --context: value is not a valid number: invalid digit found in string"),
            (&["foo", "a.txt", "-A"], "missing value for flag -A: missing argument for option '-A'"),
            (&["foo", "--after-context"], "missing value for flag --after-context: missing argument for option '--after-context'"),
            (&["--color", "foo"], "error parsing flag --color: choice 'foo' is unrecognized"),
            (&["--sort", "size", "foo"], "error parsing flag --sort: choice 'size' is unrecognized"),
            (&["-X", "foo"], "unrecognized flag -X"),
            (&["--count=1", "foo"], "invalid CLI arguments: unexpected argument for option '--count': \"1\""),
        ];
        for (list, text) in cases {
            assert_eq!(run(list), Err(text.to_owned()), "{list:?}");
        }
    }

    #[test]
    fn an_unknown_long_option_lists_the_similar_options_of_jag() {
        assert_eq!(
            run(&["--colour=always", "foo"]),
            Err("unrecognized flag --colour\n\nsimilar flags that are available: --color".to_owned())
        );
        assert_eq!(
            run(&["--frobnicate", "foo"]),
            Err("unrecognized flag --frobnicate".to_owned())
        );
    }

    #[test]
    fn the_first_error_wins() {
        assert_eq!(
            run(&["-A", "x", "--nope"]),
            Err("error parsing flag -A: value is not a valid number: invalid digit found in string".to_owned())
        );
        assert_eq!(run(&["--nope", "-A", "x"]), Err("unrecognized flag --nope".to_owned()));
    }
}
