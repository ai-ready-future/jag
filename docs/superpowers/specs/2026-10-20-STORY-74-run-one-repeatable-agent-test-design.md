---
title: "Prove that an agent with the skill uses jag well: one agent test and its record"
feature: FEATURE-26
stories: [STORY-74, STORY-75]
kind: build
gh_issue: 187
lifecycle_state: draft
inherits: [IF-3, IF-4, IF-5, CV-1, CV-8, QS-5]
session: 4acb9f7a-4603-41fb-8e74-395e5117e288
---

# Prove that an agent with the skill uses jag well: one agent test and its record

## Problem

FEATURE-26: "The maintainer runs one repeatable test. The test gives an agent
with the jag skill a set of tasks. For each task, it records whether the
agent ran jag in place of rg, whether it set a limit on the text, whether it
used the file and line of a result, and whether it gave the user the cause
and the fix that jag writes when jag could not run."

CAPABILITY-11 gives the behavior to check. After the install, the agent uses
jag in place of rg, and it decides when to use `--ask`. It sets a limit on
the text that jag returns. It reads the structured output and opens the file
and the line of each result. "If jag is not installed, the key is not set, or
the model service is down, the agent gives the user the cause and the fix
that jag writes."

The two Stories:

- STORY-74: "As an agent builder (PERSONA-01), I want to run one repeatable
  test that gives an agent with the jag skill a set of tasks, so that I can
  check the skill at any time."
  - CRITERION-128: "The agent builder starts the whole test with one
    command."
  - CRITERION-129: "Each run gives the agent the same set of tasks."
- STORY-75: "As an agent builder (PERSONA-01), I want the test to record the
  choices of the agent for each task, so that I can see if the agent uses jag
  well."
  - CRITERION-130: "For each task, the record shows if the agent ran jag in
    place of rg."
  - CRITERION-131: "For each task, the record shows if the agent set a limit
    on the text."
  - CRITERION-132: "For each task, the record shows if the agent used the
    file and the line of a result."
  - CRITERION-133: "For each task where jag cannot run, the record shows if
    the agent gave the user the cause and the fix that jag writes."

The repository has no code that starts an agent or reads its tool calls. No
workflow holds a model key, and the Actions policy allows no schedule.
DECISION-54, DECISION-55, and DECISION-56 bind the record: the skill gives no
rule on when to use which tool, no fallback rule after a failure, and no rule
to always use `--ask`. So the test records the choices of the agent. It does
not fail on them.

## Decisions

Row 2 is the answer of an open rule card. Until the ANSWER, the row holds the
recommended option, and STORY-74 and STORY-75 stay in `backlog`.

