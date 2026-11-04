---
title: "The old prompt as a new prompt version: jag-p2, its threshold, and one more development run"
epic: EPIC-02
stories: [STORY-81]
kind: build
gh_issue: 316
lifecycle_state: draft
inherits: [COMPONENT-05, COMPONENT-08, CONVENTION-01, CONVENTION-03]
session: 251aa6d8-5726-4f8b-97c5-0160ecc8a60a
---

# The old prompt as a new prompt version: jag-p2, its threshold, and one more development run

## Problem

STORY-81: "As the maintainer, I want jag to score with the prompt of the old
tool as a new prompt version, tuned and tested on the development set, so
that jag can reach the release bar before the one held-out run."

The first practice test of STORY-45 used the prompt `jag-p1`. It passed
check 1 and failed check 2 (D-011). STORY-79 then showed that the best
possible score of the `jag-bench` windows is clearly above the bar of D-008,
also after the pre-rank. So the gap is in the order that jag gives to the
windows. The old tool used the prompt `noul-b-v1`. D-015 chooses this story,
and gives a stop rule: the new prompt gets 1 development run.

CRITERION-146: jag scores with the prompt of the old tool under a new prompt
version id, with a threshold tuned on the development set, and the decoy AUC
stays at the bar of METRIC-07.

CRITERION-147: the development test of STORY-45 runs one more time with the
new prompt, the same conversion rule, the same converter and the frozen 2
checks.

### Three measures that are not the same

The story names three measures. They are independent, and this spec keeps
them apart:

1. **Check 2 (ndcg@10).** `jag-bench` ranks the windows by the model score.
   The threshold does not change the rank, so it does not change ndcg@10
   (`docs/bench.md`, "How the tool scores a case"; `run_case` in
   `src/bench.rs`). Check 2 tests only the prompt (and the pinned model).
2. **The decoy AUC (METRIC-07).** The AUC compares the top scores of questions
   with an answer and of decoy questions. It does not use a cut, so the
   threshold does not change it either. It is a property of the prompt and
   the model.
3. **The threshold (ADR-0003).** The threshold is a cut on the top score. It
   sets the label "no good answer" and exit code 1 (DECISION-11). It is
   chosen with the prompt on the development set.

So the rule of D-008 ("nothing is tuned on the result" of the gate) and the
rule of ADR-0003 ("tune the threshold on the development set") do not
conflict: the gate cannot see the threshold.

## Decisions

