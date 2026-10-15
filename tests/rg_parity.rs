//! The rg parity test (ADR-0021, FEATURE-23).
//!
//! With no `--ask`, jag must give the same stdout, the same stderr, the same
//! color codes, and the same exit code as the pinned rg for the same
//! arguments. The cases are data in `tests/parity/cases.yaml`. Each case runs
//! in three modes: in a pipe, in a pipe with `--color=always`, and with
//! stdout on a pseudo terminal.
//!
//! The test finds rg at the path in `JAG_PARITY_RG` (see
//! `scripts/parity/install-rg.sh`). Without that variable, the test fails
//! when `CI` is set, and else it skips the comparison.
//!
//! Two more variables help a developer, and CI sets neither:
//!
//! - `JAG_PARITY_CASE=id,id` runs only these cases.
//! - `JAG_PARITY_BLESS=1` writes the jag bytes of each deliberate case into
//!   `tests/parity/expected/`. Read the new files in the diff before you
//!   commit them: each file is a difference from rg that stays.

use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
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

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Case {
    id: String,
    args: Vec<String>,
    tree: String,
    #[serde(default)]
    modes: Option<Vec<RunMode>>,
    #[serde(default)]
    stdin: Option<String>,
    #[serde(default)]
    env: BTreeMap<String, String>,
    #[serde(default)]
    deliberate: Option<Deliberate>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Deliberate {
    /// The user ANSWER that records the difference.
    answer: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
enum RunMode {
    Pipe,
    Color,
    Terminal,
}

impl RunMode {
    const ALL: [RunMode; 3] = [RunMode::Pipe, RunMode::Color, RunMode::Terminal];

    fn name(self) -> &'static str {
        match self {
            RunMode::Pipe => "pipe",
            RunMode::Color => "color",
            RunMode::Terminal => "terminal",
        }
    }
}

impl Case {
    fn modes(&self) -> Vec<RunMode> {
        self.modes.clone().unwrap_or_else(|| RunMode::ALL.to_vec())
    }
}

/// What one program run gave.
#[derive(Debug, PartialEq, Eq)]
struct Run {
    stdout: Vec<u8>,
    stderr: Vec<u8>,
    code: i32,
}

fn parity_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/parity")
}

fn load_cases() -> Vec<Case> {
    let path = parity_dir().join("cases.yaml");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    serde_saphyr::from_str(&text).unwrap_or_else(|e| panic!("parse {}: {e}", path.display()))
}

