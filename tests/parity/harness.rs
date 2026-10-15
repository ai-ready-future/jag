//! The shared part of the rg parity tests (ADR-0021): the run of a program in
//! one of three modes, the pseudo terminal, the name rule, and the comparison.
//!
//! `tests/rg_parity.rs` (the cases of `cases.yaml`) and `tests/rg_parity_fs.rs`
//! (the cases that need a file system set-up) include this file with
//! `#[path]`. Each of them uses a part of it.

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::io::{self, Read};
use std::os::fd::OwnedFd;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use rustix::fs::{Mode, OFlags};
use rustix::pty::{OpenptFlags, grantpt, openpt, ptsname, unlockpt};
use rustix::termios::{Winsize, tcsetwinsize};
use serde::Deserialize;

/// The time limit of one program run.
const TIME_LIMIT: Duration = Duration::from_secs(30);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RunMode {
    Pipe,
    Color,
    Terminal,
}

impl RunMode {
    pub const ALL: [RunMode; 3] = [RunMode::Pipe, RunMode::Color, RunMode::Terminal];

    pub fn name(self) -> &'static str {
        match self {
            RunMode::Pipe => "pipe",
            RunMode::Color => "color",
            RunMode::Terminal => "terminal",
        }
    }
}

/// What one program run gave.
#[derive(Debug, PartialEq, Eq)]
pub struct Run {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub code: i32,
}

/// What a program runs with. `env` adds a variable to the fixed environment
/// (a value of `None` removes a variable of the fixed environment, for example
/// `TERM`). `stdin` names a file in `tests/parity/stdin`.
pub struct Invocation<'a> {
    pub args: &'a [String],
    pub stdin: Option<&'a str>,
    pub env: &'a BTreeMap<String, Option<String>>,
}

pub fn parity_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/parity")
}

/// The version of the pin: the first line of `rg.sha256` names an archive
/// `ripgrep-<version>-<target>.tar.gz`.
pub fn pinned_version() -> String {
    let path = parity_dir().join("rg.sha256");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let archive = text
        .lines()
        .find(|line| !line.trim().is_empty())
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or_else(|| panic!("{} has no archive line", path.display()));
    let rest = archive
        .strip_prefix("ripgrep-")
        .unwrap_or_else(|| panic!("the archive name {archive} does not start with ripgrep-"));
    rest.split('-').next().unwrap_or(rest).to_string()
}

/// The files of a fixture tree, as paths relative to the tree.
pub fn tree_files(root: &Path) -> Vec<PathBuf> {
    fn walk(root: &Path, dir: &Path, out: &mut Vec<PathBuf>) {
        let mut entries: Vec<_> = fs::read_dir(dir)
            .unwrap_or_else(|e| panic!("read {}: {e}", dir.display()))
            .map(|entry| entry.expect("read a folder entry").path())
            .collect();
        entries.sort();
        for path in entries {
            if path.is_dir() {
                walk(root, &path, out);
            } else {
                out.push(
                    path.strip_prefix(root)
                        .expect("inside the tree")
                        .to_path_buf(),
                );
            }
        }
    }
    let mut out = Vec::new();
    walk(root, root, &mut out);
    out
}

/// The real name of a stored name: a name that starts with `dot-` gets `.` in
/// its place (ADR-0021).
fn real_name(stored: &OsStr) -> OsString {
    let text = stored.to_string_lossy();
    match text.strip_prefix("dot-") {
        Some(rest) => format!(".{rest}").into(),
        None => stored.to_os_string(),
    }
}

/// Copy a fixture tree into a new folder. Git cannot store a path with a
/// `.git` part and a `.gitignore` would change what git stores, so a stored
/// name has `dot-` in place of `.`.
pub fn copy_tree(from: &Path, to: &Path) {
    for relative in tree_files(from) {
        let target: PathBuf = relative.iter().map(real_name).collect();
        let target = to.join(target);
        fs::create_dir_all(target.parent().expect("a file has a parent")).expect("make a folder");
        fs::copy(from.join(&relative), &target)
            .unwrap_or_else(|e| panic!("copy {}: {e}", relative.display()));
    }
}

/// Make a pseudo terminal with a fixed size and the default settings. The
/// first file is the master side, and the second file is the slave side.
fn open_pty() -> (OwnedFd, OwnedFd) {
    let master = openpt(OpenptFlags::RDWR | OpenptFlags::NOCTTY | OpenptFlags::CLOEXEC)
        .expect("open a pty master");
    grantpt(&master).expect("grant the pty");
    unlockpt(&master).expect("unlock the pty");
    let name = ptsname(&master, Vec::new()).expect("get the pty name");
    let slave = rustix::fs::open(
        name.as_c_str(),
        OFlags::RDWR | OFlags::NOCTTY | OFlags::CLOEXEC,
        Mode::empty(),
    )
    .expect("open the pty slave");
    tcsetwinsize(
        &slave,
        Winsize {
            ws_row: 24,
            ws_col: 80,
            ws_xpixel: 0,
            ws_ypixel: 0,
        },
    )
    .expect("set the pty size");
    (master, slave)
}

