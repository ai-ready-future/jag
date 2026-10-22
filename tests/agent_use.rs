//! The agent test and its record (FEATURE-26, CRITERION-128 to CRITERION-133).
//!
//! The maintainer runs one command. It gives Claude Code, with the jag skill,
//! a fixed set of tasks (`tests/agent_use/tasks.yaml`). For each task it
//! records what the agent did: did it run jag in place of rg, did it set a
//! limit on the text, did it use the file and the line of a result, and, when
//! jag could not run, did it give the user the cause and the fix that jag
//! writes. The test records the choices of the agent. It never fails on them.
//!
//! The real run is the ignored test `the_agent_uses_jag_well`. It uses the
//! login of the maintainer and the subscription. It is not run in CI. The other
//! tests check the rules of the record with made-up transcripts, and they run
//! in CI with no agent and no key. See `tests/agent_use/README.md`.

mod common;
#[path = "parity/harness.rs"]
mod harness;

use std::collections::BTreeSet;
use std::fs;
use std::os::unix::fs::{PermissionsExt, symlink};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use common::KEY;
use common::fake_server::{FakeServer, Reply};
use harness::copy_tree;
use serde::Deserialize;
use serde_json::{Value, json};

/// The one command that runs the whole test (CRITERION-128).
const ONE_COMMAND: &str = "cargo test --locked --test agent_use -- --ignored --nocapture";

/// The model of the agent, so that each run uses the same model (spec row 3).
const MODEL: &str = "claude-sonnet-5-5";

/// The most turns of one task, and the longest time of one task (spec row 10).
const MAX_TURNS: u32 = 30;
const TASK_TIME_LIMIT: Duration = Duration::from_secs(600);

/// The score of a snippet that no `scores` row of the task names.
const OTHER_SCORE: f64 = 0.2;

/// The link that the skill gives for the install steps of jag.
const INSTALL_LINK: &str = "https://github.com/ai-ready-future/jag#install";

/// The version of the tool that installs the skill (ADR-0024).
const SKILLS_VERSION: &str = "1.7.4";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn sample() -> PathBuf {
    root().join("tests/readme/sample")
}

