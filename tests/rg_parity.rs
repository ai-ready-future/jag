//! The rg parity test (ADR-0021, FEATURE-23).
//!
//! With no `--ask`, jag must give the same stdout, the same stderr, the same
//! color codes, and the same exit code as the pinned rg for the same
//! arguments. The cases are data in `tests/parity/cases.yaml`. Each case runs
//! in three modes: in a pipe, in a pipe with `--color=always`, and with
//! stdout on a pseudo terminal. `tests/rg_parity_fs.rs` holds the cases that
//! need a file system set-up, with the same rules.
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

#[path = "parity/harness.rs"]
mod harness;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

use harness::{
    Invocation, Run, RunMode, check_rg_version, copy_tree, find_rg, parity_dir, pinned_version,
    read_rg_name_as_jag, run, run_difference, tree_files,
};
use serde::Deserialize;

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
    /// Extra variables. A value of `null` removes the variable of the fixed
    /// environment (for example `TERM`).
    #[serde(default)]
    env: BTreeMap<String, Option<String>>,
    #[serde(default)]
    deliberate: Option<Deliberate>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Deliberate {
    /// The user ANSWER that records the difference.
    answer: String,
}

impl Case {
    fn modes(&self) -> Vec<RunMode> {
        self.modes.clone().unwrap_or_else(|| RunMode::ALL.to_vec())
    }

    fn invocation(&self) -> Invocation<'_> {
        Invocation {
            args: &self.args,
            stdin: self.stdin.as_deref(),
            env: &self.env,
        }
    }
}

fn load_cases() -> Vec<Case> {
    let path = parity_dir().join("cases.yaml");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    serde_saphyr::from_str(&text).unwrap_or_else(|e| panic!("parse {}: {e}", path.display()))
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
            let invocation = case.invocation();
            let jag_run = match run(&jag, &invocation, mode, work.path(), home.path()) {
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
            let rg_run = match run(&rg, &invocation, mode, work.path(), home.path()) {
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
