---
title: "README examples for people: the examples section, the sample folder, and the test that runs each example"
feature: FEATURE-27
stories: [STORY-76]
kind: build
gh_issue: 195
lifecycle_state: draft
inherits: [IF-3, IF-4, CV-1, CV-3, CV-6]
session: 4acb9f7a-4603-41fb-8e74-395e5117e288
---

# README examples for people: the examples section, the sample folder, and the test that runs each example

## Problem

The README of jag explains each option, but it has no worked example with
its output (FEATURE-27). CONSTRAINT-09: "The README of jag gives worked
examples for a person at the shell: the operator, the knowledge seeker, and
the shell power user. Each example shows a command that the person can copy
and what jag shows. The examples are for people. They are not instructions
for an agent; the agent side is CONSTRAINT-08 and CAPABILITY-11."

The source is the user, in the hitl card on ADR-0025, 2026-10-20: "We need
some examples in the readme for users on how to use JAG."

- STORY-76: as a person at the shell (PERSONA-02, PERSONA-03, PERSONA-04),
  I want worked examples in the README, so that I can copy a command and see
  what jag shows before I read each option.
  - CRITERION-134: the README has a section of worked examples for a person
    at the shell. Each example gives a command to copy and the output that
    jag shows. The section is for people, not for an agent.
  - CRITERION-135: a test runs each README example on a fixed test folder
    and checks that jag gives the output that the README shows. An example
    that needs the model service uses a fake service in the test. No example
    holds a key or private data.

Today only README lines 265 to 270 show a command with its output, and no
README text shows what a ranked `--ask` result looks like. README line 6
says "This release has the key setup only. The search comes later." The
search is built, so that line is wrong.

## Decisions

Row 2 waits for the rule card that the specifier sent on 2026-10-20 (Ask the
Rule, round 1). The row holds the recommended option, and STORY-76 stays in
`backlog` until the `ANSWER`. Each other row has a source. Rows marked
"inferred" are in the Assumptions list.

