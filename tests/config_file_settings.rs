//! End-to-end tests of STORY-84 (FEATURE-29): `config.timeout`,
//! `config.retry-wait`, `config.max-scored`, and `config.max-chars` in the
//! config file (CRITERION-151 and CRITERION-154). The tests run the built
//! `jag` as a user does, with a temporary `HOME` and the fake server only.
//!
//! The tests of the other files already cover the first case of each
//! setting: the file sets the value, the option and the variable win, a bad
//! value gives exit 2 with the cause and the fix, and a mode 0644 file with
//! `--json`. These tests add the cases that no other file has.

mod common;

use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Output;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use common::fake_server::{FakeServer, Reply};
use common::{jag, jag_bare, write, write_config};
use serde_json::Value;
use tempfile::TempDir;

const FIX_1000: &str =
    "To see more, use --max-chars with a number larger than 1000, or --no-max-chars.";

fn folder() -> TempDir {
    tempfile::tempdir().expect("temporary folder")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn config_path(home: &Path) -> std::path::PathBuf {
    home.join(".config/jag/config.yml")
}

/// A folder `d` with `count` files. Each file has 1 line of 100 characters
/// that holds `needle`. jag runs in the folder with the path `d`.
fn matches(count: usize) -> TempDir {
    let root = folder();
    for number in 0..count {
        let head = format!("needle {number:02} ");
        let line = format!("{head}{}\n", "x".repeat(100 - head.len()));
        write(root.path(), &format!("d/r{number:02}.txt"), line.as_bytes());
    }
    root
}

/// Run `jag [extra] --ask q needle d` in `root`, with this text in the config
/// file of a new home, and these variables.
fn ask(
    server: &FakeServer,
    root: &Path,
    config: &str,
    extra: &[&str],
    environment: &[(&str, &str)],
) -> Output {
    let home = folder();
    write_config(home.path(), config);
    let mut command = jag(home.path(), &server.base_url());
    for (name, value) in environment {
        command.env(name, value);
    }
    command
        .current_dir(root)
        .args(extra)
        .args(["--ask", "q", "needle", "d"])
        .output()
        .expect("run jag")
}

fn slow_server() -> FakeServer {
    FakeServer::start(|request| Reply::scores(request, |_| 0.8).after(Duration::from_secs(3)))
}

/// A server that gives 500 to the first call, and scores to the next. It
/// keeps the time of each call.
fn failing_once_server() -> (FakeServer, Arc<Mutex<Vec<Instant>>>) {
    let times = Arc::new(Mutex::new(Vec::new()));
    let seen = Arc::clone(&times);
    let server = FakeServer::start(move |request| {
        let mut times = seen.lock().expect("lock");
        times.push(Instant::now());
        if times.len() == 1 {
            Reply::status(500)
        } else {
            Reply::scores(request, |_| 0.8)
        }
    });
    (server, times)
}

fn gap(times: &Arc<Mutex<Vec<Instant>>>) -> Duration {
    let times = times.lock().expect("lock");
    assert_eq!(times.len(), 2, "two calls");
    times[1] - times[0]
}

// CRITERION-151: a blank variable does not hide the file.
#[test]
fn a_blank_timeout_variable_does_not_hide_the_file() {
    let server = slow_server();
    let root = matches(1);
    let started = Instant::now();
    let output = ask(
        &server,
        root.path(),
        "config:\n  timeout: 1\n",
        &[],
        &[("JAG_TIMEOUT", "")],
    );
    assert_eq!(output.status.code(), Some(6), "{}", stderr(&output));
    assert!(stdout(&output).is_empty());
    assert_eq!(server.requests().len(), 2);
    // 1 s, the 1 s wait, and 1 s, with room for a slow machine.
    assert!(started.elapsed() < Duration::from_secs(5));
}

// CRITERION-154: the file sets the wait of 3 s, and the variable wins.
#[test]
fn the_retry_wait_variable_wins_over_a_file_wait_of_three_seconds() {
    let file = "config:\n  retry-wait: 3\n";
    let root = matches(1);

    let (server, times) = failing_once_server();
    let output = ask(&server, root.path(), file, &[], &[]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(gap(&times) >= Duration::from_secs(3));

    let (server, times) = failing_once_server();
    let output = ask(&server, root.path(), file, &[], &[("JAG_RETRY_WAIT", "1")]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(gap(&times) < Duration::from_secs(3));
}

// CRITERION-154: the notice of exit 7 names the limit of the file, and the
// variable wins.
#[test]
fn the_cost_notice_names_the_limit_of_the_file_and_the_variable_wins() {
    let root = matches(30);
    let file = "config:\n  max-scored: 10\n";
    let server = FakeServer::constant(0.8);
    let output = ask(&server, root.path(), file, &[], &[]);
    assert_eq!(output.status.code(), Some(7), "{}", stderr(&output));
    assert!(
        stderr(&output).contains("scored 10 of 30 results, skipped 20"),
        "{}",
        stderr(&output)
    );
    let server = FakeServer::constant(0.8);
    let output = ask(&server, root.path(), file, &[], &[("JAG_MAX_SCORED", "20")]);
    assert_eq!(output.status.code(), Some(7), "{}", stderr(&output));
    assert!(
        stderr(&output).contains("scored 20 of 30 results, skipped 10"),
        "{}",
        stderr(&output)
    );
}

// CRITERION-154: the size notice names the limit of the file, and
// `--no-max-chars` gives no notice.
#[test]
fn the_size_notice_names_the_limit_of_the_file() {
    let root = matches(30);
    let file = "config:\n  max-chars: 1000\n";
    let server = FakeServer::constant(0.8);
    let output = ask(&server, root.path(), file, &[], &[]);
    assert_eq!(output.status.code(), Some(8), "{}", stderr(&output));
    assert!(stdout(&output).chars().count() <= 1000);
    assert!(stderr(&output).contains(FIX_1000), "{}", stderr(&output));

    let server = FakeServer::constant(0.8);
    let output = ask(&server, root.path(), file, &["--no-max-chars"], &[]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(
        !stderr(&output).contains("size limit"),
        "{}",
        stderr(&output)
    );
}

// Decision 9: `--json` with no question uses the limit of the file, and the
// document names the limit.
#[test]
fn json_with_no_question_names_the_limit_of_the_file() {
    let root = matches(30);
    let home = folder();
    write_config(home.path(), "config:\n  max-chars: 1000\n");
    let output = jag_bare(home.path())
        .current_dir(root.path())
        .args(["--json", "needle", "d"])
        .output()
        .expect("run jag");
    assert_eq!(output.status.code(), Some(8), "{}", stderr(&output));
    let document: Value = serde_json::from_str(&stdout(&output)).expect("one document");
    assert!(document["left_out"].as_u64().unwrap_or(0) > 0);
    assert_eq!(document["size_limit_fix"], FIX_1000);
}

// A value that a higher source hides does not stop the search.
#[test]
fn a_higher_source_hides_a_bad_value_of_the_file_for_each_key() {
    type Case<'a> = (&'a str, &'a [&'a str], &'a [(&'a str, &'a str)]);
    let cases: [Case; 4] = [
        ("timeout", &[], &[("JAG_TIMEOUT", "4")]),
        ("retry-wait", &["--retry-wait", "1"], &[]),
        ("max-scored", &[], &[("JAG_MAX_SCORED", "10")]),
        ("max-chars", &["--no-max-chars"], &[]),
    ];
    for (key, extra, environment) in cases {
        let server = FakeServer::constant(0.8);
        let root = matches(1);
        let output = ask(
            &server,
            root.path(),
            &format!("config:\n  {key}: abc\n"),
            extra,
            environment,
        );
        assert_eq!(output.status.code(), Some(0), "{key}: {}", stderr(&output));
    }
}

// A search with no question and no `--json` does not read the file, so a bad
// value is not an error.
#[test]
fn a_search_with_no_question_ignores_bad_values_in_the_file() {
    let root = matches(1);
    let home = folder();
    write_config(
        home.path(),
        "config:\n  timeout: abc\n  retry-wait: abc\n  max-scored: abc\n  max-chars: abc\n",
    );
    let output = jag_bare(home.path())
        .current_dir(root.path())
        .args(["needle", "d"])
        .output()
        .expect("run jag");
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(stdout(&output).contains("needle"));
}

// CRITERION-154: `--forget-key` keeps the 4 keys and the mode, and the next
// search uses the limit of the file.
#[test]
fn forget_key_keeps_the_four_keys_and_the_next_search_uses_them() {
    let home = folder();
    write_config(
        home.path(),
        "config:\n  gateway-api-key: SENTINEL-SENTINEL-SENTINEL\n  timeout: 9\n  \
         retry-wait: 3\n  max-scored: 50\n  max-chars: 1000\n",
    );
    let output = jag_bare(home.path())
        .env("JAG_BASE_URL", "https://openrouter.ai/api")
        .arg("--forget-key")
        .output()
        .expect("run jag");
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    let path = config_path(home.path());
    let text = std::fs::read_to_string(&path).expect("the file stays");
    assert!(!text.contains("gateway-api-key") && !text.contains("SENTINEL"));
    for (key, value) in [
        ("timeout", "9"),
        ("retry-wait", "3"),
        ("max-scored", "50"),
        ("max-chars", "1000"),
    ] {
        // The write can put the number in quotes, so look for the line.
        let line = text
            .lines()
            .find(|line| line.trim_start().starts_with(&format!("{key}:")))
            .unwrap_or_else(|| panic!("{key} stays: {text}"));
        assert!(line.contains(value), "{line}");
    }
    let mode = std::fs::metadata(&path)
        .expect("metadata")
        .permissions()
        .mode();
    assert_eq!(mode & 0o777, 0o600);

    // The next search uses `max-chars` of the file.
    let root = matches(30);
    let server = FakeServer::constant(0.8);
    let output = jag(home.path(), &server.base_url())
        .current_dir(root.path())
        .args(["--ask", "q", "needle", "d"])
        .output()
        .expect("run jag");
    assert_eq!(output.status.code(), Some(8), "{}", stderr(&output));
    assert!(stdout(&output).chars().count() <= 1000);
    assert!(stderr(&output).contains(FIX_1000), "{}", stderr(&output));
}

// Row 7: a bad time in the file stops `--check` with exit 2, the cause, and
// no item line.
#[test]
fn a_bad_timeout_in_the_file_stops_the_check_with_exit_2_and_no_item_line() {
    let server = FakeServer::constant(0.8);
    let home = folder();
    write_config(home.path(), "config:\n  timeout: abc\n");
    let output = jag(home.path(), &server.base_url())
        .env("JAG_MODEL", "jev-1.13.0")
        .arg("--check")
        .output()
        .expect("run jag");
    assert_eq!(output.status.code(), Some(2), "{}", stderr(&output));
    assert!(stdout(&output).is_empty(), "{}", stdout(&output));
    let cause = format!(
        "jag: config.timeout in {} is not a whole number of seconds from 1 to 60.\n",
        config_path(home.path()).display()
    );
    assert!(stderr(&output).starts_with(&cause), "{}", stderr(&output));
    assert!(!stderr(&output).contains("timeout: abc"));
    assert!(server.requests().is_empty());
}