| # | Decision | Choice | Source |
|---|---|---|---|
| 1 | The rule | jag gets the prompt of the old tool as a new prompt version. The development test of STORY-45 runs one more time with it, with the frozen 2 checks, the same conversion rule and the same converter. The prompt and the threshold are frozen before the held-out run. The stop rule of row 9 applies | STORY-81 page; D-011; D-015; CRITERION-146; CRITERION-147 |
| 2 | The prompt version id | `jag-p2`. `PROMPT_VERSION` in `src/scorer.rs` changes from `jag-p1` to `jag-p2`. The binary holds only `jag-p2`: no option, no setting and no second prompt can choose `jag-p1`. The id is new, and not `noul-b-v1`, because the request of jag (row 4) is not the request of the old tool, and the threshold is part of the version (ADR-0003) | 0004-ship-one-scoring-prompt-with-a-version-id-in-each-release.md; `src/scorer.rs` line 22 (the series `jag-p<n>`). Inferred |
| 3 | The prompt text | **Open: waits for the text of `noul-b-v1`.** The instruction text of `noul-b-v1`, word for word. The user gets the text from the old prototype with `git show` or `git archive`, and chores guides this step. The user reads the text and confirms that it holds no text of a case of the development set or the held-out set before it goes into a file of this repository (row 16). The spec then gives the text and the first 12 characters of its SHA-256 | STORY-81 page ("A step before the work"); `docs/adlc.md`, section 1, row "Extra context files", and the repository rules |
| 4 | The request shape | **Open: waits for the text.** The name says `noul` questions, shape B, version 1. If the text confirms shape B (the snippets in `state`, 1 `noul` question for each snippet), `request_body` keeps its structure, and only `instruction` (and a field name of `state`, if the old one is different) changes. If the shape is different, this row lists the changes to `request_body` and to the estimate of `split_batches`. A change of the 32 results in each call is a change of ADR-0002, so it goes to the architect as a `revisit:` | `src/scorer/prompt.rs` (the module comment: "Shape B"); 0002-score-the-top-200-results-in-calls-of-32-six-calls-in.md; `docs/adlc.md`, section 4, "Ask the Rule", item 7 |
| 5 | What does not change | The windows and the context lines, the cut of a line at 300 characters, the pre-rank to `DEFAULT_COST_LIMIT` (200), calls of 32 results and 6 calls in parallel, the estimate limit of a call (decision 22), the retry, the cost count, and the field name `prompt_version` of the structured output | 0002-score-the-top-200-results-in-calls-of-32-six-calls-in.md; DECISION-12; D-015 (no pre-rank change); `src/scorer/prompt.rs` |
| 6 | The model | `DEFAULT_MODEL` stays `jev-1.13.0`. The development run uses no model setting. The settings check of STORY-45 spec row 8 shows this in the 4th line | 0009-pin-the-jev-model-version-in-each-release.md; STORY-45 spec, row 8; `src/config/model.rs` |
| 7 | The threshold | **Open: card sent.** How the threshold of `jag-p2` is set. Recommended: a rule written before the run picks the cut from the same run (row 8, option A). Other options: keep 0.4, the value of the old run with `noul-b-v1` (STORY-45 spec, row 25); or move the work to a new story | 0003-decide-no-good-answer-with-a-fixed-threshold-on-the-top.md ("tuned with the prompt on the development set"); CRITERION-146. No page gives the method |
| 8 | The decoy measure | **Open: card sent.** `jag-bench` has no decoy measure, and a case with no answer makes no model call. Recommended (option A): a bench-only option of `jag-bench` that also scores the cases with no answer in the same run as the gate, and prints the decoy AUC and the cut of row 7. The 4 lines of the gate do not change | 0003-decide-no-good-answer-with-a-fixed-threshold-on-the-top.md ("a new prompt needs a new threshold and a new decoy run"); METRIC-07; `docs/bench.md`; STORY-78 spec and STORY-45 spec, "Out of scope" |
| 9 | The stop rule | `jag-p2` gets 1 development run under the frozen check 2. There is no third prompt, and the prompt does not change between runs. If check 2 fails, STORY-45 spec rows 23 and 25 go back to the human as a `revisit:` card. A change of the pre-rank limit (more than 200 results) is not an automatic second fix: it conflicts with METRIC-05 and needs a change of ADR-0002 and DECISION-12, so it can only be an option on that card | D-015; STORY-81 page |
| 10 | The development run commit | A clean checkout of a commit of `main` that holds the code PR of this spec. `jag-bench` is built from it with `cargo build --release --features bench --bin jag-bench`. The prompt code and `PROMPT_VERSION` do not change between this commit and the run commit of the held-out run (STORY-45 spec, row 7). The 4th line of the held-out run names the same prompt version and the threshold of row 7 | STORY-81 page ("Freeze the new prompt and the new threshold before the held-out run"); D-011; STORY-45 spec, rows 2 and 7 |
| 11 | The second development run | Chores guides the user through STORY-45 design steps 1 to 4, with the changes of "The second development run" (Design): the development set is not converted again, and the rule hash and the converter hash must be the frozen values. The 2 checks of STORY-45 spec row 23 apply as written | CRITERION-147; D-011 ("The rule hash and the converter hash stay the same"); STORY-45 spec, rows 21 and 23 |
| 12 | The cost | About 74 × $0.014 ≈ $1.05 for the cases with an answer. If row 8 scores the cases with no answer, about 9 × $0.014 ≈ $0.13 more. The user says yes before the run | STORY-45 spec, row 9; METRIC-05 |
| 13 | What the session sees and sends | STORY-45 spec rows 5 and 6 apply to each step: chores writes `<work>` and `<dev file>` as placeholders, and sees only counts, exit codes, the summary lines, and the progress and failure lines. The `DONE` of chores gives only words: `check 1 pass` or `check 1 fail`, `check 2 pass` or `check 2 fail`, and the result of row 8 in words. No number of the run and no path goes into a message, a commit, a PR, an issue or a log | STORY-45 spec, rows 5, 6, 17 and 26; orchestrator TASK of 2026-11-04 |
| 14 | The outcome | Both checks pass and row 8 passes: the prompt and the threshold are frozen, and STORY-45 goes on at design step 5 with a new run commit (STORY-45 spec, row 7). Check 2 fails: row 9. Check 1 fails: STORY-45 spec row 26 (the labels are wrong, and the planner chooses the next path) | D-008; D-015; STORY-45 spec, rows 23 and 26 |
| 15 | The change to the STORY-45 spec | This PR changes STORY-45 spec rows 23, 24, 25 and 26, and design step 4, as "The change to the STORY-45 spec" (Design) shows. Row 23 said that the practice test runs once, with `jag-p1`. It now also allows the second practice test of this spec | STORY-81 page (last item of the stop rule); D-015; orchestrator TASK of 2026-11-04 |
| 16 | The prompt text is public | The prompt text goes into `src/scorer/prompt.rs`, and that file is public. The user checks the text before chores puts it into the spec (row 3). The `private-data` check scans the spec PR and the code PR | CONSTRAINT-01; STORY-45 spec, rows 5 and 6; 0027-approve-each-fork-run-of-the-private-data-check-through-a.md |
| 17 | Tests | Unit tests in `src/scorer/prompt.rs` for the text and the shape of rows 3 and 4. The tests that name `jag-p1` change to `jag-p2` (list in "The files that change"). The bench tests of row 8 run with no key, as the tests of `tests/bench.rs` do. No test calls the real model service | `src/scorer/prompt.rs` (tests); `tests/bench.rs`; STORY-78 spec, row 22 |
| 18 | Documentation | `README.md` line 635 (the example of `prompt_version`), `docs/bench.md` line 129 (the 4th line) and the section of row 8, and `src/scorer/CLAUDE.md` line 6 | STORY-78 spec, row 23; `src/scorer/CLAUDE.md` |
| 19 | ADR | No new ADR. A new prompt version is the case that ADR-0004 describes, and no release holds `jag-p1` yet. A bench-only option adds no public interface (STORY-79 spec, row 19). If row 4 changes the call size, or the card of rows 7 and 8 chooses to leave out the decoy run, a `revisit:` goes to the architect | 0004-ship-one-scoring-prompt-with-a-version-id-in-each-release.md; 0003-decide-no-good-answer-with-a-fixed-threshold-on-the-top.md; STORY-79 spec, row 19; `docs/adr/CLAUDE.md` |
| 20 | Who does the work | Before the code: chores guides the text step of row 3. The code: 1 implementer PR. After the merge: chores guides the run of row 11. The code PR does not change `.github/`, so no security review is necessary | `docs/adlc.md`, roles table; STORY-45 spec, rows 4 and 18 |
| 21 | Done | STORY-81 is `done` when the `DONE` of chores gives `check 1 pass`, `check 2 pass`, and a pass of row 8. It is not `done` at the code merge, because STORY-81 blocks STORY-45, and `prd.py regen` sets STORY-45 to `ready` when STORY-81 is `done`. On a fail, STORY-81 stays open, and the card of row 9 or row 14 decides. The planner sets `done` with `--actor human:404pilo`, and the commit and the PR body name the planner session and D-013 | STORY-79 spec, row 16; `docs/adlc.md`, section 2, rules 2 and 4; D-014 |
| 22 | The state of STORY-81 | `backlog` while rows 3, 4, 7 and 8 are open. When they are closed, this PR sets `ready` with `--actor human:404pilo`, because `prd.py` refuses an agent for `backlog` to `ready` (D-013) | `docs/adlc.md`, section 2, rules 1 and 4; D-013; D-014 |
| 23 | Spec group | One spec for STORY-81 | Orchestrator TASK of 2026-11-04 |