/// Read a stream in a thread until its end. On Linux, the master side of a
/// pseudo terminal gives `EIO` when the last slave closes, not end of file, so
/// `EIO` is also the end.
fn read_all(mut source: impl Read + Send + 'static) -> JoinHandle<Result<Vec<u8>, String>> {
    thread::spawn(move || {
        let mut data = Vec::new();
        let mut buffer = [0u8; 8192];
        loop {
            match source.read(&mut buffer) {
                Ok(0) => return Ok(data),
                Ok(count) => data.extend_from_slice(&buffer[..count]),
                Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                Err(e) if e.raw_os_error() == Some(rustix::io::Errno::IO.raw_os_error()) => {
                    return Ok(data);
                }
                Err(e) => return Err(e.to_string()),
            }
        }
    })
}

/// Run one program with the fixed environment of ADR-0021: an empty
/// environment, then `HOME`, `TERM=xterm-256color`, and the variables of the
/// invocation. Stdin is `/dev/null` or the file of the invocation, never a
/// terminal.
pub fn run(
    program: &Path,
    invocation: &Invocation,
    mode: RunMode,
    dir: &Path,
    home: &Path,
) -> Result<Run, String> {
    let mut command = Command::new(program);
    command
        .current_dir(dir)
        .env_clear()
        .env("HOME", home)
        .env("TERM", "xterm-256color");
    for (name, value) in invocation.env {
        match value {
            Some(value) => command.env(name, value),
            None => command.env_remove(name),
        };
    }
    if mode == RunMode::Color {
        command.arg("--color=always");
    }
    command.args(invocation.args);
    match invocation.stdin {
        Some(name) => {
            let path = parity_dir().join("stdin").join(name);
            let file = File::open(&path).map_err(|e| format!("open {}: {e}", path.display()))?;
            command.stdin(Stdio::from(file));
        }
        None => {
            command.stdin(Stdio::null());
        }
    }
    command.stderr(Stdio::piped());

    let mut master = None;
    if mode == RunMode::Terminal {
        let (m, slave) = open_pty();
        command.stdout(Stdio::from(slave));
        master = Some(m);
    } else {
        command.stdout(Stdio::piped());
    }

    let mut child = command
        .spawn()
        .map_err(|e| format!("start {}: {e}", program.display()))?;
    // The command holds its own copy of the terminal side. Drop it, so that
    // the master side sees the end when the program closes the terminal.
    drop(command);

    let stdout_reader = match master {
        Some(master) => read_all(File::from(master)),
        None => read_all(child.stdout.take().expect("a stdout pipe")),
    };
    let stderr_reader = read_all(child.stderr.take().expect("a stderr pipe"));

    let start = Instant::now();
    let status = loop {
        match child.try_wait().map_err(|e| format!("wait: {e}"))? {
            Some(status) => break status,
            None if start.elapsed() > TIME_LIMIT => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(format!(
                    "the program ran for more than {} s and was stopped",
                    TIME_LIMIT.as_secs()
                ));
            }
            None => thread::sleep(Duration::from_millis(2)),
        }
    };
    let stdout = stdout_reader
        .join()
        .map_err(|_| "the stdout reader failed".to_string())??;
    let stderr = stderr_reader
        .join()
        .map_err(|_| "the stderr reader failed".to_string())??;
    Ok(Run {
        stdout,
        stderr,
        code: status.code().unwrap_or(-1),
    })
}

fn replace_all(data: &[u8], from: &[u8], to: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len());
    let mut index = 0;
    while index < data.len() {
        if data[index..].starts_with(from) {
            out.extend_from_slice(to);
            index += from.len();
        } else {
            out.push(data[index]);
            index += 1;
        }
    }
    out
}

/// The name rule of card A (ADR-0021): in the stderr of rg, `rg:` at the start
/// of a line reads as `jag:`, and each `ripgrep` reads as `jag`. No other byte
/// changes.
pub fn read_rg_name_as_jag(stderr: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(stderr.len());
    for line in stderr.split_inclusive(|byte| *byte == b'\n') {
        let line = match line.strip_prefix(b"rg:") {
            Some(rest) => [b"jag:".as_slice(), rest].concat(),
            None => line.to_vec(),
        };
        out.extend(replace_all(&line, b"ripgrep", b"jag"));
    }
    out
}

