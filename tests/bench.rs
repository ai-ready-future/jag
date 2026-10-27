//! Tests of `jag-bench` (STORY-78). They use the invented cases of
//! `tests/bench/` and the fake model service. No test reads a key file, and no
//! test calls the real model service.

#![cfg(feature = "bench")]

mod common;

use std::fs;
use std::path::{Path, PathBuf};

use assert_cmd::Command;
use common::KEY;
use common::fake_server::{FakeServer, Reply};
use serde_json::{Value, json};

/// The variables that `ureq` reads for a proxy.
const PROXY_VARIABLES: [&str; 8] = [
    "HTTP_PROXY",
    "HTTPS_PROXY",
    "ALL_PROXY",
    "NO_PROXY",
    "http_proxy",
    "https_proxy",
    "all_proxy",
    "no_proxy",
];

/// The score of the fake model: `KEEP-A` is the best answer and `KEEP-B` the
/// second best. All other snippets get the same low score.
fn model() -> FakeServer {
    FakeServer::start(|request| {
        Reply::scores(request, |text| {
            if text.contains("KEEP-A") {
                0.9
            } else if text.contains("KEEP-B") {
                0.6
            } else {
                0.1
            }
        })
    })
}

/// A copy of `tests/bench/` in a new folder of the test.
fn fixtures() -> tempfile::TempDir {
    fn copy(from: &Path, to: &Path) {
        fs::create_dir_all(to).expect("make folder");
        for entry in fs::read_dir(from).expect("read folder") {
            let entry = entry.expect("entry");
            let target = to.join(entry.file_name());
            if entry.file_type().expect("type").is_dir() {
                copy(&entry.path(), &target);
            } else {
                fs::copy(entry.path(), target).expect("copy file");
            }
        }
    }
    let temp = tempfile::tempdir().expect("temp folder");
    copy(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/bench"),
        &temp.path().join("bench"),
    );
    temp
}

fn cases_path(temp: &tempfile::TempDir) -> PathBuf {
    temp.path().join("bench/cases.yaml")
}

/// The `jag-bench` command with no key and no setting of the model service.
fn bench_bare(home: &Path) -> Command {
    let mut command = Command::new(assert_cmd::cargo::cargo_bin!("jag-bench"));
    command
        .env("HOME", home)
        .env_remove("JAG_TYPESAFE_API_KEY")
        .env_remove("TYPESAFE_API_KEY")
        .env_remove("JAG_GATEWAY_API_KEY")
        .env_remove("JAG_BASE_URL")
        .env_remove("JAG_MODEL")
        .env_remove("JAG_TIMEOUT")
        .env_remove("JAG_RETRY_WAIT")
        .env_remove("JAG_MAX_SCORED")
        .env_remove("JAG_MAX_CHARS");
    for name in PROXY_VARIABLES {
        command.env_remove(name);
    }
    command
}

/// `jag-bench` with the made-up gateway key and the fake server.
fn bench(home: &Path, base_url: &str) -> Command {
    let mut command = bench_bare(home);
    command
        .env("JAG_GATEWAY_API_KEY", KEY)
        .env("JAG_BASE_URL", base_url);
    command
}

struct Run {
    code: i32,
    stdout: String,
    stderr: String,
}

fn run(mut command: Command) -> Run {
    let output = command.output().expect("run jag-bench");
    Run {
        code: output.status.code().expect("an exit code"),
        stdout: String::from_utf8(output.stdout).expect("UTF-8"),
        stderr: String::from_utf8(output.stderr).expect("UTF-8"),
    }
}

/// Words of the cases that no output of the tool may hold.
const CASE_WORDS: [&str; 14] = [
    "tomato",
    "water",
    "rice",
    "KEEP-",
    "garden",
    "pantry",
    "notes",
    "b_water",
    "cases.yaml",
    "keeps the longest",
    "a.b",
    "intro line",
    "closing line",
    "bench/",
];

