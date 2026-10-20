//! The README examples for people (FEATURE-27, CRITERION-134 and
//! CRITERION-135).
//!
//! Each example of the README section `## Examples` has a marker
//! `<!-- example: ID -->` and a fenced block: a command, then what jag shows.
//! The test runs the command on a copy of `tests/readme/sample/`, with a fake
//! model service, and compares what jag shows with the block, byte for byte.
//!
//! To read what jag shows for each example, run
//! `JAG_README_BLESS=1 cargo test --test readme_examples -- --nocapture`. The
//! switch prints the text of each example. It does not write `README.md`.

mod common;
#[path = "parity/harness.rs"]
mod harness;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use common::KEY;
use common::fake_server::{FakeServer, Reply};
use harness::{Invocation, RunMode, copy_tree, run};

/// The input tokens that the fake service reports for each snippet of a call.
const TOKENS_PER_SNIPPET: u64 = 750;

/// The score of a snippet that no row of `scores` names.
const OTHER_SCORE: f64 = 0.2;

/// One example: its ID in the README, the fake scores, and the exit code.
struct Example {
    id: &'static str,
    /// The score of a snippet is the score of the first row whose text is in
    /// the lines of the snippet. A snippet that no row names gets
    /// `OTHER_SCORE`.
    scores: &'static [(&'static str, f64)],
    /// Whether the command calls the model service.
    needs_model: bool,
    code: i32,
}

const EXAMPLES: &[Example] = &[
    Example {
        id: "operator-pipe",
        scores: &[("step 6 of 6 failed", 0.95), ("health check failed", 0.4)],
        needs_model: true,
        code: 0,
    },
    Example {
        id: "notes-question",
        scores: &[("25 days", 0.95), ("30 days", 0.5), ("14 days", 0.3)],
        needs_model: true,
        code: 0,
    },
    Example {
        id: "code-question",
        scores: &[("time.sleep(retry_wait)", 0.9), ("retry_count", 0.5)],
        needs_model: true,
        code: 0,
    },
    Example {
        id: "no-question",
        scores: &[],
        needs_model: false,
        code: 0,
    },
    Example {
        id: "no-good-answer",
        scores: &[("security policy", 0.25), ("travel policy", 0.15)],
        needs_model: true,
        code: 1,
    },
    Example {
        id: "size-limit",
        scores: &[
            ("volume 4", 0.9),
            ("volume 2", 0.85),
            ("volume 6", 0.8),
            ("volume 1", 0.75),
            ("volume 3", 0.7),
            ("volume 5", 0.65),
        ],
        needs_model: true,
        code: 8,
    },
    Example {
        id: "missing-key",
        scores: &[],
        needs_model: false,
        code: 3,
    },
];

/// The paragraph that starts the section (spec row 7).
const NOTE: &str = "The output below is what jag shows in a terminal, with no color. It comes from a test of jag. The test searches a small sample folder, and a test service takes the place of the model. The scores, the order of the results, and the cost come from that test service. With the real model, the order of your results and your cost can be different. In a pipe, the layout and the number of results can be different.";

/// One example of the README.
struct Found {
    id: String,
    /// The line before the marker that is not empty.
    sentence: String,
    command: String,
    /// What the block shows after the command line, each line with `\n`.
    shown: String,
}

fn readme() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("README.md");
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

/// The text with each run of white space as one space.
fn flat(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The text of `## Examples`: from its heading to the next `## ` heading.
fn examples_section(readme: &str) -> &str {
    let start = readme
        .find("\n## Examples\n")
        .expect("the README has the section ## Examples")
        + 1;
    let rest = &readme[start..];
    match rest[3..].find("\n## ") {
        Some(end) => &rest[..end + 3],
        None => rest,
    }
}

/// Each example of the section, in the order of the README.
fn found_examples(section: &str) -> Vec<Found> {
    let lines: Vec<&str> = section.lines().collect();
    let mut found = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        let Some(id) = line
            .strip_prefix("<!-- example: ")
            .and_then(|rest| rest.strip_suffix(" -->"))
        else {
            continue;
        };
        let sentence = lines[..index]
            .iter()
            .rev()
            .find(|line| !line.trim().is_empty())
            .unwrap_or_else(|| panic!("{id}: no sentence before the marker"))
            .to_string();
        assert_eq!(
            lines.get(index + 1),
            Some(&"```"),
            "{id}: the line after the marker opens a block with no language tag"
        );
        let end = lines[index + 2..]
            .iter()
            .position(|line| *line == "```")
            .unwrap_or_else(|| panic!("{id}: the block does not end"))
            + index
            + 2;
        let block = &lines[index + 2..end];
        let command = block
            .first()
            .and_then(|line| line.strip_prefix("$ "))
            .unwrap_or_else(|| panic!("{id}: the first line of the block starts with `$ `"))
            .to_string();
        let shown = block[1..].iter().map(|line| format!("{line}\n")).collect();
        found.push(Found {
            id: id.to_owned(),
            sentence,
            command,
            shown,
        });
    }
    found
}

