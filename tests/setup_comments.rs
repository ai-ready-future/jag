//! End-to-end tests of the comment lines that `--setup` writes and that
//! `--setup` and `--forget-key` keep (STORY-85, CRITERION-152 and
//! CRITERION-153, rows 15, 18, 20 and 22 of the design of STORY-84). They run
//! the built `jag` as a user does, with a temporary `HOME`, made-up keys, and
//! no model server.
//!
//! `--setup` checks that standard input is a terminal (CRITERION-72), so
//! these tests give jag a pseudo terminal as standard input and answer "n" to
//! the question "Replace it?". The test cannot type a new key: jag reads the
//! key from `/dev/tty`, and that needs a controlling terminal, which the code
//! of a test cannot make (the crate forbids `unsafe`). The tests that store a
//! new key, on a new file and on a gateway route, are the unit tests in
//! `src/config/setup.rs`.
//!
//! Not repeated here, because a test already runs the built `jag`:
//! - `forget_key_keeps_each_comment_line` in `tests/setup.rs` (a key line, 6
//!   comment lines, and a file with only the key line).
//! - `a_file_in_another_form_stops_forget_key_with_exit_2` in `tests/setup.rs`
//!   (the form `config: {...}` with `--forget-key`).
//! - `forget_key_keeps_the_four_keys_and_the_next_search_uses_them` in
//!   `tests/config_file_settings.rs` (the values stay).

mod common;

use std::fs::File;
use std::io::{Read, Write};
use std::os::fd::OwnedFd;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::thread;
use std::time::{Duration, Instant};

use common::{KEY, jag_bare, jag_std, write, write_config};
use rustix::pty::{OpenptFlags, grantpt, openpt, ptsname, unlockpt};
use tempfile::TempDir;

/// The comment lines of a new file: one for each setting, with its default.
const COMMENTS: &str = "  # base-url: https://api.typesafe.ai/v1\n  # model: jev-1.13.0\n  # timeout: 4\n  # retry-wait: 1\n  # max-scored: 200\n  # max-chars: 20000\n";

/// The text of the error "Errors" in the design of STORY-84.
const NOT_EDITABLE: &str = "not in the form that jag writes";

/// The time limit of one run of jag.
const TIME_LIMIT: Duration = Duration::from_secs(30);

struct Run {
    code: Option<i32>,
    stdout: String,
    stderr: String,
}

fn config_path(home: &Path) -> PathBuf {
    home.join(".config/jag/config.yml")
}

fn read_config(home: &Path) -> String {
    std::fs::read_to_string(config_path(home)).expect("read the config file")
}

fn mode_of(path: &Path) -> u32 {
    std::fs::metadata(path)
        .expect("metadata")
        .permissions()
        .mode()
        & 0o777
}

/// The names of the files in the folder of the config file.
fn names_in_jag_folder(home: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(config_path(home).parent().expect("folder"))
        .expect("read the folder")
        .map(|entry| {
            entry
                .expect("entry")
                .file_name()
                .into_string()
                .expect("name")
        })
        .collect();
    names.sort();
    names
}

/// Read a stream of the child in a thread until its end.
fn read_all(mut source: impl Read + Send + 'static) -> thread::JoinHandle<String> {
    thread::spawn(move || {
        let mut data = Vec::new();
        source.read_to_end(&mut data).expect("read the stream");
        String::from_utf8_lossy(&data).into_owned()
    })
}

