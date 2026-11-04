---
title: "The best possible jag-bench score: a bench-only flag that orders the windows of each case in the best order"
epic: EPIC-02
stories: [STORY-79]
kind: build
gh_issue: 298
lifecycle_state: draft
inherits: [COMPONENT-08, CONVENTION-01, CONVENTION-03]
session: 251aa6d8-5726-4f8b-97c5-0160ecc8a60a
---

# The best possible jag-bench score: a bench-only flag that orders the windows of each case in the best order

## Problem

STORY-79: "As the maintainer, I want jag-bench to print the best score that
a perfect order of its own windows can get, so that I know if a fix to jag
can reach the bar."

The development test of STORY-45 failed only the jag check (STORY-45 spec,
row 23): blind grep passed, and jag got less than 0.70. The conversion rule,
the converter and the 2 checks stay frozen (D-001, D-005, D-008). Two known
differences from the old run are parts of the jag-bench score, not parts of
the jag ranking: the line credit (STORY-78 spec, row 11) and the ideal score
over all labeled lines (row 10). So the best score that any order of the
jag-bench windows can get can be less than 0.70. Decision D-011
(decision-judge, 2026-11-04) chose to measure that score first.

The criteria:

- CRITERION-142: given the development case file, the new bench-only flag
  orders the result windows of each case perfectly, makes no model call,
  and prints only the mean best ndcg@10 over the cases with an answer, the
  number of cases with more than 200 results, and the number of labeled
  lines that the pre-rank removes. It prints no question, no case text and
  no per-case result.
- CRITERION-143: without the new flag, the output and the score of
  jag-bench are the same as before, and the parity test of STORY-78 passes.
  The new flag exists only in the `bench` feature.

## Decisions

Row 1 holds the rule of D-011. Rows 2 to 16 follow from it or from the
STORY-78 spec. No row is open, so this spec sends no card.