/// The text without the color codes (`ESC [` ... `m`).
fn strip_colors(text: &str) -> String {
    let mut out = String::new();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\u{1b}' && chars.peek() == Some(&'[') {
            for next in chars.by_ref() {
                if next == 'm' {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

fn score_of(example: &Example, lines: &str) -> f64 {
    example
        .scores
        .iter()
        .find(|(text, _)| lines.contains(text))
        .map_or(OTHER_SCORE, |(_, score)| *score)
}

/// The fake service of an example: the scores of its table, and 750 input
/// tokens for each snippet of a call.
fn fake_service(example: &'static Example) -> FakeServer {
    FakeServer::start(move |request| {
        let snippets = request.snippet_texts().len() as u64;
        let usage = serde_json::json!({
            "input_tokens": TOKENS_PER_SNIPPET * snippets,
            "output_tokens": 1,
        });
        Reply::scores_with_usage(request, |lines| score_of(example, lines), Some(usage))
    })
}

/// What a person sees: the notices of the error stream, then standard output,
/// then the cost line (spec rows 9 and 14).
fn shown_text(stdout: &[u8], stderr: &[u8]) -> String {
    let stderr = String::from_utf8_lossy(stderr).into_owned();
    let stdout = strip_colors(&String::from_utf8_lossy(stdout));
    let mut notices = String::new();
    let mut cost = String::new();
    let lines: Vec<&str> = stderr.split_inclusive('\n').collect();
    for (index, line) in lines.iter().enumerate() {
        if index + 1 == lines.len() && line.starts_with("jag: cost ") {
            cost.push_str(line);
        } else {
            notices.push_str(line);
        }
    }
    format!("{notices}{stdout}{cost}")
}

/// Run the command of an example on a copy of the sample folder.
fn run_example(example: &'static Example, command: &str) -> (i32, String) {
    let sample = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/readme/sample");
    let work = tempfile::tempdir().expect("work folder");
    let home = tempfile::tempdir().expect("home folder");
    copy_tree(&sample, work.path());

    let bin = assert_cmd::cargo::cargo_bin!("jag");
    let bin_dir: PathBuf = bin.parent().expect("the binary has a folder").to_owned();
    let mut env: BTreeMap<String, Option<String>> = BTreeMap::new();
    env.insert(
        "PATH".to_owned(),
        Some(format!("{}:/usr/bin:/bin", bin_dir.display())),
    );
    // The server lives until the end of the run.
    let server = example.needs_model.then(|| fake_service(example));
    if let Some(server) = &server {
        env.insert("JAG_GATEWAY_API_KEY".to_owned(), Some(KEY.to_owned()));
        env.insert("JAG_BASE_URL".to_owned(), Some(server.base_url()));
    }
    let args = ["-c".to_owned(), command.to_owned()];
    let invocation = Invocation {
        args: &args,
        stdin: None,
        env: &env,
    };
    let result = run(
        Path::new("/bin/sh"),
        &invocation,
        RunMode::Terminal,
        work.path(),
        home.path(),
    )
    .unwrap_or_else(|error| panic!("{}: {error}", example.id));
    (result.code, shown_text(&result.stdout, &result.stderr))
}

// CRITERION-134: the section exists, for people, and each example has a
// command and an output.
#[test]
fn the_readme_has_an_examples_section_for_people() {
    let readme = readme();
    let section = examples_section(&readme);
    assert!(
        flat(section).contains(NOTE),
        "the section lacks the paragraph about the test service"
    );
    assert!(!section.contains("--json"), "an example is for people");
    // The section is between `## Check that jag is ready` and
    // `## Search with no question`.
    let before = readme.find("\n## Check that jag is ready\n").unwrap();
    let here = readme.find("\n## Examples\n").unwrap();
    let after = readme.find("\n## Search with no question\n").unwrap();
    assert!(before < here && here < after);
    assert!(!readme.contains("This release has the key setup only"));

    let found = found_examples(section);
    let ids: Vec<&str> = found.iter().map(|example| example.id.as_str()).collect();
    let want: Vec<&str> = EXAMPLES.iter().map(|example| example.id).collect();
    assert_eq!(ids, want, "the examples are those of the table, in order");
}

// CRITERION-135: each example gives the output and the exit code that the
// README shows.
#[test]
fn each_readme_example_gives_the_output_that_it_shows() {
    let readme = readme();
    let found = found_examples(examples_section(&readme));

    // One to one: each marker has a row, and each row has one marker.
    let ids: BTreeSet<&str> = found.iter().map(|example| example.id.as_str()).collect();
    assert_eq!(ids.len(), found.len(), "an ID appears two times");
    let rows: BTreeSet<&str> = EXAMPLES.iter().map(|example| example.id).collect();
    assert_eq!(ids, rows);

    let bless = std::env::var_os("JAG_README_BLESS").is_some();
    let mut failures = Vec::new();
    for example in EXAMPLES {
        let found = found
            .iter()
            .find(|found| found.id == example.id)
            .expect("one marker for each row");
        let (code, text) = run_example(example, &found.command);
        if bless {
            println!(
                "=== {} (exit code {code})\n$ {}\n{text}=== end",
                example.id, found.command
            );
            continue;
        }
        assert!(
            !text.contains(KEY),
            "{}: the output holds the key",
            found.id
        );
        assert!(
            !found.shown.contains(KEY),
            "{}: the README holds the key",
            found.id
        );
        if code != example.code {
            failures.push(format!(
                "{}: exit code {code}, the table says {}",
                found.id, example.code
            ));
        }
        if example.code != 0 {
            let words = format!("exit code {}", example.code);
            if !found.sentence.contains(&words) {
                failures.push(format!(
                    "{}: the sentence before the block does not say \"{words}\"",
                    found.id
                ));
            }
        }
        if text != found.shown {
            failures.push(format!(
                "{}: the README shows\n{}but jag shows\n{}",
                found.id, found.shown, text
            ));
        }
    }
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
}
