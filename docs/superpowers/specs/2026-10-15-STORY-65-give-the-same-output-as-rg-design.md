---
title: "Give the same output as rg for the same arguments: the parity test set, the e2e tests in CI, and the fixes in jag"
feature: FEATURE-23
stories: [STORY-65, STORY-66, STORY-67]
kind: build
gh_issue: 114
lifecycle_state: draft
inherits: [IF-3, IF-4, IF-5, CV-1, CV-3, CV-4, C-cli, C-search, C-output]
session: 4acb9f7a-4603-41fb-8e74-395e5117e288
---

# Give the same output as rg for the same arguments: the parity test set, the e2e tests in CI, and the fixes in jag

## Problem

An rg command and a jag command with the same arguments and no question must
give the same stdout, the same stderr, the same color, and the same exit code
(FEATURE-23). The user said: "an rg command and a jag command with the same
exact arguments should return exactly the same thing . including the coloring
of the output lets use this as a testing strategy" (user request of
2026-10-15).

The user answered two questions on 2026-10-15 (hitl, FEATURE-23):

- Q1 option 1: rg is the reference for jag with no question. This covers all
  output: the bytes, the color with `--color=always` and in a pseudo terminal,
  the layout, the `<stdin>` label, the error messages, the exit codes, and
  the options.
- Q2 option 1: the parity comes before release 1. FEATURE-18 is blocked by
  FEATURE-23.

The user also answered cards A to E on 2026-10-15, option 1 each (decisions
18 to 22).

- STORY-65: a shell power user (PERSONA-04) wants rg and jag with the same
  arguments to give the same bytes, in a pipe and with color, so that they
  can use jag in place of rg.
  - CRITERION-110: the repository holds the parity test set: a list of rg
    argument sets and the fixture files that each case uses. This spec gives
    the first list. Each case uses jag with no question.
  - CRITERION-111: for each case, `rg <args>` and `jag <args>` on the same
    fixture files give byte-identical stdout, byte-identical stderr, and the
    same exit code when the output goes to a pipe.
  - CRITERION-112: for each case, rg and jag give byte-identical stdout with
    the color codes, byte-identical stderr, and the same exit code in two
    modes: with `--color=always` in a pipe, and in a pseudo terminal.
- STORY-66: the same user wants the parity cases to run again as automated
  e2e tests in CI, so that a later change cannot break the parity with no
  failed check.
  - CRITERION-113: CI installs rg at a fixed version and runs each case on
    each pull request. A difference in stdout, stderr, or exit code fails the
    check, and the check output shows the difference. For a deliberate case,
    the check compares jag with the recorded jag output. The check runs rg
    with no rg config file (`RIPGREP_CONFIG_PATH` is not set).
- STORY-67: the same user wants each difference from rg to be fixed in jag
  or recorded as deliberate by a user answer, so that each difference that
  stays is known.
  - CRITERION-114: each difference that the parity test finds is fixed in
    jag, or a user ANSWER records it as a deliberate difference. For a
    deliberate difference, the test set names the ANSWER, and the e2e test
    checks the recorded jag output for that case.

Today jag gives the grep form. A probe of jag on `main` (`55b3652`) against
rg 15.2.0 ran 42 cases in 3 modes. Only 15 of 126 runs gave the same bytes,
and only after the probe removed the cost line of jag, read `rg:` as `jag:`,
and sorted the files of rg. Each of these 15 runs had no matching line, or
used `-q` or `-l`. The section "The differences" lists each difference that
the probe found.

## Decisions

The user answered cards A to E through hitl on 2026-10-15, option 1
each. ADR-0021 and ADR-0022 call the cards A to D. This spec gives both
names.

| Card | Spec name | Subject |
|---|---|---|
| A | F23-Q3 | jag's own name, help, version, and options |
| B | F23-Q4 | jag's notices and limits with no question |
| C | F23-Q5 | the rg options that jag supports |
| D | F23-Q6 | the JSON output with no question |
| E | F23-Q7 | rg hints that name options that jag does not have |