| # | Decision | Choice | Source |
|---|---|---|---|
| 1 | Specs | One spec for STORY-74 and STORY-75. They cannot ship apart: the record of STORY-75 is the output of the test of STORY-74, and a test with no record shows nothing | Orchestrator TASK ("1 feature = 1 spec = 1 PR"); FEATURE-26 page |
| 2 | The agent and the payment | Claude Code with no personal setup (`claude --bare -p`), paid by the `ANTHROPIC_API_KEY` of the maintainer. A cap of $1 stops each task (`--max-budget-usd 1`). A run has 9 tasks, so a run costs at most $9 | **Pending ANSWER** (ASK open, rule card "The agent that the skill test runs", round 1, recommended option). Claude Code docs: headless mode (`--bare` loads no hooks, plugins, MCP servers, CLAUDE.md, or memory, and it loads skills only from an `--add-dir` folder; bare mode reads no subscription login, only `ANTHROPIC_API_KEY`), CLI reference (`--max-budget-usd`) |
| 3 | The model | A constant `MODEL = "claude-sonnet-5-5"` in `tests/agent_use.rs`, given with `--model`. The record holds the model name from the `init` line of each transcript | Derived from row 2; Claude Code CLI reference (`--model`) |
| 4 | The runner (CRITERION-128) | One Rust integration test file, `tests/agent_use.rs`. The test `the_agent_uses_jag_well` has `#[ignore = "runs a paid agent: cargo test --locked --test agent_use -- --ignored --nocapture"]`. The one command is `cargo test --locked --test agent_use -- --ignored --nocapture`. `--nocapture` is necessary, because cargo keeps the output of a test that passes. `cargo test --locked` in CI does not run it | ADR-0021 (the e2e tests are cargo integration tests, with no second test runner); `tests/speed.rs` (a test that the default run leaves out); `tests/common/fake_server.rs` (the fake service runs only in a Rust test process) |
| 5 | A missing need | The test fails at once, with one line for the cause and one line for the fix, when `claude` is not on the PATH of the test, when `ANTHROPIC_API_KEY` is not set or is empty, or when `npx` is not on the PATH of the test. It does not skip, because the maintainer started it on purpose | ADR-0021 (`find_rg` fails in CI when the pinned rg is missing); CV-1 (each failure gives its cause and its fix) |
| 6 | The tasks (CRITERION-129) | One file, `tests/agent_use/tasks.yaml`, read with serde-saphyr. It holds the 9 tasks of the Design, in a fixed order. The test gives the `prompt` text of each task as it is. It adds no text, no random value, and no date. Each task runs in a new agent process, with `--no-session-persistence` | `tests/parity/cases.yaml` and `tests/rg_parity.rs` (the cases are in a YAML file, read with serde-saphyr); ADR-0021 (a tester adds a case without Rust code) |
| 7 | The task set | 4 search tasks and 5 failure tasks, one for each code 3, 4, 5, 6, and 127. No task for 126: a shell or the npm launcher gives it only when a jag file does not start, and the skill gives the same steps for 126 and 127 | Derived from row 2 (the card says 9 tasks); CAPABILITY-11 (behaviors 1 to 4, and the failure paragraph); IF-4 (126 and 127 come only from a shell or the npm launcher); `skills/jag/SKILL.md` ("When jag cannot run") |
| 8 | The corpus | Each task gets a new temp project folder with a copy of `tests/readme/sample/`, made with `copy_tree`. The test never writes to `tests/readme/sample/` | `tests/readme_examples.rs` (it copies the sample into a temp folder); FEATURE-27 spec (the README examples need the sample as it is) |
| 9 | The skill install | One time in each run: `npx --yes skills@1.7.4 add <checkout> --agent claude-code -y`, in a temp template folder, with a temp HOME and `DO_NOT_TRACK=1`. `<checkout>` is `CARGO_MANIFEST_DIR`, so the test uses the skill of the checkout. The test copies `.claude/skills/jag/` of the template into each project folder | ADR-0024 (`skills@1.7.4`); `tests/agent_skill/install.test.js` (an install from the checkout path, with `DO_NOT_TRACK=1`); ADR-0025 (`skills/jag/SKILL.md` is the one source) |
| 10 | The agent command | In the project folder: `claude --bare -p <prompt> --output-format stream-json --verbose --model <MODEL> --add-dir <project> --max-budget-usd 1 --max-turns 30 --no-session-persistence`. `--allowedTools` names Bash, Read, Edit, Grep, Glob, and Skill. `--disallowedTools` names WebFetch and WebSearch. The test stops an agent that runs longer than 10 minutes. The tasks run one after the other | Derived from row 2; Claude Code docs (headless mode, CLI reference) |
| 11 | The agent environment | The test clears the environment of the agent and sets only: `HOME`, a new temp folder; `PATH`, the `bin` folder of the task (row 12), the folder of `JAG_PARITY_RG` when that variable is set, the folder of `claude`, `/usr/bin`, and `/bin`; `ANTHROPIC_API_KEY`, from the environment of the test; `CLAUDE_CODE_SUBPROCESS_ENV_SCRUB=1`; `DO_NOT_TRACK=1`. The agent environment holds no `JAG_` variable and no key of jag | Claude Code docs (env-vars: `CLAUDE_CODE_SUBPROCESS_ENV_SCRUB=1` removes the Anthropic key and other credential variables from the shell of the agent); `tests/common/mod.rs` (the tests remove the `JAG_` variables and use a temp HOME); QS-5 |
| 12 | The jag wrapper | The test writes `<task>/bin/jag`, a POSIX `sh` file. For each call, it makes the next folder `<task>/jag-runs/<n>` with `mkdir` (n starts at 1, and `mkdir` fails when the folder exists, so two calls never get one folder). It writes the arguments to `argv`, each followed by a NUL byte, and the value of `JAG_MAX_CHARS` to `max_chars_variable` when that variable is set. It runs the jag of the build (`cargo_bin!("jag")`) with the arguments, the environment of the shell, and the jag environment below. It writes the standard output to `stdout` and the exit code to `exit`. Then it writes the standard output on its own standard output and exits with the same code. Standard input and standard error pass through. The jag environment: `HOME`, a temp folder that only jag uses; `JAG_BASE_URL`, the address of the fake service; `JAG_GATEWAY_API_KEY`, `common::KEY` (not set for task `no-key`); `JAG_TIMEOUT=1`; `JAG_RETRY_WAIT=1`. The wrapper sets these only for jag. So the scrub of row 11 cannot remove them, and the shell of the agent does not hold them | CV-8 (jag writes no log file, so the wrapper records); `tests/common/mod.rs` (`KEY`, the gateway route); ADR-0017 (a fake service on the gateway route needs the gateway key) |
| 13 | The fake service | One `FakeServer` for each task, in the test process. For a task with `service: ok`, it scores each snippet with the first `scores` row of the task whose text is in the snippet, and gives 0.2 to the other snippets. `refused` answers HTTP 401 (exit 4), `no-credit` HTTP 402 (exit 5), and `down` HTTP 500 (exit 6). Tasks `no-key` and `not-installed` need no answer | `tests/common/fake_server.rs`; `tests/readme_examples.rs` (`fake_service`, `OTHER_SCORE`); `tests/failure.rs` (401, 402, and 500 give 4, 5, and 6) |
| 14 | How a failure task reaches the failure | Without `--ask`, jag makes no model call and needs no key (IF-3). So the 4 tasks for codes 3 to 6 tell the agent in the task text to use jag with a question. This is a request of the user in the task. The skill does not change: it does not tell the agent to use `--ask` (DECISION-56). When the agent still uses no `--ask`, the record says `not reached` | IF-3; DECISION-56; DECISION-55 |
| 15 | The sources of the record | The stream-json transcript of each task (the `init` line, each tool call and its result, and the final `result` line), the wrapper folders of row 12, and nothing else | Claude Code docs (headless mode: `--output-format stream-json`); row 12 |
| 16 | The record file | One JSON document, `target/agent-use/<unix seconds>/record.json`. The same folder holds the transcript of each task as `<task id>.jsonl` and its wrapper folders. The test also prints one table on standard output, with one line for each task. Nothing in `target/` is committed | `.gitignore` (`/target`); `scripts/speed/measure.py` (it prints a summary) |
| 17 | The search rule (CRITERION-130) | The test lists the search calls of a task in their order. A Bash command is cut into parts at `\|`, `;`, `&`, and each line break. The first word of a part, after the words of the form `NAME=value` and with any folder before it removed, gives the tool: `jag` is jag; `rg` is rg; `grep`, `egrep`, `fgrep`, and `git` followed by `grep` are grep. A call of the Grep tool is the Grep tool. The verdict: `jag only`, `jag and rg`, `rg only`, or `no search`. In the verdict, rg means rg, grep, or the Grep tool. The record also gives the first search tool | `skills/jag/SKILL.md` ("Use jag in place of rg"); DECISION-54; Claude Code docs (tools reference: Grep) |
| 18 | The limit rule (CRITERION-131) | For each wrapper run, the record gives the value N of `--max-chars N` or `--max-chars=N`, `no limit` for `--no-max-chars`, or `not set`. A `JAG_MAX_CHARS` value in the shell of the agent counts as a limit too, because jag reads it in the `--json` flow. The verdict: `set in each run`, `not set in <k> of <m> runs`, or `no jag run`. The record also says if the agent called the Skill tool for the skill `jag` | `skills/jag/SKILL.md` ("`--max-chars N` is present in each command"); IF-3; `src/cli.rs` (`JAG_MAX_CHARS` in the `--ask` and `--json` flows) |
| 19 | The file and line rule (CRITERION-132) | The results are the `results` of the JSON on `stdout` of each wrapper run with exit 0, 7, or 8. A result is **opened** when a Read call names its file in the project folder and the lines that it reads hold `first_line` to `last_line`. A Read with no `offset` or with `offset` 0 starts at line 1. Any other `offset` is the first line that it reads. A Read with no `limit` reads 2,000 lines. A result is also opened when a Bash command with `cat`, `sed`, `head`, `tail`, `nl`, or `awk` names its file. A result is **cited** when the final text holds `path:number`, where `number` is the number of a line of that result. A `./` or the project folder before `path` also matches. The verdict: `used` (opened or cited), `not used`, or `no result`. For task `code-change`, the record also says if the agent opened the result before its first Edit of that file | `skills/jag/SKILL.md` ("Open `path` from `first_line` to `last_line` before you change code", "Cite a line as `path:number`"); IF-5; Claude Code Read tool (2,000 lines by default) |
| 20 | The failure rule (CRITERION-133) | For a task with a code from 3 to 6, the failure is **reached** when a wrapper run has that exit code. Then **cause given** is yes when the final text holds `error.cause` of that run, and **fix given** is yes when it holds `error.fix` of that run. Before the test compares, it changes the two texts in the same way: lower case, no backticks and no `*`, each run of white space as one space, and no `.` at the end. The record also counts the wrapper runs after the first run with that code. For task `not-installed`, the failure is reached when a Bash call runs `jag` and its result holds `not found` or exit code 127. Then the record says if the final text holds `https://github.com/ai-ready-future/jag#install`, and if it holds `not installed`. The texts come from each run, not from a copy in the test. So a change of the failure texts of jag or of the skill does not change the test | `skills/jag/SKILL.md` ("When jag cannot run"); DECISION-55; CV-1; IF-4 |
| 21 | When the test fails | The test passes when each of the 9 tasks gave a final `result` line and the record file is written. It never fails on a choice of the agent. A result with the budget cap or the turn cap is a result. The test fails, with a cause and a fix, when: a need of row 5 is missing; the skill install fails; the `init` line of a task does not name the skill `jag`; the PATH of task `not-installed` finds a `jag`; an agent runs longer than 10 minutes; or a transcript has no `result` line | FEATURE-26 ("it records"); CRITERION-130 to CRITERION-133 ("the record shows"); DECISION-54, DECISION-55, and DECISION-56 |
| 22 | The tests that CI runs | The rules of rows 17 to 20 are functions in `tests/agent_use.rs`. Tests with no `#[ignore]` give them made-up transcripts and wrapper folders and check each verdict. Other tests check the task file (CRITERION-129), the one command (CRITERION-128), and the wrapper with the jag of the build and a `FakeServer`. They run in the job `rust` with `cargo test --locked`, with no agent and no key | ADR-0021; `.github/workflows/ci.yml` (`cargo test --locked`) |
| 23 | The maintainer text | `tests/agent_use/README.md` gives the one command, the needs (Claude Code, `ANTHROPIC_API_KEY`, Node with `npx`, a network connection), the cost (at most $9 for a run), and the place of the record. The README of the product does not change | FEATURE-26 ("The maintainer runs"); `Cargo.toml` (`include` holds no `tests/`); `README.md` `## Speed` (a measure by hand gives its cost) |
| 24 | CI | No CI job runs the agent test, and no workflow gets a new secret | The scout summary (no model key in any workflow; the Actions policy allows no schedule); ADR-0021 (only `private-data` is a required check) |
| 25 | Security | `ANTHROPIC_API_KEY` is only in the environment of the `claude` process. The scrub of row 11 keeps it out of the shell of the agent. The test never prints it. Before the test writes a transcript, it replaces each copy of the key value with `[ANTHROPIC_API_KEY]`. jag gets only `common::KEY`, a made-up key, and a HOME with no key file. So jag never reads a key of the maintainer. The agent runs with the rights of the maintainer, so the tasks are fixed and the tools are limited (row 10) | QS-5; `scripts/speed/measure.py` (it never prints the key); Claude Code docs (env-vars: the scrub) |
| 26 | Third-party dependencies | No new crate. The test uses std, serde_json, serde-saphyr, `tests/common`, and `tests/parity/harness.rs` (`copy_tree`). The outside programs are `claude`, `npx` with `skills@1.7.4`, and `sh` | ADR-0012 (each new outside crate needs a reason); ADR-0024 |