| # | Decision | Choice | Source |
|---|---|---|---|
| 1 | Specs | One spec for STORY-76, the only story of the feature | Orchestrator TASK for FEATURE-27 ("1 feature = 1 spec = 1 PR") |
| 2 | The set of examples | Pending the rule card. Recommended: "Everyday searches and the results that need care", 7 examples in this order: E1 to E7 of the Design section. The other option, "Only the everyday searches", keeps E1 to E4 | Rule card of 2026-10-20 (pending); CONSTRAINT-09 (the three kinds of user) |
| 3 | For people, not for an agent | No example uses `--json`. No example tells an agent what to do | CONSTRAINT-09; CRITERION-134 |
| 4 | The place of the section | A new section `## Examples` after `## Check that jag is ready` and before `## Search with no question`. A person installs jag, stores the key, checks it, and then sees the examples before the sections on each option | Inferred: the order of the README sections; STORY-76 ("before I read each option") |
| 5 | Sections that do not change | `## Install` and each part of the README that FEATURE-24 or FEATURE-25 adds or changes (the agent skill command and the agent snippet) do not change | Orchestrator TASK note 2 (reviewer-1, comment 6288716500) |
| 6 | README line 6 | Remove "This release has the key setup only. The search comes later." Remove only. Add no other text there. The FEATURE-24 spec (row 21) removes the same line, so the build that merges second finds nothing to remove | Inferred: README line 6; FEATURE-24 spec row 21; orchestrator TASK note |
| 7 | The note on the test service | The section starts with this paragraph, word for word: "The output below is what jag shows in a terminal, with no color. It comes from a test of jag. The test searches a small sample folder, and a test service takes the place of the model. The scores, the order of the results, and the cost come from that test service. With the real model, the order of your results and your cost can be different. In a pipe, the layout and the number of results can be different." | Orchestrator TASK note 1 (reviewer-1, comment 6288716500); CRITERION-135 (fake service) |
| 8 | The form of one example | One sentence that names the person and the task. Then one HTML comment line `<!-- example: ID -->`. Then one fenced block with no language tag: the first line is `$ ` and the command; the next lines are what jag shows. For an exit code that is not 0, the sentence before the block names it as "exit code N" | Inferred: README fence style (`$ ` prefix, lines 265 to 270); the generated-block markers of `scripts/adr/adr_index.py` |
| 9 | What the block shows | What a person sees in a terminal, with no color: first the notices of the error stream, then the results of standard output, then the cost line | Inferred: `src/cli.rs` (notices before the results, the cost line last); README "In a terminal and in a pipe"; CV-3 |
| 10 | The sample folder | `tests/readme/sample/`, with `logs/`, `notes/`, and `src/`. The text is made up. It holds no key, no address, no real name of a person or a company, and no private word. `.gitattributes` gets `tests/readme/** -text` | Inferred: ADR-0021 (a fixture tree in `tests/`); `.gitattributes` (the `-text` lines of `tests/parity/`); CRITERION-135 |
| 11 | The test | A new file `tests/readme_examples.rs`. It finds each `<!-- example: ID -->` marker in `README.md`, runs the command of the block after it, and compares the text with the block. A table in the test holds, for each ID, the fake scores, the token count, and the exit code | Inferred: ADR-0021 (one cargo test, a fixture copy, byte compare); CRITERION-135 |
| 12 | How the test runs a command | It copies the sample folder into a new temporary folder and runs `sh -c "<command>"` there. `PATH` holds the folder of the jag binary of the build, then `/usr/bin:/bin`. The environment is cleared first. Then the test sets only `PATH`, `HOME` (an empty temporary folder), `TERM=xterm-256color`, and, for an example that needs the model, `JAG_GATEWAY_API_KEY` with the made-up key of `tests/common/mod.rs` and `JAG_BASE_URL` with the address of the fake service | Inferred: `tests/parity/harness.rs` (`env_clear`, `HOME`, `TERM`); `tests/common/mod.rs` (`KEY`, `jag`) |
| 13 | The terminal | Standard output of the command is a pseudo terminal, as in the `terminal` mode of `tests/parity/harness.rs`. The error stream is a pipe, so jag shows no progress line. The test removes each color code (`ESC [` ... `m`) from standard output before the compare | Inferred: ADR-0021 (the terminal mode); README "In a terminal and in a pipe" |
| 14 | The order of the compared text | The lines of the error stream other than a last line that starts with `jag: cost `, then standard output, then that cost line. This is the order of row 9 | Inferred: `src/cli.rs` (`finish` writes the cost line last); row 9 |
| 15 | The fake service | `tests/common/fake_server.rs`. The handler scores each snippet by the text of its lines, from the table of row 11. It reports 750 input tokens for each snippet of a call, so a call of 32 snippets reports 24,000 tokens, as in the README sample `jag: cost $0.002016: scored 33 of 33 results, skipped 0.` | Inferred: `tests/common/fake_server.rs` (`Reply::scores_with_usage`); `tests/cost.rs` (24,000 tokens for each call); README "The cost of a search" |
| 16 | The exit code | The test checks the exit code of each example against the table, and checks that the sentence before a block with a code that is not 0 names "exit code N" | IF-4 |
| 17 | The missing-key example | E7 runs with no key and no `JAG_BASE_URL`, so jag takes the direct route and gives the failure text of a missing TypeSafe key with its fix | CV-1; DECISION-09 (the missing-key message names the page for a key); `src/config.rs` |
| 18 | One-to-one | Each marker in the README has a row in the table, and each row has one marker. An ID appears one time only | Inferred: CRITERION-135 ("each README example") |
| 19 | CI | No new job. The job `rust` of `ci.yml` runs `cargo test --locked` on `ubuntu-latest` and `macos-latest`, so it runs the new test | `.github/workflows/ci.yml` |

## Design

### The examples (row 2, recommended option)