| # | Decision | Choice | Source |
|---|---|---|---|
| 1 | Specs | One spec for STORY-65, STORY-66, and STORY-67. The three stories share one test set and one list of decisions | Orchestrator TASK for FEATURE-23 |
| 2 | The reference | rg 15.2.0, with no rg config file. For the same arguments with no `--ask`, jag gives the same stdout, stderr, color codes, and exit code as rg. A difference stays only when a user ANSWER records it | User ANSWER of 2026-10-15 (Q1 option 1); ADR-0022 (merged in `ee7debb`, status `accepted` (PR #129)); the pin of ADR-0021 (merged in `a9f966d`, status `accepted` (PR #129)) |
| 3 | The scope | A search with no `--ask`: text mode and `--json` mode. `--setup`, `--check`, and `--forget-key` are not searches, and no case runs them. The ranked layout of `--ask` does not change | ADR-0022, Consequences; DECISION-31 |
| 4 | Color codes in all text modes | The rg color codes also in the ranked text of `--ask`: path `ESC[0m ESC[35m`, line number `ESC[0m ESC[32m`, match `ESC[0m ESC[1m ESC[31m`, each closed with `ESC[0m`. The separators get no color | DECISION-38 (PR #121) names no mode; probe of rg 15.2.0 |
| 5 | The `<stdin>` label in all modes | `src/search.rs` `STANDARD_INPUT_LABEL` becomes `<stdin>`. This changes the plain text, the ranked text, the `path` of the JSON document, and the path in the snippets that go to the model | DECISION-18 (PR #121) names no mode |
| 6 | Color auto rule | As rg: color only when stdout is a terminal, `TERM` is set and is not `dumb`, and `NO_COLOR` is not set. This applies to all text modes | DECISION-38; rg 15.2.0 `crates/core/flags/defs.rs` (the `--color` help) |
| 7 | The printer of plain text mode | jag writes the plain text output with `grep-printer` 0.3.1 and `termcolor` 1.4.1, with the settings that rg 15.2.0 uses. These are the versions in the `Cargo.lock` of rg 15.2.0. ADR-0001: no change (architect, 2026-10-15). The new crates follow ADR-0012 (a reason in the PR): the implementer gives the reason in the body of the code PR | A spike gave byte-identical output to `rg --sort path` in 14 of 14 layouts; architect answer of 2026-10-15 (the crates write output and do not find matches; ADR-0022 holds the output contract); ADR-0012 (a new outside crate needs a reason in the pull request) |
| 8 | The search of plain text mode | A new sequential pipeline, as rg: the walk of `ignore` with the settings of `HiArgs::walk_builder` of rg 15.2.0, sorted by file name, with one thread. `grep-searcher` searches each file. The printer writes the result of each file to stdout when that file is complete. The ranked search and `--json` keep the scan of `src/search/walk.rs` | rg 15.2.0 `crates/core/flags/hiargs.rs` and `crates/core/main.rs`; decision 7 |
| 9 | Order of files | Paths of the command line in their order. Inside a folder, the order of `ignore` `sort_by_file_name`: by name, folder by folder. So `ord/sub/x.txt` comes before `ord/sub.txt`. jag uses this order also with no `--sort` | Probe: rg with more than one file and no `--sort` gives a different order in each run; rg with `--sort path` gives this order |
| 10 | Cases with more than one file | A case that can match in more than one file has `--sort path` in its arguments, or the case searches one file | ADR-0021, open point "Order"; decision 9 |
| 11 | Error texts | The rg 15.2.0 text of each error, in the form that rg gives with one thread. The section "The error texts" lists the forms | Q1; probe: rg gives a shorter path error with more than one thread, and `--sort path` or one file gives one thread |
| 12 | Exit codes of a search with no `--ask` | As rg 15.2.0 (`crates/core/main.rs`): 0 when a line matched and (`-q` is on, or no error happened); 2 when an error happened; else 1. An error is any path or file that jag cannot read, not only a path that does not exist. A usage error gives 2. Ctrl-C gives 130 | Q1; probe: a file with no read permission gives rg exit 2 and jag exit 0 |
| 13 | Streaming and Ctrl-C | In plain text mode, jag writes the result of each file when that file is complete, as rg does. On Ctrl-C, jag stops and exits with 130, and the text that it already wrote stays. The "no partial result" rule of CV-4 stays for `--ask` and `--json` | Q1; decision 8 |
| 14 | `-` as a path | `-` reads stdin, as rg does. Its label is `<stdin>`. An empty path gives the rg path error, not a usage error | Q1; probe: `rg foo -` and `rg foo ''`. This changes FEATURE-14 decision 8 |
| 15 | Binary files | As rg: a file that the command line names gets the binary rule "convert" of `grep-searcher`, and the printer writes `binary file matches (found "\0" byte around offset N)`. A file of a walk gets the rule "quit", with no message | Probe; rg 15.2.0 `crates/core/flags/hiargs.rs` |
| 16 | Ignore files | As rg: `.ignore`, `.rgignore`, and `.gitignore` (only inside a git repository), and the ignore files of the parent folders. Hidden files are skipped in a walk. A path of the command line is always searched | Probe: jag does not read `.rgignore` today; rg 15.2.0 `walk_builder` |
| 17 | Usage error text with `--ask` or `--json` | The rg error texts apply when the command line has no `--ask` and no `--json` before the first `--`. Else jag keeps its own failure texts with the cause and the fix (CV-1), and the JSON error document with `--json` | ADR-0022 covers a search with no `--ask`; CV-1; IF-5 |
| 18 | Program name, help, version, and own options (card A) | Decided, option 1: jag keeps its name, `-h`, `--help`, `-V`, `--version`, and its own options (`--ask`, `--json`, `--check`, `--setup`, `--forget-key`, `--max-chars`, `--no-max-chars`, `--max-scored`, `--timeout`, `--retry-wait`, `--base-url`, `--model`). The test reads "jag" and "rg" or "ripgrep" as the same name in an error line: `rg:` at the start of an rg stderr line reads as `jag:`, and each `ripgrep` in rg stderr reads as `jag`. It changes no other byte, nothing in the stdout of rg, and nothing in the output of jag | Card A (F23-Q3), user ANSWER via hitl, 2026-10-15, option 1; ADR-0021, section "Run", in PR #127 (head `778d4c3`) |
| 19 | Notices and limits with no question (card B) | Decided, option 1: in plain text mode, jag writes no cost line, no count of skipped files, cuts no long line, and has no default size limit. It writes full lines, no notice, and the rg exit codes (decision 12). `--max-chars N` still works when the user gives it, with its size notice on stderr and exit code 8. With no question, no rank exists, so "the lowest rank first" of DECISION-21 does not apply: jag writes whole results in the rg order (decision 9) and stops at the limit. It writes the first results while they fit in N characters, and leaves out each result after them. A result is one match line, or one group of lines with `-A`, `-B`, or `-C`. When the first result alone is larger than N, jag cuts it as DECISION-23 says. `--no-max-chars` is accepted and changes nothing. `--json` and `--ask` keep all four | Card B (F23-Q4), user ANSWER via hitl, 2026-10-15, option 1. The planner changes DECISION-14, DECISION-17, DECISION-20, DECISION-41, and DECISION-43 |
| 20 | The option set (card C) | Decided, option 1: the current set in the rg form. `-g`/`--glob` replace `--include` and `--exclude`. New: `-n`, `-N`, `-H`, `-I`, `--heading`, `--no-heading`, and `--sort` with `path` or `none`. `--color` takes its value as rg does (`--color always` and `--color=always`), with no `--colour`. Each other option gives the rg text `unrecognized flag`, with exit 2. `--sort` with another value gives the rg text `error parsing flag --sort: choice 'X' is unrecognized`, with exit 2. rg accepts `modified`, `accessed`, and `created`, so these are recorded differences | Card C (F23-Q5), user ANSWER via hitl, 2026-10-15, option 1. The card names `--sort path`; `--sort none` gives the same output as rg (Q1), and ADR-0021 in PR #127 (head `778d4c3`) names `path` and `none`. The planner changes DECISION-34 |
| 21 | JSON with no question (card D) | Decided, option 1: `--json` keeps the one JSON document of jag, as a recorded difference. The case `json` records it | Card D (F23-Q6), user ANSWER via hitl, 2026-10-15, option 1; ADR-0008 |
| 22 | rg hints (card E) | Decided, option 1: jag gives the rg message with the same first part and the same exit code, with no sentence that names an option that jag does not have (`--pcre2`, `--multiline`, `-U`, `--debug`). The list of similar options after `unrecognized flag` comes from jag's own options, with the rule of rg. These are recorded differences | Card E (F23-Q7), user ANSWER via hitl, 2026-10-15, option 1 |
| 23 | Layout options with `--ask` or `--json` | `-n`, `-N`, `-H`, `-I`, `--heading`, and `--no-heading` are a usage error with `--ask` or `--json`, as `-l`, `-c`, and `-q` are today. `-g` and `--sort` work in all modes. `--sort` has no effect on the ranked order | Inferred from `src/cli.rs` (`conflicts_with_all = MODES`) |
| 24 | The test form | ADR-0021: one cargo integration test `tests/rg_parity.rs`, the cases in `tests/parity/cases.yaml`, the fixture trees in `tests/parity/fixtures/`, the stored jag bytes in `tests/parity/expected/`, the pin in `tests/parity/rg.sha256`, and `scripts/parity/install-rg.sh`. Three modes: pipe, color, and terminal | ADR-0021 (merged in `a9f966d`, status `accepted` (PR #129)); the round of the user ANSWERs in PR #127 (head `778d4c3`) |
| 25 | The first test set | The 76 cases of the section "The first test set", with the fixture trees of the section "The fixture trees" | CRITERION-110; the probes of 2026-10-15 |
| 26 | Stdin files of the cases | `tests/parity/stdin/<name>`. A case names its file with `stdin: <name>` | ADR-0021 in PR #127 (head `778d4c3`) |
| 27 | Owners and pull requests | Two pull requests. The tester opens the test PR (STORY-65 and STORY-66). The implementer opens the code PR (STORY-67). The verifier runs the cases on the code PR. See "Owners" | Orchestrator TASK ("The tester codifies the tests; the verifier runs the cases"); `docs/adlc.md`, the roles |
| 28 | Merge order | The code PR merges first. The test PR merges after it, so `main` never has a parity test that fails | Decision 27; `cargo test` stops after the first test binary that fails, so a red parity test hides the results of other tests |
| 29 | Done | All three stories are done only after both pull requests merge and the parity test passes in the `rust` job on `main` | `docs/adlc.md` ("does not block done, unless the spec says so") |
| 30 | Story state | STORY-65, STORY-66, and STORY-67 are `ready`: this spec is in this PR, no feature blocks them, and the ANSWERs of cards A to E are in. The ADR round of the ANSWERs is open in PR #127 (head `778d4c3`) | `docs/adlc.md` rule 11; orchestrator messages of 2026-10-15 |

## Design

### Owners

| Step | Owner |
|---|---|
| Write this spec; set the stories `ready` | specifier |
| Change the PRD pages and the system design for the ANSWERs (see "For the planner") | planner |
| The ADR round of the ANSWERs (PR #127 (head `778d4c3`)) | architect |
| The test PR: the harness, the test set, the fixtures, the pin, the install script, and the CI steps (STORY-65, STORY-66) | tester |
| The code PR: each fix of "The differences" (STORY-67) | implementer |
| Run the cases on the code PR with the pinned rg; send test scenarios | verifier |
| Review each PR, and do the ADR check | reviewer |
| Merge order (decision 28) | orchestrator |

### For the planner

These PRD and system design texts disagree with Q1 or with this spec. The
planner changes them after the ANSWERs:

- CV-1: "Each failure gives its cause, its fix, and its exit code." rg error
  texts have no fix. Q1 wins with no `--ask` (decision 17).
- CV-4: "writes no partial result" stays for `--ask` and `--json` only
  (decision 13).
- IF-4: exit 2 for any read error with no `--ask`, not only for a path that
  does not exist (decision 12). With no question, exit 8 comes only with
  `--max-chars` (card B).
- IF-3 and DECISION-34: the option list of card C (decision 20).
- FEATURE-14 decision 8 (`-` and the empty path are usage errors): decision
  14 changes it.
- DECISION-14, DECISION-17, DECISION-20, DECISION-41, and DECISION-43: card B
  (decision 19).

### For the architect

PR #127 (head `778d4c3`) holds the ADR round of the ANSWERs:

- ADR-0021: the name rule of card A (decision 18), the folder
  `tests/parity/stdin/` and its `.gitattributes` line (decision 26), and the
  point "Order" (decision 10).
- ADR-0022: the exceptions of cards A to E.
- ADR-0001: no change (architect, 2026-10-15). The new crates follow
  ADR-0012 (a reason in the PR).

### Files

| File | Change | Owner |
|---|---|---|
| `Cargo.toml`, `Cargo.lock` | Add `grep-printer = "=0.3.1"` and `termcolor = "=1.4.1"` | implementer |
| `src/cli.rs` | New plain text pipeline (decision 8); exit codes (decision 12); color auto rule (decision 6); options (decisions 20 and 23); `-` and the empty path (decision 14) | implementer |
| `src/cli/grep.rs` | The option code of decision 20; the rg error texts | implementer |
| `src/cli/rg_text.rs` (new) | The rg error texts and the similar-option rule (decisions 11 and 22) | implementer |
| `src/search/sequence.rs` (new) | The haystacks in rg order: paths of the command line, `-`, and the walk with the rg settings (decisions 8, 9, 15, 16) | implementer |
| `src/search.rs` | `STANDARD_INPUT_LABEL` = `<stdin>` (decision 5) | implementer |
| `src/output/printer.rs` (new) | The `grep-printer` builders with the rg settings (decision 7) | implementer |
| `src/output.rs` | The rg color codes in the ranked text (decision 4); no cost line and no notices in plain text mode, but the size notice of `--max-chars` (decision 19) | implementer |
| `src/search/filter.rs`, `src/search/walk.rs` | The `--include` and `--exclude` globs become `-g`/`--glob` (decision 20) | implementer |
| `tests/*.rs` | Each test that checks the grep form now checks the rg form | implementer |
| `README.md` | The plain search texts: the `<stdin>` label, `-` as stdin, the color rule, the exit codes, the options, and the notices (decisions 5, 6, 12, 14, 19, and 20) | implementer |
| `tests/rg_parity.rs` (new) | The harness of ADR-0021 | tester |
| `Cargo.toml` `[dev-dependencies]` | `rustix` with the features `pty` and `termios` (ADR-0021) | tester |
| `tests/parity/cases.yaml` (new) | The first test set | tester |
| `tests/parity/fixtures/` (new) | The fixture trees | tester |
| `tests/parity/stdin/` (new) | The stdin files (decision 26) | tester |
| `tests/parity/expected/` (new) | The stored jag bytes of the deliberate cases | tester |
| `tests/parity/rg.sha256` (new) | The pin of ADR-0021 | tester |
| `scripts/parity/install-rg.sh` (new) | Download, check, and extract rg | tester |
| `.gitattributes` (new) | `tests/parity/fixtures/** -text`, `tests/parity/expected/** -text`, and `tests/parity/stdin/** -text` | tester |
| `.github/workflows/ci.yml`, `.github/workflows/release.yml` | Run `scripts/parity/install-rg.sh` and set `JAG_PARITY_RG` before `cargo test` | tester |

### The differences

Each row is one difference that the probe found. "Fix" means that the code PR
changes jag to the rg form. "Recorded difference" means that a user ANSWER
keeps the jag form.

| # | What | rg 15.2.0 | jag on `main` | Action |
|---|---|---|---|---|
| D1 | One file, pipe | `beta foo` (no path, no line number) | `a.txt:2:beta foo` | Fix |
| D2 | More than one file, pipe | `a.txt:beta foo` | `a.txt:2:beta foo` | Fix |
| D3 | Terminal | Heading: the colored path on its own line, `N:text`, and an empty line between files. One file: no heading, with line numbers | grep form | Fix |
| D4 | Color codes | Decision 4 | `ESC[35m ESC[K ... ESC[m ESC[K`, cyan separators, `01;31` | Fix |
| D5 | Color auto | `NO_COLOR` and `TERM` (decision 6) | Only `TERM=dumb` | Fix |
| D6 | Order | Decision 9 | By the bytes of the full path | Fix |
| D7 | `-c` | One file: only the count. Files with 0 are not shown | `a.txt:3` and `empty.txt:0` | Fix |
| D8 | `-l` and `-c` with color | rg color codes on the path | grep codes | Fix |
| D9 | Binary file of the command line | `binary file matches (found "\0" byte around offset 3)` on stdout, exit 0 | Skipped, exit 1 | Fix |
| D10 | Stdin label | No label with stdin alone; `<stdin>` with other files | `(standard input):1:` | Fix |
| D11 | `-` and the empty path | `-` reads stdin; `''` gives the path error, exit 2 | Usage error | Fix |
| D12 | `.rgignore` | Read | Not read | Fix |
| D13 | Read error | `rg: u.txt: Permission denied (os error 13)`, exit 2 | `jag: cannot read u.txt: permission denied`, exit 0 | Fix |
| D14 | Path that does not exist | `rg: nope.txt: IO error for operation on nope.txt: No such file or directory (os error 2)` | `jag: the path nope.txt does not exist` | Fix (name rule of card A) |
| D15 | Bad pattern | The `regex parse error:` block of rg | jag's two lines | Fix (name rule of card A) |
| D16 | Usage errors | `rg: unrecognized flag --frobnicate`, `rg: error parsing flag -A: ...`, `rg: missing value for flag -A: ...` | jag's two lines | Fix (name rule of card A) |
| D17 | No arguments | `rg: ripgrep requires at least one pattern to execute a search`, exit 2 | Help text | Fix (name rule of card A) |
| D18 | Nothing searched | `rg: No files were searched, ...` and a `--debug` line, exit 2 | Exit 1 | Fix; the `--debug` line is a recorded difference (card E) |
| D19 | Hints | `--pcre2` and `--multiline` hints | No hint | Recorded difference (card E) |
| D20 | Cost line and skip notice on stderr | None | Always | Fix (card B) |
| D21 | Long line | Full line | Cut with ` [+N chars]` | Fix (card B) |
| D22 | Size limit | None, exit 0 | 20,000 characters, notice, exit 8 | Fix (card B) |
| D23 | `--include`, `--exclude`, `-g`, `-n`, `-N`, `-H`, `-I`, `--heading`, `--sort`, `--colour`, `--color VALUE` | rg form | jag form | Fix (card C). Each rg option that card C does not keep is a recorded difference |
| D24 | `-h`, `--help`, `-V`, `--version`, jag-only options | rg text | jag text | Recorded difference (card A) |
| D25 | `--json` | JSON lines with times | One document | Recorded difference (card D) |

### The plain text pipeline (decisions 7, 8, 9, 12, 13, 15, 16)

`cli::plain` keeps its place in the flow. In text mode with no `--ask` and no
`--json`, it runs the new pipeline:

1. `search::sequence` gives the haystacks in order (decision 9): each path
   of the command line, `-` as stdin, and for a folder the files of the walk.
   The walk uses `ignore::WalkBuilder` with the settings of
   `HiArgs::walk_builder` of rg 15.2.0, `.rgignore` as a custom ignore file
   name, `sort_by_file_name(|a, b| a.cmp(b))`, and one thread. With no path,
   jag searches stdin when stdin can give text (`search::input`), else the
   current folder, with no `./` before the paths.
2. For each haystack, `grep-searcher` searches with the rg settings: line
   terminator `\n`, the binary rule of decision 15, and the matcher of
   `build_matcher`.
3. `output::printer` builds the printer with the rg 15.2.0 settings of
   `HiArgs::printer_standard` and `HiArgs::printer_summary`: the default
   color specs of `grep-printer`, the heading when stdout is a terminal, the
   path when more than one haystack can match, a line number when stdout is
   a terminal and the input is not stdin alone, the separators `--`, `-`,
   and `:`, one line for each match, and the per-file separator rule of rg.
   `-n`, `-N`, `-H`, `-I`, `--heading`, and `--no-heading` change these
   defaults as in rg. `-l`, `-c`, and `-q` use the summary printer with the
   kinds `PathWithMatch`, `Count`, and `Quiet`.
4. The printer writes into a buffer for each haystack. When the search of
   the haystack is complete, jag writes the buffer to stdout. With
   `--max-chars N`, jag writes whole results in this order until the next
   result does not fit, and then stops (decision 19).
5. jag records `matched` and `errored`, and gives the exit code of decision
   12. A closed pipe ends the search with no message.

The implementer copies each setting from the rg 15.2.0 source and names the
source file in a code comment. The spike settings that gave the same bytes:
`grep_printer::StandardBuilder` with `heading(tty)`, `path(with_filename)`,
`per_match_one_line(true)`, `separator_context(Some(b"--"))`,
`separator_field_context(b"-")`, `separator_field_match(b":")`,
`separator_search(b"")` in heading mode with no context lines, and
`BinaryDetection::convert(b'\x00')` for a file of the command line.

### The error texts (decisions 11, 17, 18, 22)

With no `--ask` and no `--json`, jag writes these texts to stderr. `<name>`
is `jag` (card A, option 1).

| Error | Text | Exit |
|---|---|---|
| Path that does not exist | `<name>: P: IO error for operation on P: No such file or directory (os error 2)` | 2 |
| Path that jag cannot read | `<name>: P: Permission denied (os error 13)` | 2 |
| Empty path | `<name>: : IO error for operation on : No such file or directory (os error 2)` | 2 |
| Bad pattern | `<name>: regex parse error:` then the lines of the `grep-regex` error | 2 |
| Unknown option | `<name>: unrecognized flag --X`, then an empty line and `similar flags that are available: ...` when the rule of rg finds similar names in jag's options | 2 |
| Bad value | `<name>: error parsing flag -A: value is not a valid number: invalid digit found in string` | 2 |
| Bad choice | `<name>: error parsing flag --color: choice 'X' is unrecognized` | 2 |
| Missing value | `<name>: missing value for flag -A: missing argument for option '-A'` | 2 |
| No pattern | `<name>: <name> requires at least one pattern to execute a search` | 2 |
| Nothing searched | `<name>: No files were searched, which means <name> probably applied a filter you didn't expect.` (card E option 1 drops the `--debug` line) | 2 |
| Look-around or `\n` in the pattern | The rg text with no hint sentence (card E option 1) | 2 |

clap stays the parser (ADR-0012). `src/cli/rg_text.rs` maps each clap error
kind to the rg text. The rg source of each text is
`crates/core/flags/parse.rs` and `crates/core/main.rs` of rg 15.2.0. The
similar-option rule is `find_similar_names` of `parse.rs` (a Jaccard index
of letter groups).

### The first test set (CRITERION-110)

Each case runs in the three modes of ADR-0021 (pipe, color, and terminal),
unless the column "Modes" says other. The column "Result" says what the
case must give. `rg`: the rg bytes, with the name rule of decision 18 on
stderr. `deliberate: X`: the stored jag bytes, with the ANSWER of card X
(CRITERION-114).

| Id | Arguments | Tree | Modes | Stdin or env | Result |
|---|---|---|---|---|---|
| one-file | `foo a.txt` | basic | all | | rg |
| two-files | `--sort path foo a.txt sub/c.txt` | basic | all | | rg |
| folder | `--sort path foo sub` | basic | all | | rg |
| dot | `--sort path foo .` | basic | all | | rg |
| no-path | `--sort path foo` | basic | all | | rg |
| args-order | `--sort path foo sub/c.txt a.txt b.rs` | basic | all | | rg |
| walk-order | `--sort path x .` | order | all | | rg |
| no-match | `zzz a.txt` | basic | all | | rg |
| ignore-case | `-i foo a.txt` | basic | all | | rg |
| word | `-w foo a.txt` | basic | all | | rg |
| fixed | `-F f.o a.txt` | basic | all | | rg |
| regex | `f.o a.txt` | basic | all | | rg |
| two-e | `-e alpha -e gamma a.txt` | basic | all | | rg |
| e-dash | `-e -x a.txt` | basic | all | | rg |
| double-dash | `-- -x a.txt` | basic | all | | rg |
| empty-term | `'' a.txt` | basic | all | | rg |
| after | `-A 1 foo ctx.txt` | basic | all | | rg |
| before | `-B 2 foo ctx.txt` | basic | all | | rg |
| context | `-C 1 foo ctx.txt` | basic | all | | rg |
| context-two | `--sort path -C 1 foo a.txt ctx.txt` | basic | all | | rg |
| files | `--sort path -l foo a.txt b.rs empty.txt` | basic | all | | rg |
| count-one | `-c foo a.txt` | basic | all | | rg |
| count-many | `--sort path -c foo a.txt b.rs empty.txt` | basic | all | | rg |
| quiet | `-q foo a.txt` | basic | all | | rg |
| quiet-miss | `-q zzz a.txt` | basic | all | | rg |
| crlf | `foo crlf.txt` | basic | all | | rg |
| no-newline | `foo nonl.txt` | basic | all | | rg |
| utf8 | `foo utf8.txt` | basic | all | | rg |
| latin1 | `foo latin1.txt` | basic | all | | rg |
| hidden-file | `foo .hidden.txt` | basic | all | | rg |
| binary-file | `foo bin.dat` | basic | all | | rg |
| binary-late | `foo late.bin` | binary | all | | rg |
| binary-walk | `--sort path foo .` | binary | all | | rg |
| ignore-files | `--sort path foo .` | ignore | all | | rg |
| stdin | `foo` | basic | all | stdin `lines.txt` | rg |
| stdin-dash | `foo -` | basic | all | stdin `lines.txt` | rg |
| stdin-and-file | `--sort path foo - a.txt` | basic | all | stdin `lines.txt` | rg |
| color-space | `--color always foo a.txt` | basic | pipe | | rg |
| color-never | `--color never foo a.txt` | basic | terminal | | rg |
| no-color-env | `foo a.txt` | basic | terminal | `NO_COLOR=1` | rg |
| term-dumb | `foo a.txt` | basic | terminal | `TERM=dumb` | rg |
| missing-path | `foo nope.txt` | basic | all | | rg |
| missing-and-match | `--sort path foo nope.txt a.txt` | basic | all | | rg |
| quiet-missing | `-q --sort path foo nope.txt a.txt` | basic | all | | rg |
| empty-path | `foo ''` | basic | all | | rg |
| bad-regex | `'(' a.txt` | basic | all | | rg |
| no-args | (none) | basic | pipe | | rg |
| bad-number | `-A x foo a.txt` | basic | pipe | | rg |
| missing-value | `foo a.txt -A` | basic | pipe | | rg |
| color-bad | `--color foo a.txt` | basic | pipe | | rg |
| sort-bad | `--sort size foo a.txt` | basic | pipe | | rg |
| look-around | `'a(?=b)' a.txt` | basic | pipe | | deliberate: E |
| newline | `'a\nb' a.txt` | basic | pipe | | deliberate: E |
| nothing-searched | `foo` | hidden-only | pipe | | deliberate: E |
| unknown-option | `--frobnicate foo a.txt` | basic | pipe | | deliberate: E |
| glob | `--sort path -g '*.txt' foo .` | basic | all | | rg |
| glob-not | `--sort path -g '!*.txt' foo .` | basic | all | | rg |
| include | `--include '*.txt' foo sub` | basic | pipe | | deliberate: E |
| colour | `--colour=always foo a.txt` | basic | pipe | | deliberate: E |
| line-number | `-n foo a.txt` | basic | all | | rg |
| no-line-number | `-N --sort path foo a.txt sub/c.txt` | basic | all | | rg |
| with-filename | `-H foo a.txt` | basic | all | | rg |
| no-filename | `-I --sort path foo a.txt sub/c.txt` | basic | all | | rg |
| heading | `--heading --sort path foo a.txt sub/c.txt` | basic | all | | rg |
| no-heading | `--no-heading --sort path foo a.txt sub/c.txt` | basic | all | | rg |
| sort-none | `--sort none foo a.txt` | basic | all | | rg |
| sort-modified | `--sort modified foo a.txt` | basic | pipe | | deliberate: C |
| rg-only-option | `-v foo a.txt` | basic | pipe | | deliberate: C |
| long-line | `foo long.txt` | limits | all | | rg |
| many | `foo many.txt` | limits | all | | rg |
| limits-walk | `--sort path foo .` | limits | all | | rg |
| max-chars | `--max-chars 1000 foo many.txt` | limits | pipe | | deliberate: A |
| help-short | `-h` | basic | pipe | | deliberate: A |
| help-long | `--help` | basic | pipe | | deliberate: A |
| version | `--version` | basic | pipe | | deliberate: A |
| json | `--json foo a.txt` | basic | pipe | | deliberate: D |

The words in the column "Arguments" are shell words: `''` is one empty
argument, and `'a\nb'` is the 4 characters `a`, `\`, `n`, `b`. The tester
writes each argument as one YAML string.

A deliberate case gets `deliberate: { answer: <the ANSWER> }` and stored jag
bytes (CRITERION-114). The `answer` names the date, the card, and the option,
for example "hitl ANSWER of 2026-10-15, card C (F23-Q5), option 1". If rg
and jag give the same bytes for a `deliberate: E` case (for example, the
same list of similar options), the test fails (ADR-0021), and the tester
changes the result of that case to `rg`.

The stored bytes of the case `version` hold the version of `Cargo.toml`.
The pull request that changes that version also changes
`tests/parity/expected/` for this case.

### The fixture trees

A name that starts with `.` is stored with `dot-` in its place (ADR-0021).
The text of each file is given with `\n` for a new line and `\0` for a NUL
byte. No fixture holds a private name. No fixture name, path, or argument
contains `rg:` or `ripgrep`, so the name rule of decision 18 hides no
difference.

| Tree | File | Bytes |
|---|---|---|
| basic | `a.txt` | `alpha\nbeta foo\ngamma\nfoo foo\nFoo bar\nfood\n` |
| basic | `b.rs` | `fn main() {\n    let foo = 1;\n    println!("{}", foo);\n}\n` |
| basic | `empty.txt` | no bytes |
| basic | `sub/c.txt` | `nothing\nfoo here\n` |
| basic | `sub/deeper/d.md` | `# foo title\ntext\n` |
| basic | `dot-hidden.txt` | `foo hidden\n` |
| basic | `dot-gitignore` | `ignored.txt\n` (no `.git` folder, so rg and jag do not use it) |
| basic | `ignored.txt` | `foo ignored\n` |
| basic | `bin.dat` | `foo\0bar\n` |
| basic | `crlf.txt` | `foo one\r\nbar\r\nfoo two\r\n` |
| basic | `nonl.txt` | `foo at the end` with no `\n` at the end |
| basic | `utf8.txt` | `café foo über\n` in UTF-8 |
| basic | `latin1.txt` | `caf`, the byte `0xE9`, ` foo\n` |
| basic | `ctx.txt` | 40 lines `line N` (N from 1 to 40). Lines 5, 6, 12, and 30 end with ` foo` |
| limits | `long.txt` | 3,000 `x`, ` foo `, 100 `y`, `\n` |
| limits | `many.txt` | 500 lines: `foo `, the line number with 5 digits from `00000`, a space, 40 `z`, `\n` |
| order | `B.txt`, `a.txt`, `_u.txt`, `Z.txt`, `a-b.txt`, `a.b.txt`, `10.txt`, `9.txt` | `x in <name>\n` |
| order | `sub/x.txt` | `x in sub\n` |
| order | `sub.txt` | `x in sub.txt\n` |
| ignore | `dot-rgignore` | `r.txt\n` |
| ignore | `dot-ignore` | `i.txt\n` |
| ignore | `r.txt`, `i.txt`, `k.txt` | `foo r\n`, `foo i\n`, `foo k\n` |
| binary | `t.txt` | `foo text\n` |
| binary | `b.bin` | `foo\0`, the bytes `0x01 0x02`, ` binary\n` |
| binary | `late.bin` | 2,000 lines `foo line\n`, then `\0 late nul\n` |
| hidden-only | `dot-h/x.txt` | `foo\n` |

The stdin file `tests/parity/stdin/lines.txt` holds `a foo\nb\nfoo c\n`.

### Acceptance criteria and their tests

| Criterion | Test |
|---|---|
| CRITERION-110 | `tests/parity/cases.yaml` holds each case of "The first test set", and `tests/parity/fixtures/` holds each tree of "The fixture trees". `tests/rg_parity.rs` fails when a case names a tree or a stdin file that does not exist. The reviewer compares the two lists with this spec |
| CRITERION-111 | `tests/rg_parity.rs`, pipe mode: for each case, the same stdout, stderr, and exit code as the pinned rg, or the stored bytes of a deliberate case |
| CRITERION-112 | `tests/rg_parity.rs`, color mode (`--color=always` in a pipe) and terminal mode (stdout on a pseudo terminal) |
| CRITERION-113 | The `rust` job of `ci.yml` (Linux and macOS) and the `build` job of `release.yml` run `scripts/parity/install-rg.sh`, set `JAG_PARITY_RG`, and run `cargo test --locked`. The environment of each run is empty, plus `HOME`, `TERM`, and the env of the case (ADR-0021), so `RIPGREP_CONFIG_PATH` is not set. In the test PR, the tester pushes one commit that changes one stored byte, links the failed run that shows the difference, and then pushes a second commit that reverts it |
| CRITERION-114 | Each case passes against rg, or has `deliberate: { answer: ... }` that names a user ANSWER (date, card, and option). The test checks that jag gives the stored bytes and that rg does not. The reviewer checks each `answer:` against the ANSWER of the card |
| Decisions 12 and 13 (no parity case) | The code PR tests them in `tests/`: a test makes a file with mode 000 in a temporary folder and checks the rg text and exit 2; a second test sends SIGINT during a plain search and checks exit 130 and the text that jag already wrote. A test that runs as root skips the mode 000 check |

The implementer also changes the unit tests and the integration tests of
`tests/` that check the grep form: at least `tests/plain_grep.rs`,
`tests/grep_options.rs`, `tests/input.rs`, `tests/terminal.rs`,
`tests/walk.rs`, `tests/usage.rs`, `tests/size_limit.rs`, `tests/cost.rs`,
and `tests/output.rs`, and the unit tests of `src/cli.rs`,
`src/cli/grep.rs`, `src/output.rs`, `src/search/walk.rs`,
`src/search/snippet.rs`, `src/scorer.rs`, and `src/scorer/prompt.rs` (each
test that checks `(standard input)`).

## Build order

1. Done on 2026-10-15: the ANSWERs of cards A to E are in this spec, and
   this PR sets the stories `ready` (rule 11). Before the code PR and the
   test PR merge, PR #127 (the ADR round of the ANSWERs) merges. ADR-0021
   and ADR-0022 are accepted (PR #129). The planner changes the PRD pages ("For
   the planner").
2. Tester (STORY-65, STORY-66): the test PR from `main`, with each file of
   the tester in "Files". The parity test fails on this PR until step 3
   merges. The PR body lists each case and mode that differs. This list is
   the work list of step 3. The PR does not merge before step 3.
3. Implementer (STORY-67): the code PR from `main`, with each fix of "The
   differences". To run the parity test, the implementer and the verifier
   use a local branch that merges the code branch and the test branch, with
   rg from `scripts/parity/install-rg.sh`, not the rg of the machine. They
   push no such branch.
4. Verifier: runs each case on the code PR in the three modes and sends test
   scenarios. Reviewer: reviews the code PR. The orchestrator merges it.
5. Tester: merges `main` into the test PR, adds the deliberate records and
   their stored bytes, and runs all cases. The `rust` job passes. Reviewer:
   reviews the test PR. The orchestrator merges it.
6. The parity test passes in the `rust` job on `main`. Then all three
   stories are done (decision 29).

## Assumptions

- Decision 23 (layout options with `--ask` or `--json`) comes from the code
  convention `conflicts_with_all = MODES` in `src/cli.rs`.
- The snippet path that jag sends to the model changes from
  `(standard input)` to `<stdin>` (decision 5). This is data, not prompt
  text, so the prompt version of ADR-0004 does not change.
- The temporary folder of each case is outside each git repository and has
  no ignore file in its parent folders. A parent ignore file changes the
  output of rg and jag in the same way.

## Risks

| Risk | Signal |
|---|---|
| The ADR round of PR #127 (head `778d4c3`) changes before it merges | A new head of PR #127. The specifier changes this spec to the merged text |
| jag uses `ignore` 0.4.33 and `globset` 0.4.20; rg 15.2.0 uses 0.4.29 and 0.4.19. An error text or a skip rule can differ | A parity case fails on an error text or on the files of a walk |
| A new rg pin changes the output of rg | The pull request that changes `tests/parity/rg.sha256` fails cases |
| A walk with one thread is slower than the scan of today in a large folder | A plain search of a large folder takes more time than rg with `--sort path` |
| The workflow change needs the `workflow` scope of the token | The push of the test PR fails |
| The pseudo terminal behaves differently on macOS | The terminal mode fails only on `macos-latest` |
| After a closed pipe, rg 15.2.0 gives exit 1 when the first file had the match. The harness cannot test this | A user report. jag copies the rg rule |

## Out of scope

- The ranked layout of `--ask`, and the JSON document of `--json`, except
  the color codes (decision 4) and the `<stdin>` label (decision 5).
- `--setup`, `--check`, and `--forget-key` (ADR-0022).
- rg options that card C does not keep. With card C option 1, each one is a
  recorded difference.
- A new rg version. A later PR changes the pin (ADR-0021).
- Windows. jag supports Linux and macOS (system design, Deployment).