## Design

### The tasks (CRITERION-129)

`tests/agent_use/tasks.yaml` holds a list. Each entry has `id`, `prompt`,
`service`, and, for `service: ok`, `scores`: a list of pairs of a text and a
score. `service` is one of `ok`, `no-key`, `refused`, `no-credit`, `down`,
or `not-installed`. The forced code comes from `service`: `no-key` 3,
`refused` 4, `no-credit` 5, `down` 6, `not-installed` 127.

| # | `id` | `service` | `scores` | `prompt` |
|---|---|---|---|---|
| 1 | `code-question` | `ok` | `time.sleep(retry_wait)` 0.9 | "Where does the code wait before it tries a failed request again? Give the file and the line." |
| 2 | `notes-question` | `ok` | `25 days` 0.95 | "How many days of paid leave do new staff get? Give the file and the line." |
| 3 | `log-question` | `ok` | `step 6 of 6 failed` 0.95, `health check failed` 0.4 | "Which step made the deploy of build 412 fail? Give the file and the line." |
| 4 | `code-change` | `ok` | `retry_wait = 2` 0.9 | "Change the code so that it waits 5 seconds before it tries a failed request again." |
| 5 | `no-key` | `no-key` | none | "Use jag with a question to find where the code waits before it tries a failed request again." |
| 6 | `key-refused` | `refused` | none | "Use jag with a question to find how many days of paid leave new staff get." |
| 7 | `no-credit` | `no-credit` | none | "Use jag with a question to find which step made the deploy of build 412 fail." |
| 8 | `service-down` | `down` | none | "Use jag with a question to find where the code says that the service did not answer." |
| 9 | `not-installed` | `not-installed` | none | "Where does the code wait before it tries a failed request again? Give the file and the line." |