## Design

### The prompt

**Open (rows 3 and 4).** This section gets the text of `noul-b-v1` and its
SHA-256 when the text step is done. The work that does not need the text:

1. `src/scorer.rs`: `PROMPT_VERSION` becomes `"jag-p2"`. `THRESHOLD` gets the
   value of row 7.
2. `src/scorer/prompt.rs`: the module comment names `jag-p2` and says that
   the text comes from `noul-b-v1`. `instruction(n)` returns the text of row
   3. If row 4 confirms shape B, `request_body` and `split_batches` keep their
   code. `split_batches` computes the longest question from `instruction`, so
   the estimate follows the new text with no other change.
3. The unit test
   `the_body_has_the_model_the_state_and_one_question_for_each_snippet`
   compares the instruction of `s1` and `s2` with the new text.
4. The code keeps no copy of the `jag-p1` text, because a release holds one
   prompt (ADR-0004). The `jag-p1` text stays in the git history.

### The threshold and the decoy measure

**Open (rows 7 and 8, card sent).** The recommended option of the card:

1. A bench-only option of `jag-bench` (built only with the feature `bench`)
   runs jag also on each case with no answer of the case file, in the same
   run as the gate, with the same scan, pre-rank, prompt and model.
2. For each case, the tool keeps the top model score. A case with no window
   gets the top score 0.