| # | Decision | Choice | Source |
|---|---|---|---|
| 1 | The rule | Measure the best ndcg@10 that a perfect order of the jag-bench windows can get on the development set, before a change to the prompt or the pre-rank. The windows are those that jag-bench makes itself: the matcher, the scan, the window plan, and the pre-rank only when a case has more than 200 results. The pipeline stops before the model call. Each window gets the credit of STORY-78 spec row 11. The frozen formula of row 10 gives the score. A new flag of jag-bench, only in the `bench` feature. The default output and the default score path do not change. The run uses the development case file that is already converted. The mode prints only 3 totals and no case text (STORY-45 spec, row 5). The run is not a development run of the gate of STORY-45 | decision-judge:D-011, 2026-11-04; `roadmap:/stories/STORY-79.md`; `knowledge:/synthesis/story-45-practice-gate-and-conversion-rule.md` |
| 2 | The flag | `jag-bench --best-order CASES`. A new option of `Args` in `src/bench_main.rs`. The binary has `required-features = ["bench"]`, and `src/bench.rs` has `#[cfg(feature = "bench")]`, so the flag exists only in the `bench` feature. Releases do not hold `jag-bench`, so the flag is not a public option | Derived from row 1. STORY-78 spec, rows 3 and 4; `Cargo.toml` (`[[bin]] jag-bench`, `[features] bench`); `src/lib.rs` line 3 |
| 3 | The windows | The steps of the default path up to the model call, with the same values: `build_matcher` of the term with `ignore_case` and `fixed_strings`, `scan` of the folder with `ScanOptions::default()`, `plan_windows` with `ContextLines::DEFAULT`, and `prerank::best` with `DEFAULT_COST_LIMIT` only when the count of windows is more than `DEFAULT_COST_LIMIT`. One function of `src/bench.rs` makes these windows for both paths. The default path keeps its behavior. The new mode stops after this function | Derived from row 1. STORY-78 spec, rows 6 and 7; 0011-build-jag-as-one-cargo-package-with-one-module-per-component.md (jag-bench is a thin binary that calls the library); `src/bench.rs` lines 433 to 487 |
| 4 | No model, no key, no second read | The mode calls no `read_snippets`, no `score_with_usage` and no `rank`. It does not call `settings()` of `src/bench_main.rs`, so it reads no key and no model setting. The match lines of a window are its `Window.matches`: the line numbers that the scan found | Derived from row 1 (D-011: "The mode makes no model call, so it costs nothing"). STORY-78 spec, row 8 (the key is for the model calls); `src/search/snippet.rs` (`Window.matches`, `snippet_of`) |
| 5 | The credit | The credit of STORY-78 spec row 11, with no change: the mode calls `credit` of `src/bench.rs` | Derived from row 1. STORY-78 spec, row 11; `src/bench.rs` (`credit`) |
| 6 | The best order | Give each kept window a gain: the highest grade of the labeled lines that are its match lines, or 0. Sort the kept windows by gain, highest first. The sort is stable, so windows of equal gain keep the scan order. Score the first 10 with `credit` and `ndcg`. "The best order" (Design) shows that no other order gets a higher score | Derived from row 1. STORY-78 spec, rows 10 and 11; `src/search/snippet.rs` (`match_texts`: `plan_windows` "puts each matching line of `files` in exactly one window") |
| 7 | The score | `ndcg` of STORY-78 spec row 10, with no change: the ideal list holds all labeled lines of the case, by grade, cut at 10. The first total is the mean over the cases with an answer. A case with an answer and no match line scores 0 and stays in the mean. A case with no answer is not scanned and is not in the mean, as in the default path | Derived from row 1. STORY-78 spec, rows 10 and 13; `src/bench_main.rs` lines 141 to 144 |
| 8 | The second total | The count of the cases with an answer whose count of windows is more than `DEFAULT_COST_LIMIT`. This is the count that starts the pre-rank in the default path | Derived from row 1. `src/bench.rs` lines 476 to 478 |
| 9 | The third total | The sum, over the cases with an answer, of the labeled lines that are a match line of a window that `prerank::best` did not keep. A labeled line that is a match line of no window is not in this count. A case with no pre-rank adds 0 | Derived from row 1 |
| 10 | Standard output | Only the 3 lines of "The output" (Design), and only when each case ended. No case count, no bar line, no prompt, no model, no version, no question, no term, no file name, no path and no per-case result | CRITERION-142. Derived from row 1. STORY-45 spec, row 5; STORY-78 spec, row 16 |
| 11 | Standard error | As in the default path: the progress line `jag-bench: case N of M` for each case, and each failure with its cause and its fix. A failure names a case only by its number and the field | STORY-78 spec, row 17; STORY-45 spec, row 5 |
| 12 | `--detail` | Not with `--best-order`. Clap `conflicts_with` refuses the two together with exit 2, and no file is written | CRITERION-142 ("no per-case result"); STORY-45 spec, row 10; 0012-use-clap-serde-saphyr-and-rpassword-as-the-outside-crates.md (clap reads the command line) |
| 13 | Exit codes | 0: each case ended. The mode has no bar, so it never exits with 1. 2: a usage error, a bad case file, or the guard of STORY-78 spec row 19. 130: a Ctrl-C stopped the run, and standard output is empty. 3 to 6 do not occur, because the mode reads no key and calls no model | Derived from row 1 ("not a development run of the gate"). STORY-78 spec, rows 15 and 20; 0007-number-the-exit-codes-above-2-from-3-to-8.md |
| 14 | The guard | The guard of STORY-78 spec row 19 applies to the case file in the mode too | STORY-78 spec, row 19; `src/bench_main.rs` (`guard`) |
| 15 | The default path | Without `--best-order`, the output, the scores, the detail file and the exit codes do not change. The tests `bench_summary`, `bench_matches_jag` and `bench_matches_jag_after_the_pre_rank` pass with no change to the test or its values | CRITERION-143; STORY-78 spec, row 7 |
| 16 | The run after the merge | The researcher and the maintainer run `jag-bench --best-order <work>/<dev file>` one time on the converted development case file, with a `jag-bench` built from a commit of `main` that holds this code (`cargo build --release --features bench --bin jag-bench`). The file is not converted again, so the rule hash and the converter hash of STORY-45 stay the same. The session follows STORY-45 spec rows 5 and 6: it writes `<work>` and `<dev file>` as placeholders, and it sees only the exit code, the progress lines and the 3 totals. The run costs nothing (row 4). It is not the one run of STORY-45 spec row 11, and it is not the development run of rows 21 and 23. After the code merge, the orchestrator sends a `TASK` with this row to the researcher. The `DONE` of the researcher gives only the exit code and the 3 totals to the orchestrator. The orchestrator sends them to the planner, who chooses the next path of the STORY-79 page. STORY-79 is `done` when that `DONE` is in, not at the code merge, because STORY-79 is the only open item of the `blocked_by` list of STORY-45 (STORY-78 is `done`), and `prd.py regen` sets STORY-45 to `ready` when STORY-79 is `done`. The planner sets STORY-79 `done` with `--actor human:404pilo`, and its commit and PR body name the planner session and D-013 | `roadmap:/stories/STORY-79.md` ("The researcher runs the mode with the maintainer and reads the totals"). Derived from row 1. STORY-45 spec, rows 5, 6, 11, 18, 21 and 23; `docs/adlc.md`, section 2, rules 2 and 4 |
| 17 | Tests | Unit tests in `src/bench.rs`. Integration tests in `tests/bench.rs`, with the invented cases of `tests/bench/`. The tests of the mode run with no key and no model service | STORY-78 spec, row 22; row 4 |
| 18 | Documentation | `docs/bench.md` gets the section "The best order": the command, the 3 lines, the exit codes, and the note that the mode is not a gate run | STORY-78 spec, row 23 |
| 19 | Spec group and ADR | One spec for STORY-79. No ADR: `jag-bench` is not shipped, so the flag adds no public interface | STORY-78 spec, row 4 and "For the architect" |
| 20 | The state of STORY-79 | `ready`: this PR merges the spec, STORY-79 has no feature, and no card is open. It has no `blocked_by` list, so `prd.py regen` keeps it `ready`. `prd.py` refuses an agent for `backlog` to `ready`, so the change records `human:404pilo` for the session jag-specifier | `docs/adlc.md`, section 2, rules 1, 2 and 4; D-013 |