The texts of `scores` are in `tests/readme/sample/` (`src/client.py`,
`notes/leave.md`, `logs/deploy.log`). Tasks 1 to 3 use the scores of the
README examples. A new task is a new entry in the file, with no Rust code.

### One run (CRITERION-128)

`the_agent_uses_jag_well` does these steps:

1. It checks the needs of row 5.
2. It installs the skill into a template folder (row 9).
3. For each task, in the order of the file:
   1. It makes the project folder, the `bin` folder, the jag HOME, and the
      agent HOME, all in one new temp folder.
   2. It copies the sample and `.claude/skills/jag/` into the project
      folder.
   3. It starts the `FakeServer` of the task (row 13).
   4. It writes the wrapper (row 12), except for task `not-installed`. For
      that task, it checks that no `jag` is on the agent PATH.
   5. It runs the agent command (row 10) with the agent environment
      (row 11), and keeps each line of standard output as the transcript.
   6. It checks that the `init` line names the skill `jag` in `skills`.
   7. It builds the task record with the rules of rows 17 to 20.
4. It writes `record.json`, the transcripts, and the wrapper folders under
   `target/agent-use/<unix seconds>/` (row 16).
5. It prints the table.

### The record (CRITERION-130 to CRITERION-133)

`record.json` holds:

- `started`: the unix seconds of the start.
- `jag_version`: the output of `jag --version` of the build.
- `claude_version`: the output of `claude --version`.
- `model`, `skills`: from the `init` line of the first task.
- `task_file`: the text of `tasks.yaml`. Two records then show if two runs
  had the same tasks.
- `total_cost_usd`: the sum of the costs of the tasks.
- `tasks`: one entry for each task, in the order of the file.

Each task entry holds:

- `id`, `prompt`, `forced_code` (null for tasks 1 to 4).
- `agent`: `subtype`, `num_turns`, `total_cost_usd`, and `duration_ms` of the
  `result` line, and `skill_tool_used` (row 18).
- `searches`: the search calls in order, each with `tool` and the command or
  the pattern (row 17).
- `search_verdict` and `first_search` (CRITERION-130).
- `jag_runs`: for each wrapper run, `argv`, `exit_code`, and `limit`
  (row 18).
- `limit_verdict` (CRITERION-131).
- `file_and_line`: `opened`, `cited`, `opened_before_edit` (task
  `code-change` only), and `verdict` (CRITERION-132).
- `failure` (tasks 5 to 9 only): `reached`, `exit_code`, `cause_given`,
  `fix_given`, `jag_runs_after_failure`, and for task `not-installed`
  `install_link_given` and `not_installed_said` (CRITERION-133).
- `final_text`: the `result` text of the agent. The maintainer can read it
  when a rule gives `no` for a text in other words.

The table on standard output has the columns `task`, `search`, `limit`,
`file and line`, `failure`, and `cost`.

### Test strategy