// CRITERION-140
#[test]
fn bench_summary() {
    let server = model();
    let temp = fixtures();
    let result = run({
        let mut command = bench(temp.path(), &server.base_url());
        command.arg(cases_path(&temp));
        command
    });
    assert_eq!(result.code, 0, "stderr: {}", result.stderr);

    let lines: Vec<&str> = result.stdout.lines().collect();
    assert_eq!(lines.len(), 4, "stdout: {}", result.stdout);
    assert_eq!(lines[0], "jag-bench: 4 cases, 1 with no answer");
    assert_eq!(lines[1], "jag-bench: ndcg@10 jag 0.903, grep 0.703");
    assert_eq!(
        lines[2],
        "jag-bench: bar pass (jag 0.70 or more, and higher than grep)"
    );
    let version = env!("CARGO_PKG_VERSION");
    assert!(
        lines[3].starts_with("jag-bench: prompt jag-p1, threshold 0.4, model ")
            && lines[3].ends_with(&format!(", jag {version}")),
        "line: {}",
        lines[3]
    );

    // The progress goes to the error stream, with no text of a case.
    assert_eq!(
        result.stderr,
        "jag-bench: case 1 of 4\njag-bench: case 2 of 4\njag-bench: case 3 of 4\njag-bench: case 4 of 4\n"
    );
    let temp_name = temp.path().to_string_lossy().into_owned();
    for text in [&result.stdout, &result.stderr] {
        for word in CASE_WORDS {
            assert!(!text.contains(word), "output holds {word}");
        }
        assert!(!text.contains(&temp_name));
        assert!(!text.contains(KEY));
    }
    // The model got 3 calls: one for each case with an answer.
    assert_eq!(server.requests().len(), 3);
}

/// Run `jag --json --ask` in a folder of `temp/bench/`, then run `jag-bench`
/// on a case that labels each of the first 10 results of jag, in its order.
/// The ranks of jag-bench must be 1, 2, 3, and so on. Gives the count of
/// results that jag found.
fn check_parity(
    temp: &tempfile::TempDir,
    server: &FakeServer,
    (folder, term, question, ignore_case): (&str, &str, &str, bool),
) -> u64 {
    let folder_path = temp.path().join("bench").join(folder);
    let mut command = common::jag_std(temp.path(), &server.base_url());
    // A null device as standard input: jag then searches the current folder.
    command
        .current_dir(&folder_path)
        .stdin(std::process::Stdio::null())
        .arg("--json");
    if ignore_case {
        command.arg("-i");
    }
    command.args(["--ask", question, term]);
    let output = command.output().expect("run jag");
    let jag = Run {
        code: output.status.code().expect("an exit code"),
        stdout: String::from_utf8(output.stdout).expect("UTF-8"),
        stderr: String::from_utf8(output.stderr).expect("UTF-8"),
    };
    // Exit 7 is the cost limit: jag scored fewer results than it found.
    assert!(
        [0, 7].contains(&jag.code),
        "jag exit {}: {}",
        jag.code,
        jag.stderr
    );
    let document: Value = serde_json::from_str(&jag.stdout).expect("a JSON document");
    let results: Vec<(String, u64)> = document["results"]
        .as_array()
        .expect("results")
        .iter()
        .take(10)
        .map(|result| {
            (
                result["path"].as_str().expect("path").to_owned(),
                result["first_line"].as_u64().expect("line"),
            )
        })
        .collect();
    assert!(results.len() >= 3);
    let found = document["results_found"].as_u64().expect("results_found");

    // A case that labels each result of jag, in its order.
    let answers: Vec<Value> = results
        .iter()
        .map(|(file, line)| json!({"file": file, "line": line}))
        .collect();
    let cases = json!({"cases": [{
        "folder": folder,
        "term": term,
        "ignore_case": ignore_case,
        "question": question,
        "answers": answers,
    }]});
    let cases_file = temp.path().join("bench/parity.yaml");
    fs::write(&cases_file, cases.to_string()).expect("write cases");
    let detail = temp.path().join("detail.jsonl");
    let result = run({
        let mut command = bench(temp.path(), &server.base_url());
        command.arg("--detail").arg(&detail).arg(&cases_file);
        command
    });
    // The exit code is 0 or 1, by the bar. This test checks the ranks.
    assert!([0, 1].contains(&result.code), "stderr: {}", result.stderr);

    // jag-bench ranks the results as jag does: rank 1, 2, 3, and so on.
    let text = fs::read_to_string(&detail).expect("detail file");
    let line: Value = serde_json::from_str(text.trim()).expect("a JSON line");
    let ranks: Vec<u64> = line["answers"]
        .as_array()
        .expect("answers")
        .iter()
        .map(|answer| answer["jag_rank"].as_u64().expect("a rank"))
        .collect();
    let expected: Vec<u64> = (1..=results.len() as u64).collect();
    assert_eq!(ranks, expected, "folder {folder}");
    found
}