## Design

### The command

```
jag-bench --best-order CASES
```

- `CASES`: the path of the case file, as in the default path. The guard of
  row 14 applies.
- `--best-order`: order the windows of each case in the best order, score
  them, and print the 3 totals. With `--detail`, exit 2 (row 12).

`jag-bench --help` names the option: "Print the best score that an order of
the jag-bench windows can get, with no model call. Not a gate run."

### The code

`src/bench.rs`:

1. Move the steps of row 3 from `run_case` into one function, for example
   `plan_case(case: &Case) -> Result<Planned, Failure>`. `Planned` holds the
   blind grep hits, all windows of `plan_windows`, and the indices of the
   kept windows (all indices when the count is not more than
   `DEFAULT_COST_LIMIT`, else the output of `prerank::best`). `run_case`
   calls `plan_case`, then does `read_snippets`, `score_with_usage` and
   `rank` as today. This is a move with no change of behavior. Row 15 is
   the guard.
2. Add the function of the mode, for example
   `best_case(case: &Case) -> Result<BestScore, Failure>`. It calls
   `plan_case`, makes one `Hit` for each kept window (the file relative to
   the folder, and `match_lines = window.matches`), orders the hits as row 6
   says, and gives:
   - `ndcg`: `ndcg(&credit(&hits, &case.answers), &case.answers)`
   - `over_limit`: true when the count of windows is more than
     `DEFAULT_COST_LIMIT` (row 8)
   - `removed`: the count of row 9

`src/bench_main.rs`:

1. Add `--best-order` to `Args`, with `conflicts_with = "detail"`.
2. In `run`, after `guard` and `read_cases`: with `--best-order`, run
   `best_case` for each case with an answer, print the progress line for
   each case, and build the 3 lines of "The output". Do not call
   `settings()`. Exit 0.
3. A Ctrl-C stops the mode with exit 130 and prints no total. If the mode
   keeps the SIGINT flag of the default path, it reads the flag before each
   case.

### The best order

The proof that the order of row 6 gives the highest ndcg@10 of a case:

1. `plan_windows` puts each matching line of the scan in exactly one window.
   So a labeled line is a match line of at most one window, and two windows
   never compete for the same labeled line.