| Criterion | Test | What it proves |
|---|---|---|
| CRITERION-128 | `the_agent_test_has_one_command` | The test reads `tests/agent_use.rs` as text. The `#[ignore]` line of `the_agent_uses_jag_well` and `tests/agent_use/README.md` hold the same command, `cargo test --locked --test agent_use -- --ignored --nocapture`, and the README names its needs and its cost |
| CRITERION-128 | `the_agent_uses_jag_well` (ignored, by hand) | One command runs the 9 tasks and writes the record |
| CRITERION-129 | `the_tasks_are_fixed` | `tasks.yaml` holds 9 tasks, with the ids of the Design in that order, each id one time, a prompt that is not empty, and a known `service`. Two loads give the same list. Each `scores` text is in a file of `tests/readme/sample/` |
| CRITERION-130 | `the_record_shows_the_search_tool` | Made-up transcripts give `jag only`, also for `./bin/jag retry .`; `jag and rg` for a Grep call, then jag; `rg only` for `cd src && rg retry .`; `jag and rg` for `cat x \| grep y` with a jag call; `rg only` for `git grep retry`; and `no search` |
| CRITERION-131 | `the_record_shows_the_limit` | Wrapper runs with `--max-chars 8000`, `--max-chars=8000`, `--no-max-chars`, `JAG_MAX_CHARS=8000` in the command, and no limit give the values and verdicts of row 18 |
| CRITERION-132 | `the_record_shows_the_file_and_the_line` | A Read that holds the lines, a Read with `offset` 0, a Read that does not, a `sed -n` on the file, a cited `path:number`, a cited `./path:number`, a cited number that is not in the result, a run with no result, and, for `code-change`, an Edit before and after the Read give the values of row 19 |
| CRITERION-133 | `the_record_shows_the_cause_and_the_fix` | A final text with the cause and the fix in backticks and with a capital letter gives yes and yes. A text with the cause only gives yes and no. A task with no `--ask` gives `not reached`. Two more jag runs after the failure give the count 2. For `not-installed`, a Bash result `jag: command not found` and a final text with the install link give the values of row 20 |
| Row 12 | `the_wrapper_records_each_jag_call` | The wrapper with the jag of the build and a `FakeServer` answering 401: an argument with a space and a quote is kept in `argv`, `exit` holds 4, `stdout` holds the JSON document, and the wrapper gives the same output and exit code as jag. A second call makes folder `2` |
| Row 25 | `the_transcript_holds_no_key` | The function that writes a transcript replaces a made-up key value in each place with `[ANTHROPIC_API_KEY]` |

The made-up transcripts are short JSON lines in Rust strings, in the form of
the stream-json lines. They hold no real key, no real name, and no address.

## Build order

1. STORY-74: `tests/agent_use/tasks.yaml`, the loader, and
   `the_tasks_are_fixed`.
2. STORY-75: the record rules of rows 17 to 20, with their four tests and
   `the_transcript_holds_no_key`.
3. STORY-74: the wrapper and the fake service of each task, with
   `the_wrapper_records_each_jag_call`.
4. STORY-74: `the_agent_uses_jag_well` (row 21), the record file, and the
   table.
5. STORY-74: `tests/agent_use/README.md` and
   `the_agent_test_has_one_command`.
6. Run `cargo test --locked`, `cargo clippy --locked -- -D warnings`, and
   `cargo fmt --check`.
7. When `ANTHROPIC_API_KEY` is available, run the one command one time and
   put the table in the pull request. When it is not available, the pull
   request says so.

## Assumptions

- Row 3 (the model) is derived from row 2. Its source is the `--model`
  option of the Claude Code CLI reference.
- Row 4 (one ignored cargo test) comes from ADR-0021, `tests/speed.rs`, and
  `tests/common/fake_server.rs`.
- Row 5 (fail on a missing need) comes from ADR-0021 and CV-1.
- Row 6 (the task file) comes from `tests/parity/cases.yaml`,
  `tests/rg_parity.rs`, and ADR-0021.
- Row 7 (the task set) is derived from row 2. It also comes from
  CAPABILITY-11, IF-4, and `skills/jag/SKILL.md`.
- Row 8 (the corpus) comes from `tests/readme_examples.rs` and the FEATURE-27
  spec.