/// Run `jag ARGS` with a pseudo terminal as standard input. Standard output
/// and standard error are pipes. `answers` is what the user types, for
/// example `n\n`. `env` adds variables.
fn run_in_terminal(home: &Path, args: &[&str], env: &[(&str, &str)], answers: &str) -> Run {
    // Only Linux can set close-on-exec in `openpt`. On macOS, set it after.
    #[cfg(target_os = "linux")]
    let master = openpt(OpenptFlags::RDWR | OpenptFlags::NOCTTY | OpenptFlags::CLOEXEC)
        .expect("open a pty master");
    #[cfg(not(target_os = "linux"))]
    let master = {
        let master = openpt(OpenptFlags::RDWR | OpenptFlags::NOCTTY).expect("open a pty master");
        rustix::io::fcntl_setfd(&master, rustix::io::FdFlags::CLOEXEC)
            .expect("set close-on-exec on the pty master");
        master
    };
    grantpt(&master).expect("grant the pty");
    unlockpt(&master).expect("unlock the pty");
    let name = ptsname(&master, Vec::new()).expect("get the pty name");
    let slave: OwnedFd = rustix::fs::open(
        name.as_c_str(),
        rustix::fs::OFlags::RDWR | rustix::fs::OFlags::NOCTTY | rustix::fs::OFlags::CLOEXEC,
        rustix::fs::Mode::empty(),
    )
    .expect("open the pty slave");

    let mut command = jag_std(home, "http://127.0.0.1:9");
    command
        .env_remove("JAG_GATEWAY_API_KEY")
        .env_remove("JAG_BASE_URL")
        .args(args)
        .stdin(Stdio::from(slave.try_clone().expect("copy the slave")))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (name, value) in env {
        command.env(name, value);
    }
    let mut child = command.spawn().expect("start jag");
    drop(command);

    // The terminal keeps the typed line until jag reads it.
    let mut master = File::from(master);
    master.write_all(answers.as_bytes()).expect("type");
    let stdout = read_all(child.stdout.take().expect("stdout"));
    let stderr = read_all(child.stderr.take().expect("stderr"));
    let start = Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait().expect("wait") {
            break status;
        }
        if start.elapsed() > TIME_LIMIT {
            let _ = child.kill();
            let _ = child.wait();
            panic!("jag ran for more than {} s", TIME_LIMIT.as_secs());
        }
        thread::sleep(Duration::from_millis(5));
    };
    // The test keeps the terminal open until jag has ended.
    drop(slave);
    drop(master);
    Run {
        code: status.code(),
        stdout: stdout.join().expect("stdout reader"),
        stderr: stderr.join().expect("stderr reader"),
    }
}

/// Run `jag ARGS` with no terminal.
fn run_plain(home: &Path, args: &[&str], env: &[(&str, &str)], cwd: Option<&Path>) -> Run {
    let mut command = jag_bare(home);
    command.args(args);
    for (name, value) in env {
        command.env(name, value);
    }
    if let Some(cwd) = cwd {
        command.current_dir(cwd);
    }
    let output = command.output().expect("run jag");
    Run {
        code: output.status.code(),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    }
}

fn home_with(text: &str) -> TempDir {
    let home = tempfile::tempdir().expect("home");
    write_config(home.path(), text);
    home
}

fn crlf(text: &str) -> String {
    text.replace('\n', "\r\n")
}

// CRITERION-152, row 15: the answer "no" keeps the key and adds the comment
// lines.
#[test]
fn setup_keeps_the_key_and_adds_the_six_comment_lines() {
    let home = home_with(&format!("config:\n  api-key: {KEY}\n"));
    let run = run_in_terminal(home.path(), &["--setup"], &[], "n\n");
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert!(
        run.stdout.contains("Kept the stored key."),
        "{}",
        run.stdout
    );
    assert_eq!(
        read_config(home.path()),
        format!("config:\n  api-key: {KEY}\n{COMMENTS}")
    );
    assert_eq!(mode_of(&config_path(home.path())), 0o600);
    assert_eq!(names_in_jag_folder(home.path()), ["config.yml"]);
    assert!(!run.stdout.contains(KEY) && !run.stderr.contains(KEY));
}

// CRITERION-153: a second run changes nothing, and jag does not write.
#[test]
fn a_second_setup_gives_the_same_bytes_and_writes_nothing() {
    let home = home_with(&format!("config:\n  api-key: {KEY}\n"));
    run_in_terminal(home.path(), &["--setup"], &[], "n\n");
    let path = config_path(home.path());
    let bytes = std::fs::read(&path).expect("read");
    let before = std::fs::metadata(&path).expect("metadata");
    let run = run_in_terminal(home.path(), &["--setup"], &[], "n\n");
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert!(
        run.stdout.contains("Kept the stored key."),
        "{}",
        run.stdout
    );
    let after = std::fs::metadata(&path).expect("metadata");
    assert_eq!(std::fs::read(&path).expect("read"), bytes);
    // A write makes a new file, so the inode and the time would change.
    assert_eq!(after.ino(), before.ino());
    assert_eq!(after.modified().unwrap(), before.modified().unwrap());
}