// CRITERION-140
#[test]
fn bench_matches_jag() {
    let server = model();
    for case in [
        (
            "garden",
            "tomato",
            "How often do I water the tomato plants?",
            false,
        ),
        ("pantry", "rice", "Which rice keeps the longest?", true),
    ] {
        check_parity(&fixtures(), &server, case);
    }
}

// CRITERION-140
#[test]
fn bench_matches_jag_after_the_pre_rank() {
    // 250 results are more than the cost limit of 200, so jag pre-ranks them
    // and scores 200. The model gives a different score to each result.
    let server = FakeServer::start(|request| {
        Reply::scores(request, |text| {
            let number: u64 = text
                .split("item ")
                .nth(1)
                .and_then(|rest| rest.split_whitespace().next())
                .and_then(|word| word.parse().ok())
                .expect("a number after item");
            0.05 + (number * 37 % 251) as f64 / 300.0
        })
    });
    let temp = fixtures();
    let big = temp.path().join("bench/big");
    for number in 0..250 {
        let line = if number % 3 == 0 {
            format!("needle item {number} needs watering every day\n")
        } else {
            format!("needle item {number} is plain\n")
        };
        common::write(&big, &format!("f{number:03}.txt"), line.as_bytes());
    }
    let found = check_parity(
        &temp,
        &server,
        ("big", "needle", "Which item needs watering?", false),
    );
    assert_eq!(found, 250);
    // The pre-rank kept 200 results: the model got 200 snippets.
    let sent: usize = server
        .requests()
        .iter()
        .map(|request| request.snippet_texts().len())
        .sum();
    assert_eq!(sent, 400, "jag and jag-bench each score 200 results");
}

// CRITERION-141
#[test]
fn bench_detail_local() {
    let server = model();
    let temp = fixtures();
    let detail = temp.path().join("detail.jsonl");
    let result = run({
        let mut command = bench(temp.path(), &server.base_url());
        command.arg("--detail").arg(&detail).arg(cases_path(&temp));
        command
    });
    assert_eq!(result.code, 0, "stderr: {}", result.stderr);
    // Standard output is the same four lines as with no detail file.
    assert_eq!(result.stdout.lines().count(), 4);

    let text = fs::read_to_string(&detail).expect("detail file");
    let lines: Vec<Value> = text
        .lines()
        .map(|line| serde_json::from_str(line).expect("a JSON line"))
        .collect();
    let expected = [
        json!({"case": 1, "jag": 1.0, "grep": 0.679, "answers": [
            {"jag_rank": 1, "grep_rank": 2}, {"jag_rank": 2, "grep_rank": 3}]}),
        json!({"case": 2, "jag": 0.95, "grep": 0.67, "answers": [
            {"jag_rank": 1, "grep_rank": 2}, {"jag_rank": 3, "grep_rank": 3}]}),
        json!({"case": 3, "jag": 0.76, "grep": 0.76, "answers": [
            {"jag_rank": 1, "grep_rank": 1}, {"jag_rank": null, "grep_rank": null}]}),
        json!({"case": 4, "jag": null, "grep": null, "answers": []}),
    ];
    assert_eq!(lines, expected);
    // The file holds no text and no path of a case.
    for word in CASE_WORDS {
        assert!(!text.contains(word), "detail holds {word}");
    }

    // A labeled line that is only a context line of a result gets no credit
    // (row 11): the match is line 2 of `b_x.txt`, the label is line 3.
    let context = temp.path().join("bench/context.yaml");
    fs::write(
        &context,
        "cases:\n  - folder: notes\n    term: a.b\n    fixed_strings: true\n    question: Where is the a.b detail?\n    answers:\n      - {file: b_x.txt, line: 3}\n",
    )
    .expect("write cases");
    let result = run({
        let mut command = bench(temp.path(), &server.base_url());
        command.arg("--detail").arg(&detail).arg(&context);
        command
    });
    // Both scores are 0, so the bar fails.
    assert_eq!(result.code, 1, "stderr: {}", result.stderr);
    let text = fs::read_to_string(&detail).expect("detail file");
    let line: Value = serde_json::from_str(text.trim()).expect("a JSON line");
    assert_eq!(
        line,
        json!({"case": 1, "jag": 0.0, "grep": 0.0, "answers": [
            {"jag_rank": null, "grep_rank": null}]})
    );

    // With no --detail, no per-case result is written anywhere.
    let other = fixtures();
    let before = fs::read_dir(other.path()).expect("read").count();
    let result = run({
        let mut command = bench(other.path(), &server.base_url());
        command.arg(cases_path(&other));
        command
    });
    assert_eq!(result.code, 0);
    assert_eq!(fs::read_dir(other.path()).expect("read").count(), before);
    assert_eq!(result.stdout.lines().count(), 4);
}