| ID | Person | Command | What it shows | Exit code |
|---|---|---|---|---|
| E1 `operator-pipe` | An operator reads the output of a command | `cat logs/deploy.log \| jag --ask "which step made the deploy fail" fail` | Ranked results from standard input, with the label `<stdin>`, and the cost line | 0 |
| E2 `notes-question` | A knowledge seeker asks a folder of notes | `jag --ask "how many days of leave do new staff get" days notes` | Ranked results from a folder, with the path and the line number of each result | 0 |
| E3 `code-question` | A shell power user ranks the matches in code | `jag --ask "where does the code wait before it tries a request again" retry src` | Ranked results in code, with the lines around each match | 0 |
| E4 `no-question` | A shell power user searches with no question | `jag retry src` | The output of `rg`: the path above its lines, with line numbers, no cost line | 0 |
| E5 `no-good-answer` | A knowledge seeker asks a question that the notes do not answer | `jag --ask "what is the refund policy" policy notes` | The label `jag: no good answer. The results below do not answer the question well.` before the results | 1 |
| E6 `size-limit` | An operator searches many log lines with a small limit | `jag --max-chars 1000 --ask "why did the backup stop" error logs` | The size-limit notice with its fix, and fewer results | 8 |
| E7 `missing-key` | A new user searches before they store a key | `jag --ask "where does the code wait before it tries a request again" retry src` | The failure text of a missing key, with the page for a key and the fix | 3 |

The words of the commands and of the sample files can change in the build.
The person, the kind of input, the result that the example shows, and the
exit code do not change. Each command uses relative paths, so that a person
can copy it into a project of their own.