// CRITERION-152: a value, a comment and a comment line of the user stay, and
// the answer "yes" is not needed to add the missing lines.
#[test]
fn setup_keeps_a_value_and_a_comment_of_the_user() {
    let home = home_with(&format!(
        "# my notes\nconfig:\n  api-key: {KEY}\n  # base-url: https://api.typesafe.ai/v1\n  timeout: 9   # my value\n"
    ));
    let run = run_in_terminal(home.path(), &["--setup"], &[], "n\n");
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    let text = read_config(home.path());
    assert_eq!(
        text,
        format!(
            "# my notes\nconfig:\n  api-key: {KEY}\n  # base-url: https://api.typesafe.ai/v1\n  timeout: 9   # my value\n  # model: jev-1.13.0\n  # retry-wait: 1\n  # max-scored: 200\n  # max-chars: 20000\n"
        )
    );
}

// Row 14: a comment line with another default stays, and no line with the
// default of jag is added for that setting.
#[test]
fn a_comment_line_with_another_default_stays() {
    let home = home_with(&format!("config:\n  api-key: {KEY}\n  # timeout: 3\n"));
    let run = run_in_terminal(home.path(), &["--setup"], &[], "n\n");
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    let text = read_config(home.path());
    assert!(text.contains("  # timeout: 3\n"), "{text}");
    assert!(!text.contains("# timeout: 4"), "{text}");
    assert_eq!(text.matches("# timeout").count(), 1, "{text}");
    for key in ["base-url", "model", "retry-wait", "max-scored", "max-chars"] {
        assert_eq!(
            text.matches(&format!("# {key}:")).count(),
            1,
            "{key}: {text}"
        );
    }
}

// CRITERION-152, gateway route: the stored gateway key stays, and the
// comment lines are added to the same file.
#[test]
fn setup_on_the_gateway_route_keeps_the_gateway_key_and_adds_the_lines() {
    let home = home_with(&format!("config:\n  gateway-api-key: {KEY}\n"));
    let run = run_in_terminal(
        home.path(),
        &["--setup"],
        &[("JAG_BASE_URL", "https://openrouter.ai/api")],
        "n\n",
    );
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert!(
        run.stdout.contains("Kept the stored gateway key."),
        "{}",
        run.stdout
    );
    assert_eq!(
        read_config(home.path()),
        format!("config:\n  gateway-api-key: {KEY}\n{COMMENTS}")
    );
}

// Row 18, row 22: a file in another form stops setup with exit 2 before the
// question. The file stays, no temporary file stays, and no output holds the
// key.
#[test]
fn setup_stops_on_a_file_in_another_form_and_leaves_it_as_it_is() {
    let forms = [
        format!("config: {{api-key: {KEY}}}\n"),
        format!("config:\n  api-key: |\n    {KEY}\n"),
        "config: null\n".to_owned(),
        format!("config:\n  api-key: {KEY}\n  model: \"a\n    b\"\n"),
        format!("config:\n  api-key: {KEY}\n...\n"),
    ];
    for text in forms {
        let home = home_with(&text);
        let run = run_in_terminal(home.path(), &["--setup"], &[], "y\n");
        assert_eq!(run.code, Some(2), "{text:?}: {}", run.stderr);
        assert!(
            run.stderr.contains(NOT_EDITABLE),
            "{text:?}: {}",
            run.stderr
        );
        assert!(
            !run.stderr.contains("Replace it?") && !run.stderr.contains("API key:"),
            "no question: {}",
            run.stderr
        );
        assert!(run.stdout.is_empty(), "{}", run.stdout);
        assert!(!run.stdout.contains(KEY) && !run.stderr.contains(KEY));
        assert_eq!(read_config(home.path()), text);
        assert_eq!(names_in_jag_folder(home.path()), ["config.yml"]);
    }
}