// CRITERION-141
#[test]
fn bench_no_key_in_output() {
    let server = FakeServer::start(|_| Reply::status(401));
    let temp = fixtures();
    let detail = temp.path().join("detail.jsonl");
    let result = run({
        let mut command = bench(temp.path(), &server.base_url());
        command.arg("--detail").arg(&detail).arg(cases_path(&temp));
        command
    });
    assert_eq!(result.code, 4, "stderr: {}", result.stderr);
    assert!(!result.stdout.contains(KEY));
    assert!(!result.stderr.contains(KEY));
    assert!(!detail.exists());
}

// CRITERION-141
#[test]
fn bench_guard() {
    let server = model();
    let temp = fixtures();
    let source = Path::new(env!("CARGO_MANIFEST_DIR"));

    // A detail file in the source folder.
    let inside = source.join("tests/bench/guard-detail.jsonl");
    // An old file from a failed run must not hide a write of the tool.
    let _ = fs::remove_file(&inside);
    let result = run({
        let mut command = bench(temp.path(), &server.base_url());
        command.arg("--detail").arg(&inside).arg(cases_path(&temp));
        command
    });
    assert_eq!(result.code, 2);
    assert!(
        result
            .stderr
            .contains("detail file is in the folder of the source")
    );
    assert!(result.stdout.is_empty());
    assert!(!inside.exists());

    // A case file in the source folder.
    let result = run({
        let mut command = bench(temp.path(), &server.base_url());
        command.arg(source.join("tests/bench/cases.yaml"));
        command
    });
    assert_eq!(result.code, 2);
    assert!(
        result
            .stderr
            .contains("case file is in the folder of the source")
    );
    assert!(result.stdout.is_empty());
    assert!(server.requests().is_empty());
}

// Row 15
#[test]
fn bench_bar_fail() {
    // The same score for each result: jag ranks as grep does, so it is not
    // higher than grep.
    let server = FakeServer::constant(0.5);
    let temp = fixtures();
    let result = run({
        let mut command = bench(temp.path(), &server.base_url());
        command.arg(cases_path(&temp));
        command
    });
    assert_eq!(result.code, 1, "stderr: {}", result.stderr);
    let lines: Vec<&str> = result.stdout.lines().collect();
    assert_eq!(lines.len(), 4);
    assert_eq!(
        lines[2],
        "jag-bench: bar fail (jag 0.70 or more, and higher than grep)"
    );
}

// Rows 20 and 21
#[test]
fn bench_failures() {
    let temp = fixtures();

    // No key: exit 3, before any call.
    let result = run({
        let mut command = bench_bare(temp.path());
        command.arg(cases_path(&temp));
        command
    });
    assert_eq!(result.code, 3, "stderr: {}", result.stderr);
    assert!(result.stdout.is_empty());

    // The service is down: exit 6, and no score.
    let down = {
        let server = FakeServer::constant(0.5);
        server.base_url()
    };
    let result = run({
        let mut command = bench(temp.path(), &down);
        command.env("JAG_RETRY_WAIT", "1").arg(cases_path(&temp));
        command
    });
    assert_eq!(result.code, 6, "stderr: {}", result.stderr);
    assert!(result.stdout.is_empty());
    assert!(!result.stderr.contains(KEY));

    // A bad case file: exit 2, before any call, and the text names the case
    // and the field only.
    let server = model();
    let bad = temp.path().join("bad.yaml");
    fs::write(
        &bad,
        "cases:\n  - folder: bench/garden\n    term: tomato\n    question: secret-question\n    answers:\n      - {file: b_water.txt, line: 0}\n",
    )
    .expect("write");
    let result = run({
        let mut command = bench(temp.path(), &server.base_url());
        command.arg(&bad);
        command
    });
    assert_eq!(result.code, 2, "stderr: {}", result.stderr);
    assert!(result.stderr.contains("case 1, answers.line"));
    assert!(!result.stderr.contains("secret-question"));
    assert!(!result.stderr.contains("tomato"));
    assert!(result.stdout.is_empty());
    assert!(server.requests().is_empty());
}