2. By row 11, a window gets credit for at most one labeled line: the
   highest grade of its labeled match lines that no earlier window got.
   Step 1 says that no other window holds those lines. So, at each rank, the
   window gets its gain of row 6.
3. So each order gives DCG@10 = the sum, over the ranks 1 to 10, of the gain
   of the window at rank i times `1 / log2(i + 1)`. The discount gets
   smaller as the rank goes up. A swap of two windows whose gains are in the
   wrong order does not make the DCG smaller. So the DCG is highest when the
   windows with the 10 highest gains are at ranks 1 to 10, highest gain
   first.
4. The ideal list of row 7 does not depend on the order. So the order of
   row 6 gives the highest ndcg@10 over all orders of the kept windows.

The best score of a case can be less than 1 for 3 causes:

- A window holds 2 or more labeled match lines and gets credit for 1 only.
- A labeled line is not a match line of a window: it is a context line, or
  the scan did not find it.
- The pre-rank did not keep the window of a labeled line.

The third total counts only the last cause. The difference between 1 and
the first total, after the third cause, comes from the window and ideal
score rule of rows 10 and 11.

### The output

Standard output, after the last case, and only when each case ended:

```
jag-bench: best ndcg@10 <B>
jag-bench: cases with more than 200 results: <K>
jag-bench: labeled lines that the pre-rank removed: <L>
```

- `<B>`: the first total (row 7), rounded to 3 places.
- `<K>`: the second total (row 8). The number 200 is the value of
  `DEFAULT_COST_LIMIT`.
- `<L>`: the third total (row 9).

Standard error: the progress line of each case, and the failures (row 11).

### How each criterion is proved

Unit tests in `src/bench.rs`:

| Test | It checks |
|---|---|
| `best_order_known_value` | Invented windows with labeled lines of grades 1, 3 and 2 in 3 windows, and 1 window with no labeled line: the best order puts them as 3, 2, 1, and the score is 1.0. With 1 window removed by the pre-rank, the score is the value worked out by hand |
| `best_order_one_gain_per_window` | A window with a grade 3 and a grade 2 labeled match line gives 1 gain of 3. The score is 3 / (3 + 2 / log2(3)) |
| `best_order_context_line` | A labeled line that is a context line of a window gives no gain |
| `best_order_removed_count` | The count of row 9: a labeled line in a removed window counts 1, a labeled line in a kept window and a labeled line in no window count 0 |

Integration tests in `tests/bench.rs`. Each test runs `jag-bench` with
`bench_bare`, with no key and no model service, so a model call or a key
read makes the test fail with exit 3:

| Test | It checks | Criterion |
|---|---|---|
| `bench_best_order_summary` | On a copy of `tests/bench/cases.yaml`: exit 0, and standard output is the 3 lines `jag-bench: best ndcg@10 0.920`, `jag-bench: cases with more than 200 results: 0` and `jag-bench: labeled lines that the pre-rank removed: 0`. The value by hand: the garden and pantry cases score 1.0. In the notes case, line 3 is a context line, so the score is 2 / (2 + 1 / log2(3)) = 0.760. The mean is 0.920. Standard error is the 4 progress lines. Standard output and standard error hold no word of `CASE_WORDS` and no temp path | CRITERION-142 |
| `bench_best_order_after_the_pre_rank` | The 250 files of `bench_matches_jag_after_the_pre_rank`, with the question "Which item needs watering?" and 2 labeled lines: `f249.txt` line 1 (grade 3, a watering line, kept) and `f248.txt` line 1 (grade 2, a plain line, removed: the plain lines tie, and the stable sort of `prerank::best` keeps the first 116 in the scan order). Standard output: `best ndcg@10 0.704` (3 / (3 + 2 / log2(3))), `cases with more than 200 results: 1`, `labeled lines that the pre-rank removed: 1`. The implementer checks the 2 labeled lines against `prerank::best` before the test is final | CRITERION-142 |
| `bench_best_order_rejects_detail` | `--best-order --detail FILE` gives exit 2, and `FILE` does not exist after the run | CRITERION-142 |
| `bench_best_order_guard` | A case file in the source folder gives exit 2 with `--best-order` | Row 14 |
| The tests of STORY-78 | `bench_summary`, `bench_matches_jag`, `bench_matches_jag_after_the_pre_rank`, `bench_detail_local`, `bench_no_key_in_output`, `bench_guard`, `bench_bar_fail` and `bench_failures` pass with no change | CRITERION-143 |