- Row 9 (the skill install) comes from ADR-0024, ADR-0025, and
  `tests/agent_skill/install.test.js`.
- Row 10 (the agent command) is derived from row 2. It also comes from the
  Claude Code docs.
- Row 11 (the agent environment) comes from the Claude Code docs, from
  `tests/common/mod.rs`, and from QS-5.
- Row 12 (the jag wrapper) comes from CV-8, `tests/common/mod.rs`, and
  ADR-0017.
- Row 13 (the fake service) comes from `tests/common/fake_server.rs`,
  `tests/readme_examples.rs`, and `tests/failure.rs`.
- Row 14 (the task text asks for a question) comes from IF-3, DECISION-55,
  and DECISION-56.
- Row 15 (the sources of the record) comes from the Claude Code docs and
  row 12.
- Row 16 (the record file) comes from `.gitignore` and
  `scripts/speed/measure.py`.
- Rows 17 to 20 (the record rules) come from `skills/jag/SKILL.md`,
  DECISION-54, DECISION-55, IF-3, IF-4, IF-5, CV-1, `src/cli.rs`, and the
  Claude Code tools reference.
- Row 21 (when the test fails) comes from FEATURE-26, CRITERION-130 to
  CRITERION-133, and DECISION-54 to DECISION-56.
- Row 22 (the tests that CI runs) comes from ADR-0021 and
  `.github/workflows/ci.yml`.
- Row 23 (the maintainer text) comes from FEATURE-26, `Cargo.toml`, and
  `README.md` `## Speed`.
- Row 24 (no CI job) comes from the scout summary and ADR-0021.
- Row 25 (security) comes from QS-5, `scripts/speed/measure.py`, and the
  Claude Code docs.
- Row 26 (no new crate) comes from ADR-0012 and ADR-0024.

## Risks

| Risk | Signal |
|---|---|
| The agent and the model do not give the same choices in each run. Two runs with the same tasks can give two records | A verdict changes between two runs with no change of the skill or of `MODEL` |
| A later Claude Code version does not load a skill from `--add-dir` in bare mode | The test fails with the cause "the skill jag is not loaded" |
| A later Claude Code version changes the names of the stream-json fields or of the tools | The test fails with "no result line", or the record shows no search for each task |
| The scrub of row 11 does not remove the Anthropic key from the shell of the agent in a later version | `the_transcript_holds_no_key` passes, but a transcript holds `[ANTHROPIC_API_KEY]` |
| A task uses all of its $1 cap | `subtype` of a task is the budget cap, or `total_cost_usd` of a task is near 1 |
| The agent changes a file outside its temp project folder. It runs with the rights of the maintainer | `git status` of the checkout shows a change after a run, or a file outside the temp folder changes |
| The rule of row 20 is too strict when the agent gives the cause in other words | `cause_given` is no, and `final_text` gives the same cause in other words |
| The agent finds the made-up gateway key in the wrapper file and uses it in a command | A Bash call in a transcript holds `SENTINEL` |
| On Linux, the scrub runs each Bash call of the agent in a new PID namespace. A computer that does not let a user make a namespace stops each Bash call | The first transcript shows Bash errors with no output of the command |
| The skill install needs the network, and `claude` installed with npm needs Node on the agent PATH | The install step fails, or the agent does not start |
| The maintainer reads the verdicts as pass or fail. The test gives no bar | A pull request or an issue that cites a run as "the skill fails" with no second run |

## Out of scope

- A CI job, a schedule, or a secret for the agent test (row 24).
- An agent other than Claude Code (row 2).
- A bar that makes the test fail on the choices of the agent (row 21).
- The copied snippet (FEATURE-25). This test installs the skill.
- Exit code 126 (row 7).
- jag-bench and CAPABILITY-12: no roadmap item holds them.
- A change of the skill text: FEATURE-24. The test reads the skill of the
  checkout.

## For the architect

- The test adds three outside needs to one test: the `claude` program, a
  paid `ANTHROPIC_API_KEY`, and a cost of at most $9 for each run. It also
  sets a rule that keeps keys out of the shell of the agent: the scrub of
  row 11 and a jag wrapper that holds only the made-up jag key (row 12).
  After the ANSWER, row 2 binds only this test, so this spec writes no ADR.
  If CI gets a model key later, or if a second test uses the same agent
  runner, that decision is hard to undo and needs an ADR.