/// The version of the pin: the first line of `rg.sha256` names an archive
/// `ripgrep-<version>-<target>.tar.gz`.
fn pinned_version() -> String {
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
fn tree_files(root: &Path) -> Vec<PathBuf> {
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
fn real_name(stored: &std::ffi::OsStr) -> OsString {
    let text = stored.to_string_lossy();
    match text.strip_prefix("dot-") {
        Some(rest) => format!(".{rest}").into(),
        None => stored.to_os_string(),
    }
}

/// Copy a fixture tree into a new folder. Git cannot store a path with a
/// `.git` part and a `.gitignore` would change what git stores, so a stored
/// name has `dot-` in place of `.`.
fn copy_tree(from: &Path, to: &Path) {
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
/// case. Stdin is `/dev/null` or the file of the case, never a terminal.
fn run(program: &Path, case: &Case, mode: RunMode, dir: &Path, home: &Path) -> Result<Run, String> {
    let mut command = Command::new(program);
    command
        .current_dir(dir)
        .env_clear()
        .env("HOME", home)
        .env("TERM", "xterm-256color");
    for (name, value) in &case.env {
        command.env(name, value);
    }
    if mode == RunMode::Color {
        command.arg("--color=always");
    }
    command.args(&case.args);
    match &case.stdin {
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
fn read_rg_name_as_jag(stderr: &[u8]) -> Vec<u8> {
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

/// The three streams of two runs. `None` when they are the same.
fn run_difference(expected: &Run, actual: &Run, expected_label: &str) -> Option<(String, String)> {
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

/// The folder with the stored jag bytes of one deliberate case.
fn expected_dir(case: &Case) -> PathBuf {
    parity_dir().join("expected").join(&case.id)
}

fn read_stored(case: &Case, mode: RunMode) -> Result<Run, String> {
    let dir = expected_dir(case);
    let read = |extension: &str| {
        let path = dir.join(format!("{}.{extension}", mode.name()));
        fs::read(&path).map_err(|e| format!("read {}: {e}", path.display()))
    };
    let code =
        String::from_utf8(read("code")?).map_err(|e| format!("the stored exit code: {e}"))?;
    Ok(Run {
        stdout: read("stdout")?,
        stderr: read("stderr")?,
        code: code
            .trim()
            .parse()
            .map_err(|e| format!("the stored exit code: {e}"))?,
    })
}

fn write_stored(case: &Case, mode: RunMode, run: &Run) {
    let dir = expected_dir(case);
    fs::create_dir_all(&dir).expect("make the folder of the stored bytes");
    let write = |extension: &str, bytes: &[u8]| {
        fs::write(dir.join(format!("{}.{extension}", mode.name())), bytes)
            .expect("write the stored bytes");
    };
    write("stdout", &run.stdout);
    write("stderr", &run.stderr);
    write("code", format!("{}\n", run.code).as_bytes());
}

/// The cases that the files hold must be consistent (CRITERION-110).
#[test]
fn cases_name_files_that_exist() {
    let cases = load_cases();
    assert!(!cases.is_empty(), "tests/parity/cases.yaml has no case");
    let mut problems = Vec::new();
    let mut ids = BTreeSet::new();
    for case in &cases {
        let id = &case.id;
        if !ids.insert(id.clone()) {
            problems.push(format!("{id}: the id is not unique"));
        }
        if id.is_empty()
            || !id
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            problems.push(format!("{id}: an id has only a-z, 0-9, and -"));
        }
        let tree = parity_dir().join("fixtures").join(&case.tree);
        if !tree.is_dir() {
            problems.push(format!(
                "{id}: the fixture tree {} does not exist",
                case.tree
            ));
        }
        if let Some(stdin) = &case.stdin
            && !parity_dir().join("stdin").join(stdin).is_file()
        {
            problems.push(format!("{id}: the stdin file {stdin} does not exist"));
        }
        if let Some(modes) = &case.modes
            && (modes.is_empty()
                || modes
                    .iter()
                    .enumerate()
                    .any(|(i, m)| modes[..i].contains(m)))
        {
            problems.push(format!("{id}: the modes are empty or repeat a mode"));
        }
        if let Some(deliberate) = &case.deliberate
            && deliberate.answer.trim().is_empty()
        {
            problems.push(format!("{id}: a deliberate case must name its ANSWER"));
        }
        // The name rule hides a difference when a case holds these words.
        for argument in &case.args {
            if argument.contains("rg:") || argument.contains("ripgrep") {
                problems.push(format!("{id}: an argument holds rg: or ripgrep"));
            }
        }
    }
    let fixtures = parity_dir().join("fixtures");
    if fixtures.is_dir() {
        for relative in tree_files(&fixtures) {
            let text = relative.to_string_lossy();
            if text.contains("rg:") || text.contains("ripgrep") {
                problems.push(format!("fixture {text}: the name holds rg: or ripgrep"));
            }
            if relative
                .iter()
                .any(|part| part.to_string_lossy().starts_with('.'))
            {
                problems.push(format!(
                    "fixture {text}: a stored name must not start with a dot; use dot-"
                ));
            }
        }
    }
    // A stored record needs a deliberate case, and the reverse.
    let expected = parity_dir().join("expected");
    if expected.is_dir() {
        for entry in fs::read_dir(&expected)
            .expect("read the stored bytes")
            .flatten()
        {
            let name = entry.file_name().to_string_lossy().to_string();
            // `.gitkeep` keeps the folder in git while no case is deliberate.
            if name.starts_with('.') {
                continue;
            }
            match cases.iter().find(|case| case.id == name) {
                Some(case) if case.deliberate.is_some() => {}
                Some(_) => problems.push(format!(
                    "{name}: stored bytes of a case that is not deliberate; remove them"
                )),
                None => problems.push(format!(
                    "{name}: stored bytes of a case that does not exist; remove them"
                )),
            }
        }
    }
    assert!(
        problems.is_empty(),
        "the parity cases are not consistent:\n{}",
        problems.join("\n")
    );
}

/// The path of the pinned rg, or none when the test skips.
fn find_rg() -> Option<PathBuf> {
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
fn check_rg_version(rg: &Path) {
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

/// CRITERION-111 (pipe), CRITERION-112 (color and terminal), CRITERION-113
/// (CI runs this test with the pinned rg).
#[test]
fn jag_gives_the_same_output_as_rg() {
    let Some(rg) = find_rg() else { return };
    check_rg_version(&rg);
    let jag = PathBuf::from(env!("CARGO_BIN_EXE_jag"));
    let bless = std::env::var_os("JAG_PARITY_BLESS").is_some();
    let only: Option<Vec<String>> = std::env::var("JAG_PARITY_CASE")
        .ok()
        .map(|list| list.split(',').map(str::to_string).collect());

    let mut details = Vec::new();
    let mut summary = Vec::new();
    let mut runs = 0;
    for case in load_cases() {
        if let Some(only) = &only
            && !only.contains(&case.id)
        {
            continue;
        }
        // The copy is outside the repository, so no ignore file of the
        // repository has an effect.
        let work = tempfile::Builder::new()
            .prefix("jag-parity-")
            .tempdir()
            .expect("make a folder");
        copy_tree(&parity_dir().join("fixtures").join(&case.tree), work.path());
        // An empty `HOME` for each case: a file that a program writes there
        // does not reach the next case.
        let home = tempfile::Builder::new()
            .prefix("jag-parity-home-")
            .tempdir()
            .expect("make a folder");
        for mode in case.modes() {
            runs += 1;
            let label = format!("{} [{}]", case.id, mode.name());
            let jag_run = match run(&jag, &case, mode, work.path(), home.path()) {
                Ok(run) => run,
                Err(e) => {
                    details.push(format!("{label}: jag: {e}"));
                    summary.push(format!("{label}: jag did not finish"));
                    continue;
                }
            };
            if case.deliberate.is_some() && bless {
                write_stored(&case, mode, &jag_run);
                continue;
            }
            let rg_run = match run(&rg, &case, mode, work.path(), home.path()) {
                Ok(run) => Run {
                    stderr: read_rg_name_as_jag(&run.stderr),
                    ..run
                },
                Err(e) => {
                    details.push(format!("{label}: rg: {e}"));
                    summary.push(format!("{label}: rg did not finish"));
                    continue;
                }
            };
            match &case.deliberate {
                None => {
                    if let Some((names, text)) = run_difference(&rg_run, &jag_run, "rg") {
                        details.push(format!("{label}\n{text}"));
                        summary.push(format!("{label}: {names}"));
                    }
                }
                Some(deliberate) => {
                    let stored = match read_stored(&case, mode) {
                        Ok(stored) => stored,
                        Err(e) => {
                            details.push(format!(
                                "{label}: deliberate ({}) but no stored jag bytes: {e}",
                                deliberate.answer
                            ));
                            summary.push(format!("{label}: no stored jag bytes"));
                            continue;
                        }
                    };
                    if let Some((names, text)) = run_difference(&stored, &jag_run, "stored") {
                        details.push(format!(
                            "{label}: jag differs from its stored bytes\n{text}"
                        ));
                        summary.push(format!(
                            "{label}: deliberate, jag differs from the stored bytes: {names}"
                        ));
                    }
                    if run_difference(&stored, &rg_run, "stored").is_none() {
                        details.push(format!(
                            "{label}: rg gives the stored jag bytes now. The record ({}) must go.",
                            deliberate.answer
                        ));
                        summary.push(format!(
                            "{label}: deliberate, but rg and jag agree: remove the record"
                        ));
                    }
                }
            }
        }
    }
    assert!(
        summary.is_empty(),
        "{} of {runs} runs differ from rg {}:\n\n{}\n\nSUMMARY ({} runs)\n{}\n",
        summary.len(),
        pinned_version(),
        details.join("\n"),
        summary.len(),
        summary.join("\n")
    );
}