CRITERION-143 also needs that the flag exists only in the `bench` feature:
`cargo build --locked` with no feature builds no `jag-bench` (row 2).

## Build order

1. `src/bench.rs`: move the steps of row 3 into `plan_case`. Run
   `cargo test --locked --all-features` with no other change: each test of
   STORY-78 passes.
2. `src/bench.rs`: `best_case` and its unit tests.
3. `src/bench_main.rs`: the option, the conflict with `--detail`, the
   branch with no `settings()`, and the 3 lines.
4. `tests/bench.rs`: the 4 integration tests.
5. `docs/bench.md`: the section "The best order".
6. After the merge: the run of row 16. The orchestrator sends a `TASK` to
   the researcher. The researcher and the maintainer run the mode, and the
   `DONE` of the researcher gives the 3 totals to the orchestrator. Then
   the planner sets STORY-79 `done` with `--actor human:404pilo` and the
   note of rule 4 (D-013).

One code pull request for steps 1 to 5. The code merge does not finish
STORY-79: step 6 does (row 16).

## Assumptions

- Row 4: `Window.matches` and the `is_match` lines of the snippet that
  `read_snippets` makes from the same window are the same lines, when the
  file does not change after the scan (`snippet_of` in
  `src/search/snippet.rs`). A file that changes or that jag cannot read
  between the scan and the read is not a case of the development set.
- Row 6: `plan_windows` puts each matching line in exactly one window (the
  doc of `match_texts` in `src/search/snippet.rs`, and its `assert_eq!`).
- Row 8: the default path pre-ranks when the count of windows is more than
  `DEFAULT_COST_LIMIT` (`src/bench.rs` lines 476 to 478). jag counts its
  results as windows.
- Row 16: the converted development case file is still on the maintainer's
  machine, outside the clone (STORY-45 spec, row 6).
- Test `bench_best_order_after_the_pre_rank`: the plain lines of the 250
  files have the same BM25 score, because each has 5 words and the same
  question words (`src/prerank.rs`).

## Risks

1. The shared function changes the default path. Signal: a test of
   STORY-78 fails, mostly `bench_matches_jag_after_the_pre_rank`.
   Mitigation: step 1 of the build order lands the move alone, with each
   test green.
2. The mode reads the key or calls the model. Signal: a test of the mode
   exits with 3, or the fake service gets a request. Mitigation: the tests
   use `bench_bare` with no key.
3. The output shows case text. Signal: a test finds a word of `CASE_WORDS`.
   Mitigation: the 3 lines hold only numbers and fixed text.
4. The best score is near 0.70, and the totals do not show the cause
   clearly. Signal: the researcher reads the 3 totals. Mitigation: the next
   steps of the STORY-79 page apply. A score near 0.70 or less sends the
   gate rows 23 and 25 of the STORY-45 spec back to the human as a
   `revisit:`.
5. STORY-45 gets `ready` before anyone reads the totals. Signal: STORY-79
   is `done` at the code merge, and `prd.py regen` sets STORY-45 to
   `ready`. Mitigation: row 16 and step 6 of the build order: STORY-79 is
   `done` only after the `DONE` of the researcher.

## Out of scope

- A change to jag, to the prompt, to the threshold or to the pre-rank: the
  fix story that the planner writes after the run (STORY-79 page).
- A new conversion, or a change to the conversion rule, the converter or
  the 2 checks of the development gate: frozen (D-001, D-005, D-008).
- The held-out run: STORY-45.
- A per-case result of the mode: CRITERION-142 forbids it.
- A count of the labeled lines that the scan does not find, or of the
  windows with 2 labeled lines: CRITERION-142 allows only 3 totals.

## For the architect

- `--best-order` is a flag of `jag-bench`, which is not shipped (STORY-78
  spec, row 4). It adds no public interface.
- The shared function of row 3 is inside `src/bench.rs`. It changes no
  module boundary.
- So this spec writes no ADR.