/// A short text for a byte string: the escaped bytes around `at`.
fn excerpt(data: &[u8], at: usize) -> String {
    let start = at.saturating_sub(24);
    let end = (at + 40).min(data.len());
    let text: String = data[start..end]
        .iter()
        .flat_map(|byte| std::ascii::escape_default(*byte))
        .map(char::from)
        .collect();
    format!(
        "{}{text}{}",
        if start > 0 { "..." } else { "" },
        if end < data.len() { "..." } else { "" }
    )
}

/// The difference of one stream, or none when the two are the same.
fn stream_difference(
    name: &str,
    expected: &[u8],
    actual: &[u8],
    expected_label: &str,
) -> Option<String> {
    if expected == actual {
        return None;
    }
    let at = expected
        .iter()
        .zip(actual)
        .take_while(|(a, b)| a == b)
        .count();
    Some(format!(
        "    {name} differs at byte {at} ({expected_label} {} bytes, jag {} bytes)\n      {expected_label:<6}: {}\n      jag   : {}",
        expected.len(),
        actual.len(),
        excerpt(expected, at),
        excerpt(actual, at),
    ))
}

/// The three streams of two runs. `None` when they are the same. Else the names
/// of the streams that differ, and the text of the differences.
pub fn run_difference(
    expected: &Run,
    actual: &Run,
    expected_label: &str,
) -> Option<(String, String)> {
    let mut parts = Vec::new();
    let mut names = Vec::new();
    if let Some(text) =
        stream_difference("stdout", &expected.stdout, &actual.stdout, expected_label)
    {
        parts.push(text);
        names.push("stdout");
    }
    if let Some(text) =
        stream_difference("stderr", &expected.stderr, &actual.stderr, expected_label)
    {
        parts.push(text);
        names.push("stderr");
    }
    if expected.code != actual.code {
        parts.push(format!(
            "    exit code differs ({expected_label} {}, jag {})",
            expected.code, actual.code
        ));
        names.push("exit");
    }
    if parts.is_empty() {
        None
    } else {
        Some((names.join(" "), parts.join("\n")))
    }
}

/// The path of the pinned rg, or none when the test skips.
pub fn find_rg() -> Option<PathBuf> {
    match std::env::var_os("JAG_PARITY_RG") {
        Some(path) => Some(PathBuf::from(path)),
        None if std::env::var_os("CI").is_some() => {
            panic!(
                "JAG_PARITY_RG is not set and CI is set: run scripts/parity/install-rg.sh and set JAG_PARITY_RG"
            )
        }
        None => {
            println!("rg parity: JAG_PARITY_RG is not set, so the comparison is skipped");
            None
        }
    }
}

/// The rg of the machine must be the pinned version.
pub fn check_rg_version(rg: &Path) {
    let output = Command::new(rg)
        .arg("--version")
        .env_clear()
        .output()
        .unwrap_or_else(|e| panic!("run {} --version: {e}", rg.display()));
    let text = String::from_utf8_lossy(&output.stdout);
    let wanted = format!("ripgrep {}", pinned_version());
    // The version must end at a space or at the end of the line, so that
    // "ripgrep 15.2.01" is not "ripgrep 15.2.0".
    let first = text.lines().next().unwrap_or("");
    assert!(
        first == wanted || first.starts_with(&format!("{wanted} ")),
        "{} must be {wanted} (tests/parity/rg.sha256), but it says: {first}",
        rg.display(),
    );
}

/// Run rg and jag with the same invocation. The stderr of rg gets the name
/// rule. The error has the summary line and the detail text of a difference.
pub fn compare_with_rg(
    label: &str,
    rg: &Path,
    jag: &Path,
    invocation: &Invocation,
    mode: RunMode,
    dir: &Path,
    home: &Path,
) -> Result<(), (String, String)> {
    let jag_run = run(jag, invocation, mode, dir, home).map_err(|e| {
        (
            format!("{label}: jag did not finish"),
            format!("{label}: jag: {e}"),
        )
    })?;
    let rg_run = run(rg, invocation, mode, dir, home).map_err(|e| {
        (
            format!("{label}: rg did not finish"),
            format!("{label}: rg: {e}"),
        )
    })?;
    let rg_run = Run {
        stderr: read_rg_name_as_jag(&rg_run.stderr),
        ..rg_run
    };
    match run_difference(&rg_run, &jag_run, "rg") {
        None => Ok(()),
        Some((names, text)) => Err((format!("{label}: {names}"), format!("{label}\n{text}"))),
    }
}