The sentence before E5, E6, and E7 says what the person must do next: ask a
better question or use a broader term (E5), use a larger `--max-chars` (E6),
and store a key with `jag --setup` (E7, with a link to "Store your TypeSafe
key").

### The README

- Remove line 6 (row 6).
- Add `## Examples` between `## Check that jag is ready` and
  `## Search with no question` (row 4). It holds the paragraph of row 7, then
  the examples in the order of the table, each in the form of row 8.
- Change no other section (row 5).

One example, in the form of row 8 (the output lines are made by the build):

````markdown
An operator reads the output of a command and asks which step failed:

<!-- example: operator-pipe -->
```
$ cat logs/deploy.log | jag --ask "which step made the deploy fail" fail
<stdin>:...
jag: cost $...: scored N of N results, skipped 0.
```
````

### The sample folder

`tests/readme/sample/`:

- `logs/deploy.log`: the steps of a deploy, with many lines that hold `fail`
  and one line that gives the cause (E1).
- `logs/backup.log`: enough lines with `error` that their results are more
  than 1,000 characters (E6).
- `notes/`: a few Markdown notes, one with the days of leave of new staff
  (E2), and none with a refund policy (E5).
- `src/`: a few short source files in which `retry` appears in more than one
  place, and one place waits before it tries again (E3, E4, E7).

The sample folder is in `tests/`, so it is not in the crate (`Cargo.toml`
`include`) and not in the npm package.

### The test (CRITERION-135)

`tests/readme_examples.rs`:

1. Read `README.md` from `env!("CARGO_MANIFEST_DIR")`.
2. Find each line `<!-- example: ID -->`. The next line must open a fence.
   The first line in the fence starts with `$ `. The other lines up to the
   closing fence are the expected text.
3. Check row 18: the IDs of the README and the IDs of the table are the
   same set, and no ID appears two times.
4. For each example: start a `FakeServer` with the handler of row 15 (none
   for E7), copy `tests/readme/sample/` into a new temporary folder, and run
   the command as rows 12 and 13 say.
5. Make the text of row 14 from the error stream and the standard output,
   with the color codes removed. Compare it with the expected text, byte for
   byte. On a difference, print the ID, the expected text, and the text of
   jag.
6. Check the exit code and the "exit code N" sentence (row 16).
7. Check that no output and no README block holds the made-up key.

The test reuses the pseudo terminal code of `tests/parity/harness.rs` with
`#[path = "parity/harness.rs"] mod harness;`, as `tests/rg_parity_fs.rs`
does, or it copies the few functions that it needs. The test needs no `rg`.
It writes no file in `tests/parity/expected/`, because the test
`cases_name_files_that_exist` of `tests/rg_parity.rs` fails on a folder there
that has no case.

A bless switch, `JAG_README_BLESS=1`, prints the text of jag for each
example, so that the build can copy it into the README. The switch does not
write `README.md`.

| Criterion | Test | What it proves |
|---|---|---|
| CRITERION-134 | `tests/readme_examples.rs`, `the_readme_has_an_examples_section_for_people`. `## Examples` exists, holds the paragraph of row 7, holds each example of row 2, and holds no `--json` | The section exists, for people, with a command and an output in each example |
| CRITERION-135 | `tests/readme_examples.rs`, `each_readme_example_gives_the_output_that_it_shows`. Steps 1 to 7 above | Each example gives the output and the exit code that the README shows, with a fake service and no key |

### Error handling

E5, E6, and E7 show the results that need care (row 2). The test fails when
the README text and the output of jag are not the same.

### Security

The examples hold no key. The README shows `JAG_TYPESAFE_API_KEY` only as a
name, as today. The test key is the made-up key of `tests/common/mod.rs`,
which the private-data check passes. The sample text holds no address, no
real name, and no private word (row 10). `HOME` is an empty temporary
folder, so the test never reads `~/.config/jag/`.

### Third-party dependencies

None. The test uses `sh`, `cat`, and the crates that the tests already use
(`assert_cmd`, `tempfile`, `rustix`, `serde_json`).

## Build order

1. Write the sample folder and the `.gitattributes` line (row 10).
2. Write `tests/readme_examples.rs` with the table of the examples and the
   bless switch.
3. Run the test with `JAG_README_BLESS=1`. Read the text of jag for each
   example. If an example does not show what the table says (for example E6
   does not reach the limit), change the sample files.
4. Change `README.md`: remove line 6, and add `## Examples` with the
   paragraph of row 7 and the 7 examples.
5. Run `cargo fmt --check`, `cargo clippy --locked -- -D warnings`, and
   `cargo test --locked`.

## Assumptions

- Row 4 (the place of the section) comes from the order of the README
  sections and from STORY-76.
- Row 6 (remove line 6) comes from README line 6 and the FEATURE-24 spec,
  row 21.
- Row 8 (the marker and the block form) comes from the README fence style
  (lines 265 to 270) and from the markers of the generated blocks of
  `scripts/adr/adr_index.py`.
- Row 9 (what the block shows) comes from `src/cli.rs` and the README
  section "In a terminal and in a pipe".
- Row 10 (the sample folder) comes from ADR-0021 and `.gitattributes`.
- Rows 11 and 12 (the test and how it runs a command) come from ADR-0021,
  `tests/parity/harness.rs`, and `tests/common/mod.rs`.
- Row 13 (the terminal) comes from ADR-0021 and the README section "In a
  terminal and in a pipe".
- Row 14 (the order of the compared text) comes from `src/cli.rs` (`finish`).
- Row 15 (750 tokens for each snippet) comes from `tests/cost.rs` and the
  README section "The cost of a search".
- Row 18 (one-to-one) comes from CRITERION-135.

## Risks

| Risk | Signal |
|---|---|
| A person reads the scores, the order, or the cost of an example as real model output | A user report or an issue that says that jag gives other results than the README |
| A later change of the output text of jag makes an example wrong | `each_readme_example_gives_the_output_that_it_shows` fails in the pull request of that change |
| The FEATURE-24 build and this build both change `README.md` | A merge conflict in `README.md`. The build that merges second keeps the text of the other |
| The pseudo terminal gives other bytes on macOS | The test fails only on `macos-latest` |
| A sample file holds a word that the private-data check blocks | The job `private-data` fails on the build pull request |
| The user picks the other option of the rule card | The `ANSWER`. The specifier changes rows 2 and the table of the examples before STORY-76 goes to `ready` |

## Out of scope

- The agent instructions and the agent skill: FEATURE-24, FEATURE-25.
- `## Install` and the agent snippet (row 5).
- An example of `--json`: the README section "Output for an agent" covers it.
- A change of the output of jag.
- Examples with the real model service.
