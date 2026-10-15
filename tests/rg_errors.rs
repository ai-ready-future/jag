//! End-to-end tests of two ends of a search with no question that depend on
//! the system: a file that jag cannot read, and Ctrl-C (FEATURE-23,
//! CRITERION-114). The texts and the exit codes are those of rg 15.2.0.

mod common;

use std::fs::{OpenOptions, Permissions};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use common::{jag_bare, jag_std, write};

const SIGINT: i32 = 2;

/// jag cannot read a file with the mode 000: rg writes the path and the
/// reason on the error stream, searches the other files, and exits 2.
#[test]
fn a_file_that_jag_cannot_read_gives_the_error_text_of_rg_and_exit_2() {
    let root = tempfile::tempdir().expect("folder");
    write(root.path(), "t.txt", b"needle\n");
    write(root.path(), "u.txt", b"needle\n");
    let locked = root.path().join("u.txt");
    std::fs::set_permissions(&locked, Permissions::from_mode(0o000)).expect("chmod");
    // A user who can read each file (root) cannot make this test.
    if std::fs::read(&locked).is_ok() {
        return;
    }
    let home = tempfile::tempdir().expect("home");
    let output = jag_bare(home.path())
        .current_dir(root.path())
        .args(["--sort", "path", "needle", "t.txt", "u.txt"])
        .output()
        .expect("run jag");
    std::fs::set_permissions(&locked, Permissions::from_mode(0o644)).expect("chmod");
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(String::from_utf8_lossy(&output.stdout), "t.txt:needle\n");
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "jag: u.txt: Permission denied (os error 13)\n"
    );
}

/// jag cannot read a folder with the mode 000. A walk error has two forms
/// (decision 11): rg uses the long one `P: IO error for operation on P: ...`
/// with one thread (`--sort`, or one file, or one CPU), and the short one
/// `P: ...` with more than one thread.
#[test]
fn a_folder_that_jag_cannot_read_has_the_two_forms_of_rg() {
    let root = tempfile::tempdir().expect("folder");
    write(root.path(), "t.txt", b"needle\n");
    write(root.path(), "sub/x.txt", b"needle\n");
    let locked = root.path().join("sub");
    std::fs::set_permissions(&locked, Permissions::from_mode(0o000)).expect("chmod");
    // A user who can read each folder (root) cannot make this test.
    if std::fs::read_dir(&locked).is_ok() {
        std::fs::set_permissions(&locked, Permissions::from_mode(0o755)).expect("chmod");
        return;
    }
    let one_cpu = std::thread::available_parallelism().map_or(true, |count| count.get() == 1);
    let run = |args: &[&str]| {
        let home = tempfile::tempdir().expect("home");
        let output = jag_bare(home.path())
            .current_dir(root.path())
            .args(args)
            .output()
            .expect("run jag");
        (
            output.status.code(),
            String::from_utf8_lossy(&output.stdout).into_owned(),
            String::from_utf8_lossy(&output.stderr).into_owned(),
        )
    };
    let long = |path: &str| {
        format!("jag: {path}: IO error for operation on {path}: Permission denied (os error 13)\n")
    };
    let short = |path: &str| format!("jag: {path}: Permission denied (os error 13)\n");
    let more_threads = |path: &str| if one_cpu { long(path) } else { short(path) };

    // The folder is the path: one folder, so more threads, unless --sort.
    let (code, out, err) = run(&["needle", "sub"]);
    assert_eq!((code, out.as_str()), (Some(2), ""));
    assert_eq!(err, more_threads("sub"));
    let (code, out, err) = run(&["--sort", "path", "needle", "sub"]);
    assert_eq!((code, out.as_str()), (Some(2), ""));
    assert_eq!(err, long("sub"));
    // The last --sort decides, and `--sort none` is no sort.
    let (_, _, err) = run(&["--sort", "path", "--sort", "none", "needle", "sub"]);
    assert_eq!(err, more_threads("sub"));
    let (_, _, err) = run(&["--sort", "none", "--sort", "path", "needle", "sub"]);
    assert_eq!(err, long("sub"));

    // The folder is inside a walk.
    let (code, out, err) = run(&["needle", "."]);
    assert_eq!((code, out.as_str()), (Some(2), "./t.txt:needle\n"));
    assert_eq!(err, more_threads("./sub"));
    let (code, out, err) = run(&["--sort", "path", "needle", "."]);
    assert_eq!((code, out.as_str()), (Some(2), "./t.txt:needle\n"));
    assert_eq!(err, long("./sub"));

    // A path that does not exist: one file is one thread, two paths are not.
    let (code, _, err) = run(&["needle", "nope.txt"]);
    assert_eq!(code, Some(2));
    assert_eq!(
        err,
        "jag: nope.txt: IO error for operation on nope.txt: No such file or directory (os error 2)\n"
    );
    let (code, out, err) = run(&["needle", "nope.txt", "t.txt"]);
    assert_eq!((code, out.as_str()), (Some(2), "t.txt:needle\n"));
    let long_missing = "jag: nope.txt: IO error for operation on nope.txt: No such file or directory (os error 2)\n";
    let short_missing = "jag: nope.txt: No such file or directory (os error 2)\n";
    assert_eq!(err, if one_cpu { long_missing } else { short_missing });
    let (_, _, err) = run(&["--sort", "none", "needle", "nope.txt", "t.txt"]);
    assert_eq!(err, if one_cpu { long_missing } else { short_missing });
    let (_, _, err) = run(&["--sort", "path", "needle", "nope.txt", "t.txt"]);
    assert_eq!(err, long_missing);

    std::fs::set_permissions(&locked, Permissions::from_mode(0o755)).expect("chmod");
}

/// Ctrl-C ends a search that waits for more text. The end is by the signal
/// SIGINT (a shell shows the code 130), as in FEATURE-15, decision 13.
#[test]
fn ctrl_c_ends_a_search_with_no_question_with_the_signal_sigint() {
    let root = tempfile::tempdir().expect("folder");
    let fifo = root.path().join("fifo");
    let made = Command::new("mkfifo").arg(&fifo).status().expect("mkfifo");
    assert!(made.success());
    // jag reads the fifo, and the test keeps it open: the text never ends.
    let writer = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&fifo)
        .expect("open the fifo");
    let home = tempfile::tempdir().expect("home");
    let mut child = jag_std(home.path(), "https://example.invalid")
        .current_dir(root.path())
        .args(["needle", "fifo"])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("start jag");
    // Give jag the time to start its search.
    std::thread::sleep(Duration::from_millis(500));
    assert!(child.try_wait().expect("wait").is_none(), "jag has ended");
    let sent = Command::new("kill")
        .args(["-INT", &child.id().to_string()])
        .status()
        .expect("run kill");
    assert!(sent.success());
    let started = Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait().expect("wait") {
            break status;
        }
        if started.elapsed() > Duration::from_secs(5) {
            let _ = child.kill();
            panic!("jag did not stop in 5 seconds");
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    drop(writer);
    assert!(
        status.code() == Some(130) || status.signal() == Some(SIGINT),
        "{status:?}"
    );
    let output = child.wait_with_output().expect("output");
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
}
