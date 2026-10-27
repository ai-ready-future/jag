---
title: "The run tool of the held-out evaluation: score jag and grep on a labeled set from a local path"
epic: EPIC-02
stories: [STORY-78]
kind: build
gh_issue: 258
lifecycle_state: draft
inherits: [QS-1, C-bench, IF-4, CV-1, CV-2, CV-3, CV-6]
session: 4acb9f7a-4603-41fb-8e74-395e5117e288
---

# The run tool of the held-out evaluation: score jag and grep on a labeled set from a local path

## Problem

STORY-78: "As an agent builder (PERSONA-01), I want the Rust jag to score
jag and blind grep on a labeled set that stays on the maintainer's machine,
so that the held-out evaluation can run with no private data in the
repository."

The Rust jag has no run tool for the held-out evaluation. STORY-45 needs
one: before the first release tag, the maintainer runs the Rust jag one
time on the held-out split, on their own machine (DECISION-70, "Run Rust
jag once, privately").

The criteria:

- CRITERION-140: given a labeled set at a local path, the run scores jag
  and blind grep and prints the ndcg@10 of each and the number of cases.
  The output shows no question, no case text, and no per-case result.
- CRITERION-141: the tests of the run use only invented cases. The data
  path and the key come from the maintainer's machine, never from the
  repository, and the per-case results stay on that machine.

The target (METRIC-01, QS-1): ndcg@10 of 0.70 or more on the held-out set,
higher than blind grep.

## Decisions

Row 2 is open. A rule card about it went to the orchestrator on
2026-10-27. Rows 9 to 13 hold the recommended option of that card. They
have the source "pending row 2", and they change if the user picks the
other option. STORY-78 stays `backlog` until the answer (rule 11).

| # | Decision | Choice | Source |
|---|---|---|---|
| 1 | The rule | The maintainer runs the held-out split one time, on their own machine, with a guiding session. No CI job runs it. The prompt, the threshold and the 0.70 bar are frozen. The held-out split chooses nothing. Only a summary of the numbers goes public. The data path, the key and the per-case results stay local. The run tool and its invented test cases are public | DECISION-70; CONSTRAINT-01; CRITERION-140; CRITERION-141 |
| 2 | The case format and the score rule | **Open.** Recommended: a new documented case file and a standard ndcg@10 formula (rows 9 to 13). The guiding session of STORY-45 converts the held-out set to this file on the maintainer's machine. The other option: copy the case format and the formula of the earlier bench harness | Rule card "The case format and the score rule of the held-out run", sent 2026-10-27. DECISION-70 names no case format and no formula. This session cannot read the earlier harness |
| 3 | The form of the tool | A second binary, `jag-bench`, in the package `jag-cli`. The code is the module `bench` (`src/bench.rs`). The binary is `src/bench_main.rs` | 0005-build-jag-as-one-rust-binary.md ("two commands, `jag` and `jag-bench`"); 0011-build-jag-as-one-cargo-package-with-one-module-per-component.md ("A second binary, `jag-bench`, comes later in the same package"); system design C-bench |
| 4 | Not shipped | `Cargo.toml` gets the feature `bench`, with no dependency, and `[[bin]] name = "jag-bench"` with `required-features = ["bench"]`. So `cargo install jag-cli`, the release archives and the npm packages do not hold `jag-bench`. The maintainer builds it with `cargo build --release --features bench --bin jag-bench` | Inferred: FEATURE-18 spec, decision 27 ("Not in the archives and not in the npm packages. The feature that builds it decides"); `.github/workflows/release.yml` line 209 builds with no feature. A shipped command would be a public option |
| 5 | CI | `ci.yml` runs clippy and the tests with `--all-features` (lines 45 and 53), so CI builds and tests `jag-bench`. `release.yml` does not change | Inferred: `.github/workflows/ci.yml` lines 45 and 53; `scripts/release/release.py` `check_crate_list` allows each path under `src/` |
| 6 | The run path | In-process. `jag-bench` calls the library functions of a search in the order of `cli::search` (`src/cli.rs` lines 688 to 805): the matcher of the term, `search::walk::scan` of the case folder as `jag` scans its current folder when it gets no path, `plan_windows` with `ContextLines::DEFAULT`, `prerank::best` only when the count is more than `DEFAULT_COST_LIMIT`, `read_snippets`, `score_with_usage`, then `rank`. The matcher builder (`build_matcher`, now `pub(super)` in `src/cli/grep.rs`) becomes reachable from the library, with no change of behavior | 0014-call-the-model-service-with-ureq-and-serde-json-on-std.md ("C-scorer, C-check and C-bench use one shared `ureq` agent"); system design C-bench (depends on C-search, C-prerank and C-scorer); `src/cli.rs` |
| 7 | The guard against drift | A parity test: on the invented cases, with the fake model service, the top 10 results of `jag-bench` (file and line, in order) equal the `results` of `jag --json --ask`, run in the case folder with no path, for the same term and question. The output of each invented case stays under the default size limit, so `jag --json` cuts no result | Inferred: `tests/common/fake_server.rs`; row 6. The held-out run must measure the ranking that ships |
| 8 | The key and the settings | As in jag: `find_key` and `model_settings` of C-config, from the variables and `~/.config/jag/config.yml`. A gateway route works. `jag-bench` never prints the key | CV-6; 0015-read-the-base-url-and-the-model-name-from-config-base-url.md; 0017-use-a-separate-key-for-an-ai-gateway.md |
| 9 | The case file | One YAML file, read with serde-saphyr. Format in "The case file". The path is the one argument of `jag-bench` | Pending row 2. 0012-use-clap-serde-saphyr-and-rpassword-as-the-outside-crates.md; `tests/parity/cases.yaml` (cases in a YAML file) |
| 10 | The score | ndcg@10 for each case, with gain = grade and the discount log2(rank + 1). The ideal list holds all labeled lines of the case, by grade, cut at 10. The jag list is the ranked list as `jag --json --ask` writes it, also when its top score is below `THRESHOLD`. The score of a tool is the mean over the cases that have answers | Pending row 2. Inferred: the standard ndcg@k (trec_eval form, gain = grade) |
| 11 | The credit of a result | A result gets the highest grade of the labeled lines that are match lines of the result (`is_match`), and that no earlier result got. Context lines get no credit. Each labeled line gives credit one time | Pending row 2. Inferred: `src/search/snippet.rs` (`SnippetLine.is_match`). Without this rule, the context lines of jag get credit that grep cannot get |
| 12 | Blind grep | The matching lines of the same scan, ordered by the bytes of the path, then by the line number. Each line is one result. The top 10 count. This is the order of `rg --sort path` | Pending row 2. Inferred: METRIC-01 ("blind grep"); rg 15.1.0 (the default order of rg across files is not fixed, so the order must be sorted) |
| 13 | A case with no answer | Not in the mean. The summary gives the count of such cases | Pending row 2. METRIC-07 (decoy questions) is out of scope |
| 14 | Frozen values | `jag-bench` has no option for the prompt, the threshold, the model, the cost limit, the context lines or the bar. It uses `PROMPT_VERSION` and `THRESHOLD` (`src/scorer.rs`), `DEFAULT_COST_LIMIT`, `ContextLines::DEFAULT`, and the constant `BAR = 0.70`. It reads no limit variable. `THRESHOLD` changes no score (row 10). The summary prints it, because it is part of the frozen release. The model name comes from the settings (row 8), and the summary prints it | DECISION-70; 0003-decide-no-good-answer-with-a-fixed-threshold-on-the-top.md; 0004-ship-one-scoring-prompt-with-a-version-id-in-each-release.md |
| 15 | The bar | `pass` when the jag score is 0.70 or more and higher than the grep score. Else `fail`. The test uses the values before rounding | DECISION-70; CRITERION-73 |
| 16 | Standard output | Only the 4 summary lines of "The output". No question, no term, no case text, no file name, no path and no per-case result | CRITERION-140; CV-3 |
| 17 | Standard error | Progress `jag-bench: case N of M`, with no text of the case. A failure names the case by its number in the file and the field, never by its text or a path. Each failure gives its cause and its fix (CV-1) | CRITERION-140; CV-1 |
| 18 | Per-case results | Only with `--detail FILE`. One JSON line for each case: the case number, the two ndcg@10 values, and the rank of each labeled line in each list (or `null`). No text and no path | CRITERION-141; DECISION-70 ("the per-case results stay on the maintainer's machine") |
| 19 | The guard against data in the repository | `jag-bench` stops with exit 2 when the case file or the `--detail` file is in the folder of the source that built it (`env!("CARGO_MANIFEST_DIR")`) | Inferred: CRITERION-141 ("never from the repository"). The case folders can be git repositories, so a general test for `.git` is not used |
| 20 | Exit codes | 0: the run ends and the bar passes. 1: the run ends and the bar fails. 2: a usage error, a bad case file, or the guard of row 19. 3 to 6 and 130: as in jag. A case with more matches than `DEFAULT_COST_LIMIT` pre-ranks as in jag (row 6), and it does not change the exit code of `jag-bench`. The codes 7 and 8 do not apply, because `jag-bench` writes no result text | IF-4; 0007-number-the-exit-codes-above-2-from-3-to-8.md |
| 21 | A failure during the run | The run stops with the failure and prints no score (CV-2). A new run after such a stop keeps "one time" true, because nobody saw a score | CV-2; DECISION-70 |
| 22 | Tests | Unit tests of the score in `src/bench.rs`. Integration tests in `tests/bench.rs`, with `#![cfg(feature = "bench")]`, the invented cases of `tests/bench/`, and the fake model service. Each test copies the cases to a temp folder | 0021-check-rg-parity-with-a-cargo-test-that-runs-a-pinned-rg-and.md (the e2e tests are cargo integration tests); `tests/readme_examples.rs` (it copies the sample to a temp folder); CRITERION-141 |
| 23 | Documentation | `docs/bench.md`: how to build and run `jag-bench`, the case file with an invented example, the score rule, and the output. `jag-bench --help` names the options. The README does not change, because users do not get `jag-bench` (row 4) | Inferred: row 4; STORY-45 needs a written run path for the guiding session |
| 24 | Spec group | One spec for STORY-78. STORY-45 gets its own spec. It is `blocked_by` STORY-78 | Orchestrator TASK of 2026-10-27; STORY-45 page |
| 25 | The state of STORY-78 | `backlog`, because the card of row 2 is open | docs/adlc.md, rule 11 |

## Design

### The command

```
jag-bench [--detail FILE] CASES
```

- `CASES`: the path of the case file. It comes from the maintainer's
  machine. The repository holds no held-out case file.
- `--detail FILE`: write the per-case results to `FILE` (row 18). With no
  `--detail`, `jag-bench` writes no per-case result.
- `--help` and `--version`, from clap.

`jag-bench` reads the key and the model settings as jag does (row 8).

### The case file

Pending row 2. An invented example:

```yaml
cases:
  - folder: shop            # relative to the case file, or absolute
    term: retry
    question: Where does the client wait before it tries again?
    answers:
      - file: src/client.rs # relative to the folder
        line: 42
        grade: 2
      - file: src/client.rs
        line: 57            # grade 1 when not given
  - folder: shop
    term: refund
    ignore_case: true       # as -i; false when not given
    fixed_strings: false    # as -F; false when not given
    question: Who approves a refund above the limit?
    answers: []             # a case with no answer (row 13)
```

The rules. A broken rule stops `jag-bench` with exit 2, before the first
model call, and names the case number and the field:

- `cases` holds 1 case or more. An unknown field is an error.
- `folder` is a folder that exists.
- `term` is not empty, and it is a valid term for jag.
- `question` is not empty.
- Each answer has `file` and `line`. The file exists in the folder.
  `line` is 1 or more. `grade` is 1, 2 or 3.
- Two answers of one case do not name the same file and line.

### One case

For each case, in the order of the file:

1. Build the matcher of `term`, with `ignore_case` and `fixed_strings`.
2. Scan `folder` (row 6).
3. Blind grep (row 12): sort the matching lines by the bytes of the path,
   then by the line number. Keep the first 10.
4. jag (row 6): the windows, the pre-rank when the count is more than
   `DEFAULT_COST_LIMIT`, the snippets, the model scores, the rank. Keep the
   first 10.
5. Score each list with rows 10 and 11. A labeled line matches a result
   when the file of the result, relative to `folder`, and the line are the
   same.

A case with no match gives both lists a score of 0, and it stays in the
mean. A case with no answer is not in the mean (row 13).

### The score

For one list of up to 10 results with the gains g1 to g10:

```
DCG@10  = sum over i of g_i / log2(i + 1)
IDCG@10 = the DCG@10 of the grades of all labeled lines, highest first, cut at 10
ndcg@10 = DCG@10 / IDCG@10
```

The score of a tool is the mean of its ndcg@10 over the cases with
answers. The summary and the `--detail` file round to 3 places. The bar
(row 15) uses the values before rounding.

### The output

Standard output, after the last case, and only when each case ended:

```
jag-bench: <N> cases, <D> with no answer
jag-bench: ndcg@10 jag <J>, grep <G>
jag-bench: bar pass (jag 0.70 or more, and higher than grep)
jag-bench: prompt <PROMPT_VERSION>, threshold <THRESHOLD>, model <model>, jag <version>
```

When the bar fails, the third line is
`jag-bench: bar fail (jag 0.70 or more, and higher than grep)`.

These 4 lines are the public-safe summary. STORY-45 copies the numbers to
the release notes and to one README line.

Standard error: one progress line for each case (row 17), and the
failures. The progress line holds no text of the case.

`--detail FILE` writes one JSON line for each case, for example:

```json
{"case":1,"jag":0.832,"grep":0.117,"answers":[{"jag_rank":1,"grep_rank":null},{"jag_rank":4,"grep_rank":9}]}
```

### The test plan

Unit tests in `src/bench.rs`:

| Test | It checks |
|---|---|
| `ndcg_perfect_order` | The ideal order gives 1.0 |
| `ndcg_known_value` | An invented list with grades 2, 0, 1 gives the value worked out by hand |
| `ndcg_credit_once` | Two results that hold the same labeled line: only the first gets credit |
| `ndcg_context_lines` | A labeled line that is a context line of a result gives no credit |
| `ndcg_ideal_cut` | 12 labeled lines: the ideal list holds the 10 highest grades |
| `case_file_errors` | Each rule of "The case file" gives exit 2 with the case number and the field |

Integration tests in `tests/bench.rs` (`#![cfg(feature = "bench")]`). The
invented cases are in `tests/bench/cases.yaml` and the invented folders in
`tests/bench/`. Each test copies them to a temp folder. The fake model
service of `tests/common/fake_server.rs` gives fixed scores. The key is a
made-up key in a variable.

| Test | It checks | Criterion |
|---|---|---|
| `bench_summary` | 3 cases with answers and 1 with no answer. Standard output is the 4 lines with the values worked out by hand. Standard output and standard error hold no question, no term, no file name of a case, no folder path and no line text | CRITERION-140 |
| `bench_matches_jag` | For each invented case, the jag ranks of `--detail` equal the order of the `results` of `jag --json --ask`, run in the case folder with no path, for the same term and question (row 7) | CRITERION-140 |
| `bench_detail_local` | Without `--detail`, no file is written. With `--detail`, each line has only the fields of the example, and no text and no path | CRITERION-141 |
| `bench_no_key_in_output` | The made-up key is in no output | CRITERION-141 |
| `bench_guard` | A case file or a `--detail` file in the source folder gives exit 2 | CRITERION-141 |
| `bench_bar_fail` | Fake scores that rank the answers low give exit 1 and `bar fail` | Row 15 |
| `bench_failures` | No key gives exit 3. The fake service down gives exit 6. In both, standard output is empty | Rows 20 and 21 |

CRITERION-141 also has a check outside the tests: the `private-data`
check (FEATURE-17) scans each file of the pull request with the private
patterns, and the reviewer checks that `tests/bench/` holds only invented
text.

## Build order

1. `Cargo.toml`: the feature `bench` and the binary `jag-bench` (row 4).
2. Make the matcher builder reachable from the library, with no change of
   behavior (row 6). `cargo test --locked` passes with no change.
3. `src/bench.rs`: the case file, the run of one case, the score, the
   summary, and the unit tests.
4. `src/bench_main.rs`: the options, the guard of row 19, the exit codes.
5. `tests/bench.rs` and the invented cases of `tests/bench/`.
6. `.github/workflows/ci.yml`: `--all-features` for clippy and the tests
   (row 5).
7. `docs/bench.md` (row 23).

One code pull request. A code merge finishes STORY-78. The held-out run
itself is STORY-45.

## Assumptions

- Row 4: `required-features` keeps `jag-bench` out of `cargo install` and
  out of `cargo build` with no feature (the Cargo reference). The release
  archives take only `jag` (FEATURE-18 spec, decision 27).
- Row 5: `scripts/release/release.py` `check_crate_list` allows each path
  under `src/`. `tests/` and `docs/` are not in the crate (`include` in
  `Cargo.toml`).
- Row 6: the order of the library calls in `cli::search` is the ranking
  that ships. The test of row 7 shows a change of that order.
- Rows 10 to 12: the formula, the credit rule and the grep order are the
  recommended option of the open card (row 2).
- Row 12: the sort of the matching lines by path bytes gives the order of
  `rg --sort path` (rg 15.1.0).
- Row 19: `env!("CARGO_MANIFEST_DIR")` is the folder of the clone that
  built `jag-bench`. A copy of the binary keeps that path.

## Risks

1. The user picks the other option of row 2. Signal: the `ANSWER` of the
   card. Mitigation: rows 9 to 13 change, and the rest of the spec stays.
2. The ndcg@10 of the new formula differs from the number of the earlier
   harness on the same split. Signal: a run of `jag-bench` on the
   development split, on the maintainer's machine, gives a score far from
   the about 0.72 of METRIC-01. Mitigation: the card of row 2 names this
   cost. STORY-45 can run the development split first, because that split
   is for tuning and DECISION-70 does not limit it.
3. `jag-bench` and `jag` rank in different ways. Signal: `bench_matches_jag`
   fails. Mitigation: fix `jag-bench`, not the test.
4. A held-out case file goes into a commit. Signal: the `private-data`
   check fails, or a reviewer sees case text. Mitigation: row 19; the
   case file never has a place in the repository.
5. The held-out split has many cases, and the run costs more than
   expected. Signal: the cost of the model service after the run.
   Mitigation: the development set costs about $0.014 for each question
   (METRIC-05). STORY-45 states the expected cost before the run.
6. A model failure stops the run near the end. Signal: exit 3 to 6 with no
   summary. Mitigation: row 21; run again.

## Out of scope

- The held-out run, the conversion of the held-out set, the public summary
  and the no-tag rule: STORY-45.
- A decoy measure (METRIC-07, QS-2).
- `jag-bench` for users (CAPABILITY-12): a later feature decides whether
  it ships.
- A CI job that runs the held-out split: DECISION-70 forbids it.

## For the architect

- `jag-bench` is the second binary of 0005 and 0011. It is not shipped
  (row 4), so it adds no public interface.
- The case file and the output are for the maintainer only.
- The no-tag rule is a step of the maintainer that DECISION-70 records.
  No release code checks it, and
  0010-ship-jag-on-github-releases-crates-io-and-npm.md does not change.
- So this spec writes no ADR. If a later feature ships `jag-bench` to
  users, that feature needs an ADR for the case file and the output.