// Row 18: `--forget-key` stops when it must edit a value that jag cannot edit.
#[test]
fn forget_key_stops_when_another_value_is_in_a_form_jag_cannot_edit() {
    let text = format!("config:\n  api-key: {KEY}\n  model: |\n    m\n");
    let home = home_with(&text);
    let run = run_plain(home.path(), &["--forget-key"], &[], None);
    assert_eq!(run.code, Some(2), "{}", run.stderr);
    assert!(run.stderr.contains(NOT_EDITABLE), "{}", run.stderr);
    assert!(!run.stdout.contains(KEY) && !run.stderr.contains(KEY));
    assert_eq!(read_config(home.path()), text);
    assert_eq!(names_in_jag_folder(home.path()), ["config.yml"]);
}

// Row 18, exception: when the key is the only value, `--forget-key` removes
// the file, also when the file is in another form.
#[test]
fn forget_key_removes_a_file_in_another_form_when_only_the_key_is_in_it() {
    for text in [
        format!("config: {{api-key: {KEY}}}\n"),
        format!("config:\n  api-key: {KEY}\n...\n"),
    ] {
        let home = home_with(&text);
        let run = run_plain(home.path(), &["--forget-key"], &[], None);
        assert_eq!(run.code, Some(0), "{text:?}: {}", run.stderr);
        assert!(!run.stdout.contains(KEY) && !run.stderr.contains(KEY));
        assert!(!config_path(home.path()).exists(), "{text:?}");
    }
}

// Row 20: a file with CRLF line ends keeps them, also on each new line.
#[test]
fn setup_and_forget_key_keep_the_crlf_line_ends() {
    let home = home_with(&crlf(&format!("config:\n  api-key: {KEY}\n")));
    let run = run_in_terminal(home.path(), &["--setup"], &[], "n\n");
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    let bytes = std::fs::read(config_path(home.path())).expect("read");
    assert_eq!(
        String::from_utf8(bytes).expect("text"),
        crlf(&format!("config:\n  api-key: {KEY}\n{COMMENTS}"))
    );
    // A second run adds nothing.
    let before = read_config(home.path());
    run_in_terminal(home.path(), &["--setup"], &[], "n\n");
    assert_eq!(read_config(home.path()), before);
    // `--forget-key` removes the key line, and each other line stays.
    let run = run_plain(home.path(), &["--forget-key"], &[], None);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert_eq!(
        read_config(home.path()),
        crlf(&format!("config:\n{COMMENTS}"))
    );

    let home = home_with(&crlf(&format!("config:\n  api-key: {KEY}\n  model: m\n")));
    let run = run_plain(home.path(), &["--forget-key"], &[], None);
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert_eq!(read_config(home.path()), crlf("config:\n  model: m\n"));
}

// Row 20: a last line with no line end gets one before the new lines.
#[test]
fn setup_adds_a_line_end_before_the_new_lines() {
    let home = home_with(&format!("config:\n  api-key: {KEY}"));
    let run = run_in_terminal(home.path(), &["--setup"], &[], "n\n");
    assert_eq!(run.code, Some(0), "{}", run.stderr);
    assert_eq!(
        read_config(home.path()),
        format!("config:\n  api-key: {KEY}\n{COMMENTS}")
    );
}

/// A folder `d` with one file that holds `needle`, for a search that asks
/// no question and calls no model.
fn needle_folder() -> TempDir {
    let root = tempfile::tempdir().expect("folder");
    write(root.path(), "d/a.txt", b"needle\n");
    root
}

// Scope 4: a comment line is not a value. `max-chars: abc` is a bad value
// (exit 2), and `# max-chars: abc` is a comment (exit 0).
#[test]
fn a_comment_line_is_not_a_value_for_the_read_path() {
    let root = needle_folder();
    let home = home_with("config:\n  # max-chars: abc\n");
    let run = run_plain(
        home.path(),
        &["--json", "needle", "d"],
        &[],
        Some(root.path()),
    );
    assert_eq!(run.code, Some(0), "{}", run.stderr);

    let home = home_with("config:\n  max-chars: abc\n");
    let run = run_plain(
        home.path(),
        &["--json", "needle", "d"],
        &[],
        Some(root.path()),
    );
    // With `--json`, the error is a field of the document on standard output.
    assert_eq!(run.code, Some(2), "{}", run.stdout);
    assert!(run.stdout.contains("config.max-chars"), "{}", run.stdout);
}