// ---------------------------------------------------------------------------
// The tasks (CRITERION-129)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
struct Score {
    text: String,
    score: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct Task {
    id: String,
    prompt: String,
    service: String,
    #[serde(default)]
    scores: Vec<Score>,
    #[serde(default)]
    jag_named_in_task: bool,
}

impl Task {
    /// The code that the service of the task forces, or `None` for a task
    /// where jag works.
    fn forced_code(&self) -> Option<i32> {
        match self.service.as_str() {
            "no-key" => Some(3),
            "refused" => Some(4),
            "no-credit" => Some(5),
            "down" => Some(6),
            "not-installed" => Some(127),
            _ => None,
        }
    }
}

fn tasks_text() -> String {
    let path = root().join("tests/agent_use/tasks.yaml");
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn load_tasks() -> Vec<Task> {
    serde_saphyr::from_str(&tasks_text()).unwrap_or_else(|e| panic!("parse tasks.yaml: {e}"))
}

// ---------------------------------------------------------------------------
// The transcript of one task
// ---------------------------------------------------------------------------

/// One tool call of the agent and its result.
#[derive(Debug, Clone)]
struct Call {
    id: String,
    name: String,
    input: Value,
    result: Option<String>,
    is_error: bool,
}

#[derive(Debug, Default)]
struct Transcript {
    init: Option<Value>,
    calls: Vec<Call>,
    result: Option<Value>,
}

impl Transcript {
    /// The final text of the agent. A result with an error has none.
    fn final_text(&self) -> String {
        self.result
            .as_ref()
            .and_then(|result| result["result"].as_str())
            .unwrap_or_default()
            .to_owned()
    }
}

/// The text of the `content` of a `tool_result` block.
fn content_text(content: &Value) -> String {
    match content {
        Value::String(text) => text.clone(),
        Value::Array(blocks) => blocks
            .iter()
            .filter_map(|block| block["text"].as_str())
            .collect::<Vec<_>>()
            .join("\n"),
        _ => String::new(),
    }
}

/// Read the lines of a `--output-format stream-json` transcript. A line that
/// is not JSON is left out.
fn parse_transcript(lines: &[String]) -> Transcript {
    let mut transcript = Transcript::default();
    for line in lines {
        let Ok(value) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        match value["type"].as_str() {
            Some("system") if value["subtype"] == "init" && transcript.init.is_none() => {
                transcript.init = Some(value);
            }
            Some("assistant") => {
                for block in value["message"]["content"].as_array().into_iter().flatten() {
                    if block["type"] == "tool_use" {
                        transcript.calls.push(Call {
                            id: block["id"].as_str().unwrap_or_default().to_owned(),
                            name: block["name"].as_str().unwrap_or_default().to_owned(),
                            input: block["input"].clone(),
                            result: None,
                            is_error: false,
                        });
                    }
                }
            }
            Some("user") => {
                for block in value["message"]["content"].as_array().into_iter().flatten() {
                    if block["type"] != "tool_result" {
                        continue;
                    }
                    let id = block["tool_use_id"].as_str().unwrap_or_default();
                    if let Some(call) = transcript.calls.iter_mut().find(|call| call.id == id) {
                        call.result = Some(content_text(&block["content"]));
                        call.is_error = block["is_error"].as_bool().unwrap_or(false);
                    }
                }
            }
            Some("result") => transcript.result = Some(value),
            _ => {}
        }
    }
    transcript
}

// ---------------------------------------------------------------------------
// The shell commands of the agent
// ---------------------------------------------------------------------------

/// The Bash command of a call.
fn bash_command(call: &Call) -> Option<&str> {
    if call.name == "Bash" {
        call.input["command"].as_str()
    } else {
        None
    }
}

/// A command cut into parts at `|`, `;`, `&`, and each line break. Each part
/// is a list of words.
fn command_parts(command: &str) -> Vec<Vec<String>> {
    let mut parts = vec![String::new()];
    for c in command.chars() {
        if matches!(c, '|' | ';' | '&' | '\n') {
            parts.push(String::new());
        } else if let Some(last) = parts.last_mut() {
            last.push(c);
        }
    }
    parts
        .iter()
        .map(|part| {
            part.split_whitespace()
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .filter(|words| !words.is_empty())
        .collect()
}

/// A word of the form `NAME=value`.
fn is_assignment(word: &str) -> bool {
    match word.split_once('=') {
        Some((name, _)) => {
            !name.is_empty()
                && !name.starts_with(|c: char| c.is_ascii_digit())
                && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        }
        None => false,
    }
}

/// The program of a part: the first word after the words `NAME=value`, with
/// any folder before it removed. And the words that follow it.
fn program(part: &[String]) -> Option<(&str, &[String])> {
    let at = part.iter().position(|word| !is_assignment(word))?;
    let name = part[at].rsplit('/').next().unwrap_or_default();
    Some((name, &part[at + 1..]))
}

/// The search tool that a part runs (spec row 17).
fn search_tool(part: &[String]) -> Option<&'static str> {
    let (name, rest) = program(part)?;
    match name {
        "jag" => Some("jag"),
        "rg" => Some("rg"),
        "grep" | "egrep" | "fgrep" => Some("grep"),
        "git" if rest.first().map(String::as_str) == Some("grep") => Some("grep"),
        _ => None,
    }
}

/// Whether a Bash command runs `jag`.
fn runs_jag(command: &str) -> bool {
    command_parts(command)
        .iter()
        .any(|part| search_tool(part) == Some("jag"))
}

// ---------------------------------------------------------------------------
// The runs of the jag wrapper
// ---------------------------------------------------------------------------

/// One call of the wrapper `bin/jag`, read from its folder `jag-runs/<n>`.
#[derive(Debug, Clone)]
struct JagRun {
    number: u32,
    argv: Vec<String>,
    max_chars_variable: Option<String>,
    stdout: String,
    exit: i32,
}

impl JagRun {
    fn has_argument(&self, wanted: &str) -> bool {
        self.argv
            .iter()
            .any(|argument| argument == wanted || argument.starts_with(&format!("{wanted}=")))
    }

    /// The JSON document on standard output, when it is one.
    fn document(&self) -> Option<Value> {
        serde_json::from_str(self.stdout.trim()).ok()
    }
}

/// The wrapper runs of a task folder, in their order.
fn read_runs(task_dir: &Path) -> Vec<JagRun> {
    let runs = task_dir.join("jag-runs");
    let mut numbers: Vec<u32> = fs::read_dir(&runs)
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.ok()?.file_name().to_str()?.parse().ok())
        .collect();
    numbers.sort_unstable();
    numbers
        .into_iter()
        .map(|number| {
            let dir = runs.join(number.to_string());
            let argv = fs::read(dir.join("argv")).unwrap_or_default();
            let mut argv: Vec<String> = argv
                .split(|byte| *byte == 0)
                .map(|argument| String::from_utf8_lossy(argument).into_owned())
                .collect();
            // Each argument ends with a NUL byte, so the last piece is empty.
            argv.pop();
            JagRun {
                number,
                argv,
                max_chars_variable: fs::read_to_string(dir.join("max_chars_variable")).ok(),
                stdout: fs::read_to_string(dir.join("stdout")).unwrap_or_default(),
                exit: fs::read_to_string(dir.join("exit"))
                    .ok()
                    .and_then(|text| text.trim().parse().ok())
                    .unwrap_or(-1),
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// The record rules (spec rows 17 to 20)
// ---------------------------------------------------------------------------

/// One search call of the agent.
#[derive(Debug, Clone, PartialEq)]
struct Search {
    tool: &'static str,
    text: String,
}

/// The search calls of a task, in their order (spec row 17).
fn searches(calls: &[Call]) -> Vec<Search> {
    let mut found = Vec::new();
    for call in calls {
        if let Some(command) = bash_command(call) {
            for part in command_parts(command) {
                if let Some(tool) = search_tool(&part) {
                    found.push(Search {
                        tool,
                        text: command.to_owned(),
                    });
                }
            }
        } else if call.name == "Grep" {
            found.push(Search {
                tool: "Grep",
                text: call.input["pattern"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned(),
            });
        }
    }
    found
}

/// The verdict of CRITERION-130. In the verdict, rg means rg, grep, or the
/// Grep tool.
fn search_verdict(found: &[Search]) -> &'static str {
    let jag = found.iter().any(|search| search.tool == "jag");
    let rg = found.iter().any(|search| search.tool != "jag");
    match (jag, rg) {
        (true, false) => "jag only",
        (true, true) => "jag and rg",
        (false, true) => "rg only",
        (false, false) => "no search",
    }
}

/// The limit of one wrapper run (spec row 18): the number of `--max-chars`,
/// `no limit`, or `not set`.
fn run_limit(run: &JagRun) -> String {
    let mut limit = None;
    let mut arguments = run.argv.iter();
    while let Some(argument) = arguments.next() {
        if argument == "--max-chars" {
            limit = arguments.next().cloned();
        } else if let Some(value) = argument.strip_prefix("--max-chars=") {
            limit = Some(value.to_owned());
        } else if argument == "--no-max-chars" {
            limit = Some("no limit".to_owned());
        }
    }
    // jag reads `JAG_MAX_CHARS` only in the flows with `--ask` or `--json`.
    let reads_variable = run.has_argument("--ask") || run.has_argument("--json");
    match (limit, &run.max_chars_variable) {
        (Some(limit), _) => limit,
        (None, Some(value)) if reads_variable => value.clone(),
        _ => "not set".to_owned(),
    }
}

/// The verdict of CRITERION-131.
fn limit_verdict(runs: &[JagRun]) -> String {
    if runs.is_empty() {
        return "no jag run".to_owned();
    }
    let unset = runs
        .iter()
        .filter(|run| run_limit(run) == "not set")
        .count();
    if unset == 0 {
        "set in each run".to_owned()
    } else {
        format!("not set in {unset} of {} runs", runs.len())
    }
}

/// A result of jag: a file and its lines.
#[derive(Debug, Clone, PartialEq)]
struct Hit {
    path: String,
    first_line: u64,
    last_line: u64,
}

/// The path without the project folder before it and without `./`.
fn relative_path(path: &str, project: &[String]) -> String {
    let mut path = path;
    for prefix in project {
        if let Some(rest) = path.strip_prefix(&format!("{prefix}/")) {
            path = rest;
            break;
        }
    }
    path.strip_prefix("./").unwrap_or(path).to_owned()
}

/// The results of the wrapper runs that gave results (exit 0, 7, or 8).
fn hits(runs: &[JagRun]) -> Vec<Hit> {
    let mut found = Vec::new();
    for run in runs.iter().filter(|run| [0, 7, 8].contains(&run.exit)) {
        let Some(document) = run.document() else {
            continue;
        };
        for result in document["results"].as_array().into_iter().flatten() {
            if let (Some(path), Some(first_line), Some(last_line)) = (
                result["path"].as_str(),
                result["first_line"].as_u64(),
                result["last_line"].as_u64(),
            ) {
                found.push(Hit {
                    path: path.strip_prefix("./").unwrap_or(path).to_owned(),
                    first_line,
                    last_line,
                });
            }
        }
    }
    found
}

/// Whether a call opens the lines of a result (spec row 19): a Read that
/// holds the lines, or a Bash command with `cat`, `sed`, `head`, `tail`,
/// `nl`, or `awk` that names the file.
fn call_opens(call: &Call, hit: &Hit, project: &[String]) -> bool {
    if call.name == "Read" {
        let file = call.input["file_path"].as_str().unwrap_or_default();
        if relative_path(file, project) != hit.path {
            return false;
        }
        let start = match call.input["offset"].as_u64() {
            None | Some(0) => 1,
            Some(offset) => offset,
        };
        let limit = call.input["limit"].as_u64().unwrap_or(2000);
        return start <= hit.first_line && hit.last_line < start + limit;
    }
    let Some(command) = bash_command(call) else {
        return false;
    };
    command_parts(command).iter().any(|part| {
        let Some((name, rest)) = program(part) else {
            return false;
        };
        ["cat", "sed", "head", "tail", "nl", "awk"].contains(&name)
            && rest.iter().any(|word| {
                relative_path(word.trim_matches(|c| c == '\'' || c == '"'), project) == hit.path
            })
    })
}

/// Whether the text cites the line of a result as `path:number`.
fn text_cites(text: &str, hit: &Hit) -> bool {
    let needle = format!("{}:", hit.path);
    text.match_indices(&needle).any(|(index, _)| {
        let digits: String = text[index + needle.len()..]
            .chars()
            .take_while(char::is_ascii_digit)
            .collect();
        digits
            .parse::<u64>()
            .is_ok_and(|number| (hit.first_line..=hit.last_line).contains(&number))
    })
}

/// The file and line record of CRITERION-132.
fn file_and_line(
    calls: &[Call],
    runs: &[JagRun],
    final_text: &str,
    project: &[String],
    check_edit: bool,
) -> Value {
    let found = hits(runs);
    let mut opened = BTreeSet::new();
    let mut cited = BTreeSet::new();
    for hit in &found {
        if calls.iter().any(|call| call_opens(call, hit, project)) {
            opened.insert(hit.path.clone());
        }
        if text_cites(final_text, hit) {
            cited.insert(hit.path.clone());
        }
    }
    let verdict = if found.is_empty() {
        "no result"
    } else if opened.is_empty() && cited.is_empty() {
        "not used"
    } else {
        "used"
    };
    let mut record = json!({
        "opened": opened,
        "cited": cited,
        "verdict": verdict,
    });
    if check_edit {
        // Did the agent open a result before its first Edit of that file?
        let edit = calls
            .iter()
            .position(|call| call.name == "Edit")
            .map(|index| {
                let file = calls[index].input["file_path"].as_str().unwrap_or_default();
                (index, relative_path(file, project))
            });
        record["opened_before_edit"] = match edit {
            None => Value::Null,
            Some((index, file)) => json!(found.iter().any(|hit| {
                hit.path == file
                    && calls[..index]
                        .iter()
                        .any(|call| call_opens(call, hit, project))
            })),
        };
    }
    record
}

/// A text for the comparison of the failure rule (spec row 20): lower case,
/// no backticks and no `*`, one space for each run of white space, and no `.`
/// at the end.
fn normalized(text: &str) -> String {
    let text: String = text
        .to_lowercase()
        .chars()
        .filter(|c| *c != '`' && *c != '*')
        .collect();
    text.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim_end_matches('.')
        .to_owned()
}

/// `yes` or `no`.
fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

/// The failure record of CRITERION-133, for a task with a forced code.
fn failure_record(task: &Task, runs: &[JagRun], calls: &[Call], final_text: &str) -> Value {
    let Some(code) = task.forced_code() else {
        return Value::Null;
    };
    let said = normalized(final_text);
    if code == 127 {
        // jag is not installed: a Bash call that runs jag gives "not found"
        // or the exit code 127.
        let reached = calls.iter().any(|call| {
            bash_command(call).is_some_and(runs_jag)
                && call
                    .result
                    .as_deref()
                    .is_some_and(|result| result.contains("not found") || result.contains("127"))
        });
        return json!({
            "reached": reached,
            "exit_code": if reached { json!(127) } else { Value::Null },
            "install_link_given": final_text.contains(INSTALL_LINK),
            "not_installed_said": said.contains("not installed"),
        });
    }
    let Some(first) = runs.iter().find(|run| run.exit == code) else {
        return json!({ "reached": false });
    };
    let error = first.document().map(|document| document["error"].clone());
    let (cause, fix) = match error.as_ref() {
        Some(error) => (error["cause"].as_str(), error["fix"].as_str()),
        None => (None, None),
    };
    let given = |text: Option<&str>| match text {
        Some(text) => yes_no(said.contains(&normalized(text))),
        None => "no JSON",
    };
    json!({
        "reached": true,
        "exit_code": code,
        "cause_given": given(cause),
        "fix_given": given(fix),
        "jag_runs_after_failure": runs.iter().filter(|run| run.number > first.number).count(),
    })
}

/// The `setup` of the record (spec row 27): what the `init` line says loaded.
fn setup_record(init: &Value, claude_version: &str, personal_skill: &str) -> Value {
    let mut setup = serde_json::Map::new();
    for name in [
        "model",
        "skills",
        "plugins",
        "mcp_servers",
        "tools",
        "apiKeySource",
    ] {
        if !init[name].is_null() {
            setup.insert(name.to_owned(), init[name].clone());
        }
    }
    json!({
        "claude_version": claude_version,
        "personal_skill": personal_skill,
        "init": setup,
    })
}

/// Whether the `init` line names the project skill `jag`.
fn init_names_skill(init: &Value) -> bool {
    init["skills"].as_array().is_some_and(|skills| {
        skills
            .iter()
            .any(|skill| skill.as_str() == Some("jag") || skill["name"].as_str() == Some("jag"))
    })
}

// ---------------------------------------------------------------------------
// The setup of one task: the environment, the wrapper, the skill
// ---------------------------------------------------------------------------

/// The failure text of the test: one line for the cause and one for the fix.
fn cause_and_fix(cause: &str, fix: &str) -> String {
    format!("cause: {cause}\nfix: {fix}")
}

/// The environment of the agent (spec row 11): the environment of the test,
/// without `ANTHROPIC_API_KEY`, without each `JAG_` variable, and without
/// `TYPESAFE_API_KEY`. `PATH` starts with `bin`. A folder of `PATH` that holds
/// a file `jag` is replaced by a new folder under `scratch` with a link to each
/// of its other files.
fn agent_environment(
    vars: &[(String, String)],
    bin: &Path,
    scratch: &Path,
) -> Vec<(String, String)> {
    let mut environment: Vec<(String, String)> = vars
        .iter()
        .filter(|(name, _)| {
            name != "ANTHROPIC_API_KEY"
                && name != "TYPESAFE_API_KEY"
                && name != "PATH"
                && !name.starts_with("JAG_")
        })
        .cloned()
        .collect();
    let path = vars
        .iter()
        .find(|(name, _)| name == "PATH")
        .map(|(_, value)| value.as_str())
        .unwrap_or_default();
    let mut folders = vec![bin.to_path_buf()];
    for (index, folder) in std::env::split_paths(path).enumerate() {
        if fs::symlink_metadata(folder.join("jag")).is_err() {
            folders.push(folder);
            continue;
        }
        let replaced = scratch.join(format!("path-{index}"));
        fs::create_dir_all(&replaced).expect("make a replaced PATH folder");
        for entry in fs::read_dir(&folder).into_iter().flatten().flatten() {
            if entry.file_name() != "jag" {
                let _ = symlink(entry.path(), replaced.join(entry.file_name()));
            }
        }
        folders.push(replaced);
    }
    let joined = std::env::join_paths(folders).expect("join the PATH folders");
    environment.push(("PATH".to_owned(), joined.to_string_lossy().into_owned()));
    environment
}

/// A word for a POSIX shell, in single quotes.
fn sh_quote(text: &str) -> String {
    format!("'{}'", text.replace('\'', "'\\''"))
}

/// The proxy variables that jag would read; the wrapper removes them.
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

/// Write the wrapper `<task_dir>/bin/jag` (spec row 12). It records each call
/// in a new folder `<task_dir>/jag-runs/<n>`, and it runs `jag` with the
/// environment of the service of the task. The agent shell never holds that
/// environment.
fn write_wrapper(task_dir: &Path, jag: &Path, base_url: &str, jag_home: &Path, key: Option<&str>) {
    let bin = task_dir.join("bin");
    fs::create_dir_all(&bin).expect("make the bin folder");
    fs::create_dir_all(jag_home).expect("make the jag home");
    let runs = task_dir.join("jag-runs");
    let removed: String = PROXY_VARIABLES
        .iter()
        .map(|name| format!("-u {name} "))
        .collect();
    let key = key.map_or(String::new(), |key| {
        format!("JAG_GATEWAY_API_KEY={} ", sh_quote(key))
    });
    let script = format!(
        "#!/bin/sh\n\
         runs={runs}\n\
         mkdir -p \"$runs\"\n\
         n=1\n\
         while ! mkdir \"$runs/$n\" 2>/dev/null; do n=$((n + 1)); done\n\
         dir=\"$runs/$n\"\n\
         : > \"$dir/argv\"\n\
         for argument in \"$@\"; do printf '%s\\0' \"$argument\" >> \"$dir/argv\"; done\n\
         if [ \"${{JAG_MAX_CHARS+set}}\" = set ]; then printf '%s' \"$JAG_MAX_CHARS\" > \"$dir/max_chars_variable\"; fi\n\
         env {removed}HOME={home} JAG_BASE_URL={url} {key}JAG_TIMEOUT=1 JAG_RETRY_WAIT=1 {jag} \"$@\" > \"$dir/stdout\"\n\
         code=$?\n\
         printf '%s' \"$code\" > \"$dir/exit\"\n\
         cat \"$dir/stdout\"\n\
         exit \"$code\"\n",
        runs = sh_quote(&runs.to_string_lossy()),
        home = sh_quote(&jag_home.to_string_lossy()),
        url = sh_quote(base_url),
        jag = sh_quote(&jag.to_string_lossy()),
    );
    let path = bin.join("jag");
    fs::write(&path, script).expect("write the wrapper");
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).expect("make it executable");
}

/// The personal skill check (spec row 28). A personal skill `jag` hides the
/// project skill. It gives `none`, `same`, or an error with the cause and the
/// fix.
fn personal_skill(home: &Path, checkout_skill: &[u8]) -> Result<&'static str, String> {
    let path = home.join(".claude/skills/jag/SKILL.md");
    match fs::read(&path) {
        Err(_) => Ok("none"),
        Ok(bytes) if bytes == checkout_skill => Ok("same"),
        Ok(_) => Err(cause_and_fix(
            &format!(
                "a personal jag skill ({}) hides the skill of the checkout",
                path.display()
            ),
            "update or remove ~/.claude/skills/jag, then run the command again",
        )),
    }
}

/// The first file `name` in the folders of a `PATH` value.
fn find_in_path(name: &str, path: &str) -> Option<PathBuf> {
    std::env::split_paths(path)
        .map(|folder| folder.join(name))
        .find(|candidate| candidate.is_file())
}

/// The needs of the real run (spec row 5): `claude` and `npx` on the PATH.
fn check_needs(path: &str) -> Result<PathBuf, String> {
    let claude = find_in_path("claude", path).ok_or_else(|| {
        cause_and_fix(
            "the program `claude` (Claude Code) is not on the PATH of the test",
            "install Claude Code and log in, then run the command again",
        )
    })?;
    if find_in_path("npx", path).is_none() {
        return Err(cause_and_fix(
            "the program `npx` (Node.js) is not on the PATH of the test",
            "install Node.js 18 or later, then run the command again",
        ));
    }
    Ok(claude)
}

/// Copy a folder and what its links point to.
fn copy_following_links(from: &Path, to: &Path) {
    fs::create_dir_all(to).expect("make a folder");
    for entry in fs::read_dir(from).expect("read a folder") {
        let entry = entry.expect("read an entry");
        let target = to.join(entry.file_name());
        if fs::metadata(entry.path()).expect("read an entry").is_dir() {
            copy_following_links(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), &target).expect("copy a file");
        }
    }
}

/// Install the skill of the checkout into a template folder (spec row 9).
fn install_skill(template: &Path, home: &Path) -> Result<(), String> {
    fs::create_dir_all(template.join(".claude")).expect("make the template");
    fs::create_dir_all(home).expect("make the home");
    let output = Command::new("npx")
        .args(["--yes", &format!("skills@{SKILLS_VERSION}"), "add"])
        .arg(root())
        .args(["--agent", "claude-code", "-y"])
        .current_dir(template)
        .env("HOME", home)
        .env("DO_NOT_TRACK", "1")
        .env("NO_COLOR", "1")
        .stdin(Stdio::null())
        .output()
        .map_err(|e| cause_and_fix(&format!("npx did not start: {e}"), "install Node.js"))?;
    if !output.status.success() || !template.join(".claude/skills/jag/SKILL.md").exists() {
        return Err(cause_and_fix(
            &format!(
                "the install of the skill failed:\n{}\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ),
            "check the network connection, then run the command again",
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// The run of one task and of all tasks (CRITERION-128)
// ---------------------------------------------------------------------------

/// What a run needs. The real test gives `claude` from the PATH, the real
/// HOME, and the template made by `install_skill`.
struct Config {
    claude: PathBuf,
    home: PathBuf,
    template: PathBuf,
    out: PathBuf,
    time_limit: Duration,
}

/// The arguments of the agent command (spec row 10).
fn agent_arguments(prompt: &str) -> Vec<String> {
    [
        "-p",
        prompt,
        "--output-format",
        "stream-json",
        "--verbose",
        "--model",
        MODEL,
        "--max-turns",
        &MAX_TURNS.to_string(),
        "--no-session-persistence",
        "--permission-mode",
        "dontAsk",
        "--allowedTools",
        "Bash,Read,Edit,Grep,Glob,Skill",
        "--disallowedTools",
        "WebFetch,WebSearch",
    ]
    .iter()
    .map(|argument| (*argument).to_owned())
    .collect()
}

/// The fake model service of a task (spec row 13).
fn fake_service(task: &Task) -> FakeServer {
    let scores = task.scores.clone();
    let service = task.service.clone();
    FakeServer::start(move |request| match service.as_str() {
        "refused" => Reply::status(401),
        "no-credit" => Reply::status(402),
        "down" => Reply::status(500),
        _ => Reply::scores(request, |text| {
            scores
                .iter()
                .find(|row| text.contains(&row.text))
                .map_or(OTHER_SCORE, |row| row.score)
        }),
    })
}

/// Run `program` with `arguments`. Read standard output as lines, and stop
/// the program when it runs longer than `limit`.
fn run_agent(
    program: &Path,
    arguments: &[String],
    dir: &Path,
    environment: &[(String, String)],
    limit: Duration,
) -> Result<(Vec<String>, String), String> {
    let mut child = Command::new(program)
        .args(arguments)
        .current_dir(dir)
        .env_clear()
        .envs(environment.iter().map(|(name, value)| (name, value)))
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| {
            cause_and_fix(
                &format!("claude did not start: {e}"),
                "install Claude Code, then run the command again",
            )
        })?;
    let mut stdout = child.stdout.take().expect("piped standard output");
    let mut stderr = child.stderr.take().expect("piped standard error");
    let out_reader = thread::spawn(move || {
        let mut text = String::new();
        let _ = std::io::Read::read_to_string(&mut stdout, &mut text);
        text
    });
    let err_reader = thread::spawn(move || {
        let mut text = String::new();
        let _ = std::io::Read::read_to_string(&mut stderr, &mut text);
        text
    });
    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(_)) => break,
            Ok(None) if start.elapsed() > limit => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(cause_and_fix(
                    &format!("the agent ran longer than {} seconds", limit.as_secs()),
                    "run the command again; check the usage limit of the subscription",
                ));
            }
            Ok(None) => thread::sleep(Duration::from_millis(100)),
            Err(e) => return Err(cause_and_fix(&format!("wait for claude: {e}"), "run again")),
        }
    }
    let out = out_reader.join().unwrap_or_default();
    let err = err_reader.join().unwrap_or_default();
    Ok((out.lines().map(str::to_owned).collect(), err))
}

/// The unix seconds now.
fn unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs())
}

/// The output of `program --version`, trimmed.
fn version_of(program: &Path) -> String {
    Command::new(program)
        .arg("--version")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
        .unwrap_or_default()
}

/// Run one task and give its entry of the record (spec rows 8 to 15, 21).
fn run_task(task: &Task, config: &Config, folder: &Path, jag: &Path) -> Result<Value, String> {
    let work = tempfile::tempdir().expect("make the task folder");
    let task_dir = work.path();
    let project = task_dir.join("project");
    copy_tree(&sample(), &project);
    copy_following_links(
        &config.template.join(".claude/skills/jag"),
        &project.join(".claude/skills/jag"),
    );
    let server = (task.service != "not-installed").then(|| fake_service(task));
    let bin = task_dir.join("bin");
    fs::create_dir_all(&bin).expect("make the bin folder");
    if let Some(server) = &server {
        let key = (task.service != "no-key").then_some(KEY);
        write_wrapper(
            task_dir,
            jag,
            &server.base_url(),
            &task_dir.join("jag-home"),
            key,
        );
    }
    let vars: Vec<(String, String)> = std::env::vars().collect();
    let environment = agent_environment(&vars, &bin, &task_dir.join("scratch"));
    if task.service == "not-installed" {
        let path = environment
            .iter()
            .find(|(name, _)| name == "PATH")
            .map_or("", |(_, value)| value.as_str());
        if let Some(found) = find_in_path("jag", path) {
            return Err(cause_and_fix(
                &format!(
                    "the PATH of task {} finds a jag at {}",
                    task.id,
                    found.display()
                ),
                "remove that jag from the PATH of the test, then run the command again",
            ));
        }
    }

    let (lines, stderr) = run_agent(
        &config.claude,
        &agent_arguments(&task.prompt),
        &project,
        &environment,
        config.time_limit,
    )?;
    fs::create_dir_all(folder).expect("make the record folder");
    fs::write(folder.join(format!("{}.jsonl", task.id)), lines.join("\n")).expect("write");
    // Keep the wrapper folders next to the transcript.
    if task_dir.join("jag-runs").exists() {
        copy_following_links(
            &task_dir.join("jag-runs"),
            &folder.join(&task.id).join("jag-runs"),
        );
    }

    let transcript = parse_transcript(&lines);
    let Some(result) = transcript.result.clone() else {
        return Err(cause_and_fix(
            &format!(
                "the transcript of task {} has no result line. Standard error:\n{}",
                task.id,
                stderr.trim()
            ),
            "run `claude -p hello` to check the login and the setup, then run the command again",
        ));
    };
    if result["is_error"].as_bool().unwrap_or(false) && transcript.calls.is_empty() {
        return Err(cause_and_fix(
            &format!(
                "task {} ended with an error before its first tool call: {}",
                task.id,
                result["result"].as_str().unwrap_or(&stderr).trim()
            ),
            "run claude and log in, or wait for the limit, then run the command again",
        ));
    }
    let Some(init) = transcript.init.clone() else {
        return Err(cause_and_fix(
            &format!("the transcript of task {} has no init line", task.id),
            "run the command again",
        ));
    };
    if !init_names_skill(&init) {
        return Err(cause_and_fix(
            &format!(
                "the init line of task {} does not name the skill jag",
                task.id
            ),
            "check that the install of the skill worked, then run the command again",
        ));
    }

    let runs = read_runs(task_dir);
    for call in &transcript.calls {
        let Some(command) = bash_command(call).filter(|command| runs_jag(command)) else {
            continue;
        };
        if task.service == "not-installed" {
            let result = call.result.as_deref().unwrap_or_default();
            if !(result.contains("not found") || result.contains("127")) {
                return Err(cause_and_fix(
                    &format!(
                        "in task not-installed, `{command}` ran a jag that is not the wrapper"
                    ),
                    "remove that jag (for example an env of the Claude Code settings, a SessionStart hook, or an alias in the shell startup file), then run the command again",
                ));
            }
        }
    }
    // Each Bash call that runs jag must add at least one wrapper folder. A
    // jag that is not the wrapper (for example from a hook or an alias) adds
    // none. A failed call (is_error) is no exception.
    if task.service != "not-installed" {
        let jag_calls: Vec<&str> = transcript
            .calls
            .iter()
            .filter_map(bash_command)
            .filter(|command| runs_jag(command))
            .collect();
        if jag_calls.len() > runs.len() {
            return Err(cause_and_fix(
                &format!(
                    "in task {}, the agent made {} Bash calls that run jag, but the wrapper saw {} runs (for example `{}`)",
                    task.id,
                    jag_calls.len(),
                    runs.len(),
                    jag_calls[runs.len()]
                ),
                "check the PATH of the agent and the Claude Code settings (env, hooks, aliases), then run the command again",
            ));
        }
    }

    let final_text = transcript.final_text();
    let found = searches(&transcript.calls);
    let prefixes: Vec<String> = [Some(project.clone()), project.canonicalize().ok()]
        .into_iter()
        .flatten()
        .map(|path| path.to_string_lossy().into_owned())
        .collect();
    let skill_tool_used = transcript.calls.iter().any(|call| {
        call.name == "Skill"
            && call.input["skill"]
                .as_str()
                .is_some_and(|name| name.ends_with("jag"))
    });
    Ok(json!({
        "id": task.id,
        "prompt": task.prompt,
        "forced_code": task.forced_code(),
        "jag_named_in_task": task.jag_named_in_task,
        "agent": {
            "subtype": result["subtype"],
            "num_turns": result["num_turns"],
            "total_cost_usd": result["total_cost_usd"],
            "duration_ms": result["duration_ms"],
            "skill_tool_used": skill_tool_used,
        },
        "init": init,
        "searches": found.iter().map(|s| json!({"tool": s.tool, "text": s.text})).collect::<Vec<_>>(),
        "search_verdict": search_verdict(&found),
        "first_search": found.first().map(|s| s.tool),
        "jag_runs": runs.iter().map(|run| json!({
            "argv": run.argv,
            "exit_code": run.exit,
            "limit": run_limit(run),
        })).collect::<Vec<_>>(),
        "limit_verdict": limit_verdict(&runs),
        "file_and_line": file_and_line(
            &transcript.calls, &runs, &final_text, &prefixes, task.id == "code-change"),
        "failure": failure_record(task, &runs, &transcript.calls, &final_text),
        "final_text": final_text,
    }))
}

/// Run all tasks and write the record (spec rows 16, 21, and 27). It gives the
/// record and the folder that holds it.
fn run_all(config: &Config, jag: &Path) -> Result<(Value, PathBuf), String> {
    let checkout_skill = fs::read(root().join("skills/jag/SKILL.md")).expect("read the skill");
    let personal = personal_skill(&config.home, &checkout_skill)?;
    let path = std::env::var("PATH").unwrap_or_default();
    let started = unix_seconds();
    let folder = config.out.join("agent-use").join(started.to_string());

    let mut entries = Vec::new();
    for task in load_tasks() {
        println!("task {} ...", task.id);
        entries.push(run_task(&task, config, &folder, jag)?);
    }
    let first_init = entries[0]["init"].clone();
    let total: f64 = entries
        .iter()
        .filter_map(|entry| entry["agent"]["total_cost_usd"].as_f64())
        .sum();
    let record = json!({
        "started": started,
        "jag_version": version_of(jag),
        "claude_version": version_of(&config.claude),
        "setup": setup_record(&first_init, &version_of(&config.claude), personal),
        "rg": find_in_path("rg", &path).map_or_else(|| "none".to_owned(), |rg| rg.display().to_string()),
        "task_file": tasks_text(),
        "total_cost_usd": total,
        "tasks": entries,
    });
    fs::create_dir_all(&folder).expect("make the record folder");
    fs::write(
        folder.join("record.json"),
        serde_json::to_string_pretty(&record).expect("write JSON"),
    )
    .expect("write the record");
    Ok((record, folder))
}

/// The table of the record: one line for each task (spec Design).
fn table(record: &Value) -> String {
    let mut rows = vec![[
        "task".to_owned(),
        "search".to_owned(),
        "limit".to_owned(),
        "file and line".to_owned(),
        "failure".to_owned(),
        "turns".to_owned(),
    ]];
    for task in record["tasks"].as_array().into_iter().flatten() {
        let mut search = task["search_verdict"]
            .as_str()
            .unwrap_or_default()
            .to_owned();
        if task["jag_named_in_task"] == true {
            search.push_str(" (asked)");
        }
        let failure = &task["failure"];
        let failure_text = if failure.is_null() {
            "-".to_owned()
        } else if failure["reached"] != true {
            "not reached".to_owned()
        } else if failure["exit_code"] == 127 {
            format!(
                "link {}, said {}",
                yes_no(failure["install_link_given"] == true),
                yes_no(failure["not_installed_said"] == true)
            )
        } else {
            format!(
                "cause {}, fix {}",
                failure["cause_given"].as_str().unwrap_or_default(),
                failure["fix_given"].as_str().unwrap_or_default()
            )
        };
        rows.push([
            task["id"].as_str().unwrap_or_default().to_owned(),
            search,
            task["limit_verdict"]
                .as_str()
                .unwrap_or_default()
                .to_owned(),
            task["file_and_line"]["verdict"]
                .as_str()
                .unwrap_or_default()
                .to_owned(),
            failure_text,
            task["agent"]["num_turns"].to_string(),
        ]);
    }
    let widths: Vec<usize> = (0..6)
        .map(|column| {
            rows.iter()
                .map(|row| row[column].chars().count())
                .max()
                .unwrap_or(0)
        })
        .collect();
    rows.iter()
        .map(|row| {
            row.iter()
                .zip(&widths)
                .map(|(cell, width)| format!("{cell:<width$}"))
                .collect::<Vec<_>>()
                .join("  ")
                .trim_end()
                .to_owned()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// CRITERION-128: one command runs the 9 tasks and writes the record. It uses
// the login of the maintainer, so the default run leaves it out.
#[test]
#[ignore = "runs Claude Code with your login: cargo test --locked --test agent_use -- --ignored --nocapture"]
fn the_agent_uses_jag_well() {
    let path = std::env::var("PATH").unwrap_or_default();
    let claude = check_needs(&path).unwrap_or_else(|error| panic!("{error}"));
    let work = tempfile::tempdir().expect("make a folder");
    let template = work.path().join("template");
    install_skill(&template, &work.path().join("home")).unwrap_or_else(|error| panic!("{error}"));
    let config = Config {
        claude,
        home: PathBuf::from(std::env::var("HOME").expect("HOME is set")),
        template,
        out: root().join("target"),
        time_limit: TASK_TIME_LIMIT,
    };
    let jag = PathBuf::from(assert_cmd::cargo::cargo_bin!("jag"));
    let (record, folder) = run_all(&config, &jag).unwrap_or_else(|error| panic!("{error}"));
    println!("\n{}\n", table(&record));
    println!("record: {}", folder.join("record.json").display());
}

// ---------------------------------------------------------------------------
// The tests that CI runs: no agent and no key
// ---------------------------------------------------------------------------

#[cfg(test)]
mod rules {
    use super::*;

    fn bash(id: &str, command: &str, result: &str) -> Vec<String> {
        vec![
            json!({"type": "assistant", "message": {"content": [
                {"type": "tool_use", "id": id, "name": "Bash", "input": {"command": command}}]}})
            .to_string(),
            json!({"type": "user", "message": {"content": [
                {"type": "tool_result", "tool_use_id": id, "content": result, "is_error": false}]}})
            .to_string(),
        ]
    }

    fn tool(id: &str, name: &str, input: Value) -> Vec<String> {
        vec![
            json!({"type": "assistant", "message": {"content": [
                {"type": "tool_use", "id": id, "name": name, "input": input}]}})
            .to_string(),
            json!({"type": "user", "message": {"content": [
                {"type": "tool_result", "tool_use_id": id, "content": "ok"}]}})
            .to_string(),
        ]
    }

    fn calls(parts: &[Vec<String>]) -> Vec<Call> {
        let lines: Vec<String> = parts.iter().flatten().cloned().collect();
        parse_transcript(&lines).calls
    }

    fn run(number: u32, argv: &[&str], variable: Option<&str>, stdout: &str, exit: i32) -> JagRun {
        JagRun {
            number,
            argv: argv.iter().map(|a| (*a).to_owned()).collect(),
            max_chars_variable: variable.map(str::to_owned),
            stdout: stdout.to_owned(),
            exit,
        }
    }

    fn project() -> Vec<String> {
        vec!["/work/project".to_owned()]
    }

    // CRITERION-128: the one command is in the test and in the maintainer text.
    #[test]
    fn the_agent_test_has_one_command() {
        let source = fs::read_to_string(root().join("tests/agent_use.rs")).unwrap();
        let lines: Vec<&str> = source.lines().collect();
        let at = lines
            .iter()
            .position(|line| line.starts_with("fn the_agent_uses_jag_well"))
            .expect("the test exists");
        let attribute = lines[at - 1];
        assert!(attribute.starts_with("#[ignore = "), "{attribute}");
        assert!(attribute.contains(ONE_COMMAND), "{attribute}");

        let readme = fs::read_to_string(root().join("tests/agent_use/README.md")).unwrap();
        assert!(readme.contains(ONE_COMMAND));
        let flat = readme.split_whitespace().collect::<Vec<_>>().join(" ");
        for needed in [
            "subscription",
            "Claude Code",
            "login",
            "npx",
            "30 turns",
            "target/agent-use",
        ] {
            assert!(flat.contains(needed), "the maintainer text lacks {needed}");
        }
    }

    // CRITERION-129: each run gives the agent the same tasks.
    #[test]
    fn the_tasks_are_fixed() {
        let tasks = load_tasks();
        let ids: Vec<&str> = tasks.iter().map(|task| task.id.as_str()).collect();
        assert_eq!(
            ids,
            [
                "code-question",
                "notes-question",
                "log-question",
                "code-change",
                "no-key",
                "key-refused",
                "no-credit",
                "service-down",
                "not-installed"
            ]
        );
        let again: Vec<String> = load_tasks().into_iter().map(|task| task.prompt).collect();
        let prompts: Vec<String> = tasks.iter().map(|task| task.prompt.clone()).collect();
        assert_eq!(prompts, again);
        let mut files = String::new();
        for relative in ["src/client.py", "notes/leave.md", "logs/deploy.log"] {
            files.push_str(&fs::read_to_string(sample().join(relative)).unwrap());
        }
        for task in &tasks {
            assert!(!task.prompt.trim().is_empty(), "{} has no prompt", task.id);
            assert!(
                [
                    "ok",
                    "no-key",
                    "refused",
                    "no-credit",
                    "down",
                    "not-installed"
                ]
                .contains(&task.service.as_str()),
                "{}: unknown service",
                task.id
            );
            for row in &task.scores {
                assert!(
                    files.contains(&row.text),
                    "{}: {} is not in the sample",
                    task.id,
                    row.text
                );
            }
            // Only a task where jag fails names jag (spec row 14).
            assert_eq!(
                task.jag_named_in_task,
                task.id.starts_with("no-") && task.service != "ok"
                    || ["key-refused", "service-down"].contains(&task.id.as_str())
            );
        }
    }

    // CRITERION-130
    #[test]
    fn the_record_shows_the_search_tool() {
        let verdict = |parts: &[Vec<String>]| search_verdict(&searches(&calls(parts)));
        assert_eq!(verdict(&[bash("1", "./bin/jag retry .", "x")]), "jag only");
        assert_eq!(
            verdict(&[bash("1", "FOO=1 /usr/local/bin/jag --ask q retry src", "x")]),
            "jag only"
        );
        assert_eq!(
            verdict(&[
                tool("1", "Grep", json!({"pattern": "retry"})),
                bash("2", "jag retry .", "x")
            ]),
            "jag and rg"
        );
        assert_eq!(
            verdict(&[bash("1", "cd src && rg retry .", "x")]),
            "rg only"
        );
        assert_eq!(
            verdict(&[bash("1", "cat x | grep y", "x"), bash("2", "jag y .", "x")]),
            "jag and rg"
        );
        assert_eq!(verdict(&[bash("1", "git grep retry", "x")]), "rg only");
        assert_eq!(
            verdict(&[tool("1", "Read", json!({"file_path": "/work/project/a"}))]),
            "no search"
        );
        let found = searches(&calls(&[
            tool("1", "Grep", json!({"pattern": "retry"})),
            bash("2", "jag retry .", "x"),
        ]));
        assert_eq!(found[0].tool, "Grep");
        assert_eq!(found[1].tool, "jag");

        // A task where jag is named gets `(asked)` after its verdict.
        let record = json!({"tasks": [{
            "id": "no-key", "search_verdict": "jag only", "jag_named_in_task": true,
            "limit_verdict": "set in each run", "file_and_line": {"verdict": "no result"},
            "failure": null, "agent": {"num_turns": 3}}]});
        assert!(table(&record).contains("jag only (asked)"));
    }

    // CRITERION-131
    #[test]
    fn the_record_shows_the_limit() {
        let limit =
            |argv: &[&str], variable: Option<&str>| run_limit(&run(1, argv, variable, "", 0));
        assert_eq!(
            limit(&["--max-chars", "8000", "--ask", "q", "x", "."], None),
            "8000"
        );
        assert_eq!(limit(&["--max-chars=8000", "x", "."], None), "8000");
        assert_eq!(limit(&["--no-max-chars", "x", "."], None), "no limit");
        // `JAG_MAX_CHARS=8000 jag ...` counts in a run with --ask or --json.
        assert_eq!(limit(&["--json", "x", "."], Some("8000")), "8000");
        assert_eq!(limit(&["--ask", "q", "x", "."], Some("8000")), "8000");
        assert_eq!(limit(&["x", "."], Some("8000")), "not set");
        assert_eq!(limit(&["--json", "x", "."], None), "not set");

        let set = run(1, &["--max-chars", "8000", "x"], None, "", 0);
        let unset = run(2, &["x"], None, "", 0);
        assert_eq!(limit_verdict(std::slice::from_ref(&set)), "set in each run");
        assert_eq!(limit_verdict(&[set, unset]), "not set in 1 of 2 runs");
        assert_eq!(limit_verdict(&[]), "no jag run");
    }

    fn document(first: u64, last: u64) -> String {
        json!({"exit_code": 0, "results": [
            {"rank": 1, "path": "src/client.py", "first_line": first, "last_line": last, "lines": []}]})
        .to_string()
    }

    // CRITERION-132
    #[test]
    fn the_record_shows_the_file_and_the_line() {
        let runs = [run(1, &["--json", "x", "."], None, &document(10, 14), 0)];
        let verdict = |parts: &[Vec<String>], text: &str| {
            file_and_line(&calls(parts), &runs, text, &project(), false)["verdict"]
                .as_str()
                .unwrap()
                .to_owned()
        };
        let read = |extra: Value| {
            let mut input = json!({"file_path": "/work/project/src/client.py"});
            for (name, value) in extra.as_object().unwrap() {
                input[name] = value.clone();
            }
            tool("r", "Read", input)
        };
        // A Read that holds the lines, with no offset or limit, and with offset 0.
        assert_eq!(
            verdict(&[read(json!({"offset": 5, "limit": 20}))], ""),
            "used"
        );
        assert_eq!(verdict(&[read(json!({}))], ""), "used");
        assert_eq!(
            verdict(&[read(json!({"offset": 0, "limit": 14}))], ""),
            "used"
        );
        // A Read that does not hold the lines.
        assert_eq!(
            verdict(&[read(json!({"offset": 0, "limit": 8}))], ""),
            "not used"
        );
        assert_eq!(
            verdict(&[read(json!({"offset": 20, "limit": 5}))], ""),
            "not used"
        );
        // A Read of another file.
        assert_eq!(
            verdict(
                &[tool(
                    "r",
                    "Read",
                    json!({"file_path": "/work/project/notes/leave.md"})
                )],
                ""
            ),
            "not used"
        );
        // A command that prints the file.
        assert_eq!(
            verdict(&[bash("1", "sed -n '10,14p' src/client.py", "x")], ""),
            "used"
        );
        assert_eq!(
            verdict(&[bash("1", "cat ./src/client.py", "x")], ""),
            "used"
        );
        assert_eq!(
            verdict(&[bash("1", "ls src/client.py", "x")], ""),
            "not used"
        );
        // A cited path and line.
        assert_eq!(verdict(&[], "The wait is in src/client.py:13."), "used");
        assert_eq!(verdict(&[], "The wait is in ./src/client.py:13."), "used");
        assert_eq!(
            verdict(&[], "The wait is in /work/project/src/client.py:10."),
            "used"
        );
        assert_eq!(verdict(&[], "The wait is in src/client.py:99."), "not used");
        assert_eq!(verdict(&[], "The wait is in src/client.py."), "not used");
        // No result: an exit code 1, or an empty list.
        for (stdout, exit) in [("", 1), (r#"{"results": []}"#, 0)] {
            let none = [run(1, &["x"], None, stdout, exit)];
            let record = file_and_line(&calls(&[read(json!({}))]), &none, "", &project(), false);
            assert_eq!(record["verdict"], "no result");
        }

        // For the task code-change: did the agent open the result before its
        // first Edit of that file?
        let edit = tool(
            "e",
            "Edit",
            json!({"file_path": "/work/project/src/client.py"}),
        );
        let before = file_and_line(
            &calls(&[read(json!({})), edit.clone()]),
            &runs,
            "",
            &project(),
            true,
        );
        assert_eq!(before["opened_before_edit"], true);
        let after = file_and_line(
            &calls(&[edit, read(json!({}))]),
            &runs,
            "",
            &project(),
            true,
        );
        assert_eq!(after["opened_before_edit"], false);
        let no_edit = file_and_line(&calls(&[read(json!({}))]), &runs, "", &project(), true);
        assert!(no_edit["opened_before_edit"].is_null());
    }

    fn failure_document() -> String {
        json!({"exit_code": 3, "error": {
            "cause": "No key was found.",
            "fix": "Run `jag --setup` in your terminal."}})
        .to_string()
    }

    fn task(id: &str) -> Task {
        load_tasks().into_iter().find(|task| task.id == id).unwrap()
    }

    // CRITERION-133
    #[test]
    fn the_record_shows_the_cause_and_the_fix() {
        let failed = [run(
            1,
            &["--json", "--ask", "q", "x", "."],
            None,
            &failure_document(),
            3,
        )];
        let record = |runs: &[JagRun], text: &str| failure_record(&task("no-key"), runs, &[], text);
        // The cause and the fix, in backticks and with a capital letter.
        let both = record(
            &failed,
            "jag could not run. **No key was found**. Fix: run `Jag --setup` in your terminal",
        );
        assert_eq!(both["reached"], true);
        assert_eq!(both["exit_code"], 3);
        assert_eq!(both["cause_given"], "yes");
        assert_eq!(both["fix_given"], "yes");
        // The cause only.
        let cause = record(&failed, "No key was found.");
        assert_eq!(
            (cause["cause_given"].as_str(), cause["fix_given"].as_str()),
            (Some("yes"), Some("no"))
        );
        // The fix of jag has backticks, and the text of the agent has none.
        let plain_text = record(
            &failed,
            "No key was found. Run jag --setup in your terminal.",
        );
        assert_eq!(plain_text["fix_given"], "yes");
        // A task where the agent used no --ask: jag gave no failure.
        let quiet = [run(1, &["x", "."], None, "", 0)];
        assert_eq!(record(&quiet, "done")["reached"], false);
        // A failure run with no JSON document.
        let text = [run(1, &["--ask", "q", "x", "."], None, "jag: no key", 3)];
        let plain = record(&text, "No key was found.");
        assert_eq!(plain["cause_given"], "no JSON");
        assert_eq!(plain["fix_given"], "no JSON");
        // Two more jag runs after the failure.
        let more = [
            failed[0].clone(),
            run(2, &["x"], None, "", 0),
            run(3, &["x"], None, "", 0),
        ];
        assert_eq!(record(&more, "")["jag_runs_after_failure"], 2);

        // Task not-installed.
        let installed = calls(&[bash("1", "jag --help", "jag: command not found")]);
        let said = failure_record(
            &task("not-installed"),
            &[],
            &installed,
            &format!("jag is not installed. See {INSTALL_LINK}"),
        );
        assert_eq!(said["reached"], true);
        assert_eq!(said["install_link_given"], true);
        assert_eq!(said["not_installed_said"], true);
        let silent = failure_record(&task("not-installed"), &[], &installed, "I used grep.");
        assert_eq!(silent["install_link_given"], false);
        assert_eq!(silent["not_installed_said"], false);
        let unreached = failure_record(&task("not-installed"), &[], &[], "x");
        assert_eq!(unreached["reached"], false);
    }

    // Row 12
    #[test]
    fn the_wrapper_records_each_jag_call() {
        let server = FakeServer::start(|_| Reply::status(401));
        let work = tempfile::tempdir().unwrap();
        let project = work.path().join("project");
        copy_tree(&sample(), &project);
        let jag = PathBuf::from(assert_cmd::cargo::cargo_bin!("jag"));
        write_wrapper(
            work.path(),
            &jag,
            &server.base_url(),
            &work.path().join("home"),
            Some(KEY),
        );
        let wrapper = work.path().join("bin/jag");

        let argv = [
            "--json",
            "--ask",
            "say \"it's\" a $HOME `test`",
            "retry",
            "src",
        ];
        let call = || {
            Command::new(&wrapper)
                .args(argv)
                .current_dir(&project)
                .env_clear()
                .env("PATH", "/usr/bin:/bin")
                .env("JAG_MAX_CHARS", "8000")
                .stdin(Stdio::null())
                .output()
                .unwrap()
        };
        let first = call();
        let runs = read_runs(work.path());
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].argv, argv);
        assert_eq!(runs[0].exit, 4);
        assert_eq!(first.status.code(), Some(4));
        assert_eq!(runs[0].max_chars_variable.as_deref(), Some("8000"));
        let document = runs[0].document().expect("the output is a JSON document");
        assert_eq!(document["exit_code"], 4);
        // The wrapper gives the same output and the same exit code as jag.
        assert_eq!(String::from_utf8_lossy(&first.stdout), runs[0].stdout);

        // A second call makes folder 2.
        call();
        let runs = read_runs(work.path());
        assert_eq!(
            runs.iter().map(|run| run.number).collect::<Vec<_>>(),
            [1, 2]
        );
        // The key of the test is not in the files that the agent can read.
        let script = fs::read_to_string(&wrapper).unwrap();
        assert!(script.contains(KEY), "the wrapper holds the made-up key");
        assert!(!runs[0].argv.concat().contains(KEY));
    }

    // Rows 11 and 25
    #[test]
    fn the_agent_environment_holds_no_key() {
        let work = tempfile::tempdir().unwrap();
        let (one, two) = (work.path().join("one"), work.path().join("two"));
        fs::create_dir_all(&one).unwrap();
        fs::create_dir_all(&two).unwrap();
        for name in ["jag", "rg"] {
            fs::write(one.join(name), "x").unwrap();
        }
        fs::write(two.join("node"), "x").unwrap();
        let path = std::env::join_paths([&one, &two]).unwrap();
        let var = |name: &str, value: &str| (name.to_owned(), value.to_owned());
        let vars = vec![
            var("ANTHROPIC_API_KEY", "made-up"),
            var("JAG_TYPESAFE_API_KEY", "made-up"),
            var("JAG_BASE_URL", "http://127.0.0.1:1"),
            var("TYPESAFE_API_KEY", "made-up"),
            var("HOME", "/home/maintainer"),
            var("PATH", &path.to_string_lossy()),
        ];
        let bin = work.path().join("bin");
        let environment = agent_environment(&vars, &bin, &work.path().join("scratch"));
        let names: Vec<&str> = environment.iter().map(|(name, _)| name.as_str()).collect();
        for gone in [
            "ANTHROPIC_API_KEY",
            "JAG_TYPESAFE_API_KEY",
            "JAG_BASE_URL",
            "TYPESAFE_API_KEY",
        ] {
            assert!(!names.contains(&gone), "{gone} is in the agent environment");
        }
        assert!(environment.contains(&var("HOME", "/home/maintainer")));

        let path = &environment
            .iter()
            .find(|(name, _)| name == "PATH")
            .unwrap()
            .1;
        let folders: Vec<PathBuf> = std::env::split_paths(path).collect();
        assert_eq!(folders[0], bin);
        assert_eq!(folders[2], two);
        // The replaced folder holds rg and no jag.
        assert!(folders[1].join("rg").exists());
        assert!(fs::symlink_metadata(folders[1].join("jag")).is_err());
        assert!(find_in_path("jag", path).is_none());
        assert!(find_in_path("rg", path).is_some());
    }

    // Row 28
    #[test]
    fn a_different_personal_skill_stops_the_test() {
        let home = tempfile::tempdir().unwrap();
        let skill = fs::read(root().join("skills/jag/SKILL.md")).unwrap();
        assert_eq!(personal_skill(home.path(), &skill), Ok("none"));
        let folder = home.path().join(".claude/skills/jag");
        fs::create_dir_all(&folder).unwrap();
        fs::write(folder.join("SKILL.md"), &skill).unwrap();
        assert_eq!(personal_skill(home.path(), &skill), Ok("same"));
        fs::write(folder.join("SKILL.md"), "another skill").unwrap();
        let error = personal_skill(home.path(), &skill).unwrap_err();
        assert!(error.starts_with("cause: "), "{error}");
        assert!(
            error.contains("\nfix: update or remove ~/.claude/skills/jag"),
            "{error}"
        );
    }

    // The runner, with a scripted agent in place of Claude Code. The script
    // runs the jag wrapper that it finds on its PATH, and it prints stream-json
    // lines. So the test checks the setup of each task, the wrapper, the
    // reading of the folders, and the record file, with no login.
    fn scripted_config(work: &Path, script: &str) -> Config {
        let claude = work.join("claude");
        fs::write(&claude, script).unwrap();
        fs::set_permissions(&claude, fs::Permissions::from_mode(0o755)).unwrap();
        let template = work.join("template");
        let skills = template.join(".claude/skills/jag");
        fs::create_dir_all(&skills).unwrap();
        fs::copy(root().join("skills/jag/SKILL.md"), skills.join("SKILL.md")).unwrap();
        Config {
            claude,
            home: work.join("home"),
            template,
            out: work.join("out"),
            time_limit: Duration::from_secs(60),
        }
    }

    // Row 21: a Bash call that runs jag but adds no wrapper folder stops the
    // test, also when the call failed (is_error). The scripted agent prints a
    // jag call with "Exit code 3" and does not run the wrapper.
    #[test]
    fn a_jag_call_with_no_wrapper_run_stops_the_test() {
        let work = tempfile::tempdir().unwrap();
        let config = scripted_config(
            work.path(),
            r#"#!/bin/sh
if [ "$1" = "--version" ]; then echo "0.0.0 (scripted)"; exit 0; fi
echo '{"type":"system","subtype":"init","model":"scripted","skills":["jag"]}'
echo '{"type":"assistant","message":{"content":[{"type":"tool_use","id":"t1","name":"Bash","input":{"command":"jag --json --ask q retry ."}}]}}'
echo '{"type":"user","message":{"content":[{"type":"tool_result","tool_use_id":"t1","content":"Exit code 3","is_error":true}]}}'
echo '{"type":"result","subtype":"success","is_error":false,"num_turns":2,"result":"done"}'
"#,
        );
        let jag = PathBuf::from(assert_cmd::cargo::cargo_bin!("jag"));
        let error = run_all(&config, &jag).unwrap_err();
        assert!(error.starts_with("cause: "), "{error}");
        assert!(error.contains("in task code-question"), "{error}");
        assert!(error.contains("the wrapper saw 0 runs"), "{error}");
        assert!(error.contains("jag --json --ask q retry ."), "{error}");
        assert!(error.contains("\nfix: "), "{error}");
    }

    // The same stop when only a later call misses the wrapper: the first call
    // runs the wrapper, the second call does not.
    #[test]
    fn a_later_jag_call_with_no_wrapper_run_stops_the_test() {
        let work = tempfile::tempdir().unwrap();
        let config = scripted_config(
            work.path(),
            r#"#!/bin/sh
if [ "$1" = "--version" ]; then echo "0.0.0 (scripted)"; exit 0; fi
echo '{"type":"system","subtype":"init","model":"scripted","skills":["jag"]}'
jag --json --ask q retry . >/dev/null 2>&1
echo '{"type":"assistant","message":{"content":[{"type":"tool_use","id":"t1","name":"Bash","input":{"command":"jag --json --ask q retry ."}}]}}'
echo '{"type":"user","message":{"content":[{"type":"tool_result","tool_use_id":"t1","content":"{}"}]}}'
echo '{"type":"assistant","message":{"content":[{"type":"tool_use","id":"t2","name":"Bash","input":{"command":"~/.cargo/bin/jag retry ."}}]}}'
echo '{"type":"user","message":{"content":[{"type":"tool_result","tool_use_id":"t2","content":"Exit code 1","is_error":true}]}}'
echo '{"type":"result","subtype":"success","is_error":false,"num_turns":3,"result":"done"}'
"#,
        );
        let jag = PathBuf::from(assert_cmd::cargo::cargo_bin!("jag"));
        let error = run_all(&config, &jag).unwrap_err();
        assert!(error.contains("~/.cargo/bin/jag retry ."), "{error}");
    }

    // The runner, with a scripted agent in place of Claude Code. The script
    // runs the jag wrapper that it finds on its PATH, and it prints stream-json
    // lines. So the test checks the setup of each task, the wrapper, the
    // reading of the folders, and the record file, with no login.
    #[test]
    fn the_runner_writes_the_record_with_a_scripted_agent() {
        let work = tempfile::tempdir().unwrap();
        let config = scripted_config(
            work.path(),
            r#"#!/bin/sh
if [ "$1" = "--version" ]; then echo "0.0.0 (scripted)"; exit 0; fi
echo '{"type":"system","subtype":"init","model":"scripted","skills":["jag"],"tools":["Bash"],"plugins":[],"mcp_servers":[],"apiKeySource":"none"}'
if command -v jag >/dev/null 2>&1; then
  jag --json --max-chars 8000 --ask where retry . >/dev/null 2>&1
  echo '{"type":"assistant","message":{"content":[{"type":"tool_use","id":"t1","name":"Bash","input":{"command":"jag --json --max-chars 8000 --ask where retry ."}}]}}'
  echo '{"type":"user","message":{"content":[{"type":"tool_result","tool_use_id":"t1","content":"{}"}]}}'
else
  echo '{"type":"assistant","message":{"content":[{"type":"tool_use","id":"t1","name":"Bash","input":{"command":"jag retry ."}}]}}'
  echo '{"type":"user","message":{"content":[{"type":"tool_result","tool_use_id":"t1","content":"jag: command not found","is_error":true}]}}'
fi
echo '{"type":"result","subtype":"success","is_error":false,"num_turns":2,"total_cost_usd":0.5,"duration_ms":5,"result":"done"}'
"#,
        );
        let jag = PathBuf::from(assert_cmd::cargo::cargo_bin!("jag"));
        let (record, folder) = run_all(&config, &jag).unwrap_or_else(|error| panic!("{error}"));

        let written: Value =
            serde_json::from_str(&fs::read_to_string(folder.join("record.json")).unwrap()).unwrap();
        assert_eq!(written, record);
        assert_eq!(record["task_file"], tasks_text());
        assert_eq!(record["claude_version"], "0.0.0 (scripted)");
        assert_eq!(record["setup"]["personal_skill"], "none");
        let tasks = record["tasks"].as_array().unwrap();
        assert_eq!(tasks.len(), 9);
        let find = |id: &str| tasks.iter().find(|task| task["id"] == id).unwrap();
        for id in [
            "code-question",
            "notes-question",
            "log-question",
            "code-change",
        ] {
            let entry = find(id);
            assert_eq!(entry["jag_runs"].as_array().unwrap().len(), 1, "{id}");
            // The scripted question fits only the first task; the others find
            // no match (exit code 1), and that is fine for this test.
            let code = entry["jag_runs"][0]["exit_code"].as_i64().unwrap();
            assert!([0, 1].contains(&code), "{id}: {code}");
            assert_eq!(entry["search_verdict"], "jag only", "{id}");
            assert_eq!(entry["limit_verdict"], "set in each run", "{id}");
            assert!(entry["failure"].is_null());
        }
        for (id, code) in [
            ("no-key", 3),
            ("key-refused", 4),
            ("no-credit", 5),
            ("service-down", 6),
        ] {
            let entry = find(id);
            assert_eq!(entry["failure"]["reached"], true, "{id}");
            assert_eq!(entry["failure"]["exit_code"], code, "{id}");
            // The scripted agent says "done", so it gave neither text.
            assert_eq!(entry["failure"]["cause_given"], "no", "{id}");
        }
        let missing = find("not-installed");
        assert_eq!(missing["failure"]["reached"], true);
        assert!(missing["jag_runs"].as_array().unwrap().is_empty());
        assert_eq!(record["total_cost_usd"], 4.5);
        // The field `rg` is a path or the text `none`, never null.
        assert!(record["rg"].is_string(), "{}", record["rg"]);
        assert!(folder.join("code-question.jsonl").exists());
        assert!(folder.join("code-question/jag-runs/1/argv").exists());
        assert!(table(&record).lines().count() == 10);
    }

    // The scripted agent makes the real tests of the wrapper work only when
    // the sample is as the README examples need it.
    #[test]
    fn the_tasks_use_the_files_of_the_sample() {
        for relative in ["src/client.py", "notes/leave.md", "logs/deploy.log"] {
            assert!(sample().join(relative).is_file(), "{relative}");
        }
    }
}