3. The decoy AUC: the share of pairs (a case with an answer, a case with no
   answer) where the case with an answer has the higher top score. A tie
   counts as one half.
4. The cut: from 0.05 to 0.95 in steps of 0.05, the cut with the best
   balanced accuracy. A case with an answer is correct when its top score is
   the cut or more. A case with no answer is correct when its top score is
   below the cut. On a tie, the cut nearest to 0.4, then the lower cut. This
   rule is written here before the run.
5. The tool prints the 4 lines of the gate with no change, then 2 lines: the
   decoy AUC and the cut. The bar of the decoy line is 0.85 (METRIC-07).
6. After the run, a small PR sets `THRESHOLD` to the cut, if it is not 0.4.
   The prompt does not change, so check 2 does not change (see "Three
   measures that are not the same").

The cases with no answer are the decoy questions of the old set (see
"Assumptions").

### The second development run

Chores guides the user through STORY-45 design steps 1 to 4, with these
changes:

1. Step 1: the run commit is the commit of row 10. Its `Cargo.toml` does not
   need the release version, because this is not the held-out run.
2. Step 3 (the settings check): the 4th line must name `jag-p2`, the
   `THRESHOLD` of the commit, and `DEFAULT_MODEL`.
3. Step 4.1: chores reports the differences of the changed STORY-45 spec row
   25 to the orchestrator.
4. Steps 4.2 and 4.3: chores writes no new converter. If the converted
   development case file of the first practice test is still on the
   machine, the user does not convert again. If the file is gone, the user
   converts the development set again with the frozen converter, and the
   counts must be 74 cases with an answer and 9 cases with no answer.
5. Step 4.4: chores computes the rule hash and the SHA-256 of the converter,
   prints only the first 12 characters of each, and compares them with the
   values that it recorded before the first practice test. A different value
   stops the run: chores sends `BLOCKED` with the tag `decision` and no
   number.
6. Step 4.5: the cost of row 12.
7. Step 4.6: the command of the run, with the option of row 8 if the card
   chooses it, and no `--detail`.
8. Step 4.7: the 2 checks of STORY-45 spec row 23, as written, and the decoy
   bar of row 8. Then row 14.

### The change to the STORY-45 spec

This PR changes the STORY-45 spec as follows:

- Row 23: the check 2 sentence names `jag-p1` for the first practice test.
  A new last part allows the second practice test: same converted file,
  `jag-p2` in place of `jag-p1`, the same 2 checks, rule hash and converter
  hash, and no third practice test. The source adds D-015 and this spec.
- Row 24: the check of the fixed values runs before each development run.
- Row 25: item 1 adds that the second practice test uses `jag-p2`, with the
  text of `noul-b-v1`. Item 2 adds that the second practice test uses the
  threshold of row 7 of this spec.
- Row 26: a new last part gives the stop rule of D-015 for the second
  practice test.
- Design step 4: a note for the second practice test.

Rows 1 to 22 of the STORY-45 spec, its other steps, and its outcome rules for
the held-out run do not change.

### The files that change

| File | Change |
|---|---|
| `src/scorer.rs` | `PROMPT_VERSION` = `"jag-p2"`; `THRESHOLD` of row 7 |
| `src/scorer/prompt.rs` | Module comment; `instruction` (row 3); shape (row 4); the unit test |
| `src/scorer/CLAUDE.md` | Line 6 names `jag-p2` |
| `src/output.rs` | Line 891: the test string `"prompt_version":"jag-p1"` |
| `tests/output.rs` | Line 603 |
| `tests/plain_grep.rs` | Line 231 |
| `tests/bench.rs` | Line 155: the 4th line; the tests of row 8 |
| `tests/parity/expected/json/pipe.stdout` | Line 1: `"prompt_version"` |
| `README.md` | Line 635: the example of `prompt_version` |
| `docs/bench.md` | Line 129: the 4th line; the section of row 8 |
| `src/bench.rs`, `src/bench_main.rs` | The option of row 8, if the card chooses it |

`src/output.rs` lines 17, 53, 204 and 1122 and `src/bench_main.rs` lines 21
and 200 use the constants, so they do not change. `src/cli/rg_text.rs` line
218 has its own `THRESHOLD` for option names, and it does not change.

### How each criterion is proved

| Criterion | Proof |
|---|---|
| CRITERION-146 | The code PR: `PROMPT_VERSION` is `jag-p2`, and the unit test shows the text of `noul-b-v1` in each question. The 4th line of the development run names `jag-p2`, the threshold, and `DEFAULT_MODEL`. The decoy line of the run is at the bar of METRIC-07 (chores reports `decoy pass`, with no number). The threshold comes from the rule of row 7, written before the run |
| CRITERION-147 | The `DONE` of chores: the run used the frozen rule hash and converter hash (step 5 of "The second development run"), the commit of row 10, and the 2 checks of STORY-45 spec row 23, with the result in words |

## Build order

1. Chores guides the text step of row 3. The orchestrator sends the text, or
   its place on the machine, to the specifier.
2. The specifier closes rows 3 and 4, and rows 7 and 8 from the `ANSWER` of
   the card, and sets STORY-81 to `ready` (row 22). A reviewer reviews the
   spec PR, and the merger merges it.
3. The implementer opens the code PR (rows 2 to 8, 17 and 18). A reviewer
   reviews it, and the merger merges it.
4. Chores guides the second development run (row 11).
5. On a pass: the small threshold PR of row 7, if the card chooses it. Then
   the planner sets STORY-81 to `done` (row 21), and STORY-45 goes on. On a
   fail: row 9 or row 14.

## Assumptions

- Row 2: the series `jag-p<n>` is the naming of jag. Source: `src/scorer.rs`
  line 22.
- Row 4: `noul-b-v1` is shape B with `noul` questions. Source: the name of
  the prompt and the module comment of `src/scorer/prompt.rs`. The text
  confirms or rejects this.
- Rows 7 and 8: the 9 cases with no answer of the development case file are
  the decoy questions of the old set. A converted case has no answer when
  each of its subjects is `absent` (no gold document), or when no line of a
  gold document matches the term (STORY-45 spec, "The conversion rule").
  Before the run, chores counts, from the field names only, the cases whose
  subjects are all `absent`. If that count is not 9, chores sends `BLOCKED`
  with the tag `decision` before the run.
- Row 11: the frozen hashes are on the machine of the maintainer, as STORY-45
  spec row 21 says.
- Row 12: a case with no answer costs about as much as a case with an answer
  (METRIC-05).

## Risks

1. The text of `noul-b-v1` holds text of a case. Signal: the check of the user
   in row 3, or a finding of the `private-data` check. Mitigation: the text
   does not go into a file of this repository; chores sends `BLOCKED` with
   the tag `decision`.
2. The shape of `noul-b-v1` is not shape B. Signal: the text. Mitigation:
   row 4 lists the changes, and a change of the call size goes to the
   architect.
3. Check 2 fails again. Signal: chores reports `check 2 fail`. Mitigation:
   the stop rule of row 9.
4. The decoy AUC is below the bar. Signal: chores reports `decoy fail`.
   Mitigation: no held-out run. Chores sends `BLOCKED` with the tag
   `decision`, and the prompt does not change (row 9).
5. The development case file changed. Signal: a hash of step 5 of "The
   second development run" is different. Mitigation: stop before the run.
6. A setting changes the model. Signal: the 4th line of the settings check.
   Mitigation: STORY-45 spec, row 8.
7. 9 decoy cases give a coarse AUC. Signal: none before the run. Mitigation:
   the card names this con.

## Out of scope

- A pre-rank limit of more than 200 results: D-015 allows it only as an
  option on the `revisit:` card of row 9, because it conflicts with METRIC-05
  and needs a change of ADR-0002 and DECISION-12.
- A third prompt, or a change of `jag-p2` after the development run: D-015.
- A change of the conversion rule, the converter, or the 2 checks: D-008 and
  D-011.
- The held-out run: STORY-45.
- A prompt option or a model option of `jag-bench`: `docs/bench.md` ("These
  values are fixed, so that each run is the same test").

## For the architect

- The prompt version changes inside the rule of ADR-0004. No release holds
  `jag-p1`.
- If row 4 changes the call size of ADR-0002, or the card leaves out the
  decoy run of ADR-0003, a `revisit:` comes to the architect before the code.
- The option of row 8 is bench-only. Releases do not hold `jag-bench`.
