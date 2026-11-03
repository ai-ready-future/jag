---
title: "The held-out evaluation: one private run of jag-bench before the first tag"
epic: EPIC-02
stories: [STORY-45]
kind: build
gh_issue: 70
lifecycle_state: draft
inherits: [QS-1, C-bench]
session: 4acb9f7a-4603-41fb-8e74-395e5117e288
---

# The held-out evaluation: one private run of jag-bench before the first tag

## Problem

STORY-45: "As an agent builder (PERSONA-01), I want the maintainer to run
the Rust jag once on the private held-out split before release 1, so that I
can trust that jag answers better than grep."

DECISION-70 ("Run Rust jag once, privately") gives the rule. The held-out
set is private case material that is not jag's. It stays on the
maintainer's machine and never goes into this repository (CONSTRAINT-01).
Before the first release tag, the maintainer runs the Rust jag one time on
the held-out split, with a session that guides the run. Only a summary of
the numbers goes public: the release notes and one README line.

CRITERION-73: one local run of the Rust jag on the held-out split gives an
ndcg@10 of 0.70 or more, higher than blind grep. Only the numbers go
public. A lower score means no release tag.

The run tool is `jag-bench` of STORY-78 (`docs/bench.md`). This spec gives
the steps of the run, the checks before it, and the path of the summary to
the README and the release notes. It adds no code.

## Decisions

| # | Decision | Choice | Source |
|---|---|---|---|
| 1 | The rule | The maintainer runs the held-out split one time, on their own machine, with a guiding session. No CI job runs it. The prompt, the threshold and the 0.70 bar are frozen. The held-out split chooses nothing, and nothing is tuned on the result. Only a summary of the numbers goes public. The data path, the key and the per-case results stay on the machine. Below 0.70, or not higher than blind grep: no tag, and a fix story comes first | DECISION-70; CRITERION-73; CONSTRAINT-01 |
| 2 | The run tool | `jag-bench`, built from a clean checkout of the run commit (row 7) with `cargo build --release --features bench --bin jag-bench` | STORY-78 spec, rows 3 and 4; `docs/bench.md`, "Build" |
| 3 | The case file | The documented case file of `docs/bench.md`. Before the run, the guiding session converts the held-out set to this file on the maintainer's machine, with the converter of row 22. The development set goes through the same converter first (row 21) | STORY-78 spec, row 2 (the user, hitl card of 2026-10-27, option 1); rows 21 and 22 |
| 4 | The guiding session | The chores session. It guides the user one step at a time, gives the exact commands, checks the result of each step, and asks the user directly (rule 4) | Inferred: `docs/adlc.md`, roles table and the chores details; `scripts/release/README.md` ("The chores role guides the user through these steps") |
| 5 | What the guiding session sees | Only field names, counts, exit codes, the 4 summary lines, and the progress and failure lines of `jag-bench` (these name a case by number and field). The session never prints or reads a question, a term, a file name, a line of a case folder, or a path of the held-out set into its context. Its converter prints only counts. It never reads or prints the files in `~/.config/jag/`. The same rules apply to the development set (row 21), because it is also private case material that is not jag's | Inferred: DECISION-70; CONSTRAINT-01; README, "When jag cannot run" ("Never read or print the files in `~/.config/jag/`"); STORY-78 spec, row 17 |
| 6 | The place of the data | The held-out set, the development set, the converter, each case file and each copy stay outside the clone of jag. Nothing of them goes into a commit, a PR, an issue, a message to another session, or a log | DECISION-70; STORY-78 spec, row 19 (exit 2 for a case file in the source folder) |
| 7 | The run commit | A commit of `main` whose `Cargo.toml` has the release version. The checkout is clean (`git status --porcelain` is empty). Between the run commit and the tag commit, only `README.md` and files under `docs/` change. From the run to the tag, the orchestrator merges no PR that changes another file | Inferred: `scripts/release/README.md`, "The version" and "Before the first tag"; `docs/bench.md` (the 4th line names the jag version); DECISION-70 (frozen prompt and threshold) |
| 8 | The settings check | Before chores opens the held-out set, the user runs `jag-bench` on a temp copy of the invented cases of `tests/bench/`. Its 4th line must name `PROMPT_VERSION` and `THRESHOLD` of `src/scorer.rs`, `DEFAULT_MODEL` of `src/config/model.rs`, and the release version. If the model is different, the user removes the model setting and runs the check again | Inferred: DECISION-70 ("no setting change"); `docs/bench.md`, "Output"; `tests/bench/cases.yaml` |
| 9 | The cost | Before the run, chores counts the cases with an answer on the machine, and tells the user the expected cost: about $0.014 for each case. The upper limit: each case scores at most `DEFAULT_COST_LIMIT` (200) results. The user says yes before the run. The spec gives no count. The development run of row 21 also costs money. A case with no answer makes no model call, so it costs about 74 × $0.014 ≈ $1.05. The user says yes before that run too | METRIC-05 ($1.13 for 83 questions); `src/config/limit.rs`; README, "The cost of a search" |
| 10 | Per-case results | The run uses no `--detail` file. Nobody reads a per-case result | Inferred: DECISION-70 ("nothing is tuned on the result"). A per-case result has no other use |
| 11 | One time | The run with exit 0 or 1 is the one run. Exit 2 (before the first model call), 3 to 6, or 130 prints no score, so a new run after the fix of the cause is still the one run. A run on the development set (row 21) is not the one run | STORY-78 spec, row 21; `docs/bench.md`, "Exit codes" |
| 12 | The bar | `bar pass` (exit 0): the work goes on to row 13. `bar fail` (exit 1): row 17 | DECISION-70; CRITERION-73; `docs/bench.md` (the bar of `jag-bench` is the bar of CRITERION-73) |
| 13 | The public record | After a `bar pass`, chores opens 1 docs PR. It adds the README line of row 14, and the section "Held-out evaluation" to `docs/release-readiness.md` with the run commit, the date, and the 4 summary lines. A reviewer reviews it, and the `private-data` check scans it | Inferred: DECISION-70; STORY-78 spec, row 16 and "The output" ("These 4 lines are the public-safe summary"); `docs/release-readiness.md` (the record of the checks before the first publish) |
| 14 | The README line | One line at the end of the section "What jag is", with the template of "The public text". The section "Examples" does not change, because `tests/readme_examples.rs` reads it | DECISION-70 ("one README line"); STORY-78 spec, row 16 |
| 15 | The release notes | At the release (U6 of `scripts/release/README.md`), after the job `github-draft` makes the draft, chores guides the user: get the generated notes, put the section of "The public text" before them, and write the result back. `gh release edit --notes-file` replaces the full text, so the first step gets the old text. Do this before the user approves `github-publish`. If the release is published already, the same commands work. `release.yml` does not change | Inferred: `.github/workflows/release.yml` line 404 (`--generate-notes`); GitHub docs, "Immutable releases" ("You can still edit the title and release notes of a published immutable release") |
| 16 | The order | The run, then the docs PR of row 13, then the list "Before the first tag" on the new commit, then U4 (npm part), U5 and U6. The run comes before the npm part of U4, because a new npm trusted publisher expires when no publish uses it in 2 days | Inferred: `scripts/release/README.md`, U4 to U6 and "Before the first tag" |
| 17 | A failed bar | No tag. The numbers stay on the machine: no README line, no release notes, no record. Chores sends `BLOCKED` with the tag `decision` to the orchestrator, with the words "bar fail" and no number. The planner writes a fix story | DECISION-70 ("A fix story comes first"); `docs/adlc.md`, rule 14 |
| 18 | Who does the work | No implementer. The orchestrator sends a chore request with this spec to chores. Chores guides the run, opens the docs PR of row 13, and later does the notes step of row 15 at U6 | Inferred: `docs/adlc.md`, roles table (chores: "A manual step is necessary") |
| 19 | Spec group | One spec for STORY-45 | Orchestrator TASK of 2026-10-27 |
| 20 | The state of STORY-45 | `ready`: its spec is on `main`, STORY-78 is `done`, and no card is open. The card of row 23 has its ANSWER (D-008) | `docs/adlc.md`, section 2, rule 1 |
| 21 | Test the rule first | Before chores opens the held-out set, the user converts the development set (83 cases) with the converter of row 22 and runs `jag-bench` on it. The result goes to the gate of row 23. Before the development run, chores freezes the rule and the converter: it records the rule hash and the SHA-256 of the converter, and prints only the first 12 characters of each. After a pass of both checks, the user converts the held-out set with the same converter, chores checks that the hashes are the same, and the one run follows. The rule and the converter do not change after the development scores are seen | human:404pilo, 2026-11-03 (D-001, hitl: "test the rule on the development set, then freeze it, then convert the private set"); knowledge page `synthesis/story-45-test-the-labeling-rule-on-the-development-set-first` (the first test recorded a rule hash of 12 characters) |
| 22 | The conversion rule | The method of the earlier evaluation tool, mapped to the labeled lines of `jag-bench`. The steps are in "The conversion rule" (Design). This rule replaces the rule of the first development test: the rule under test of D-001 (the term is the first `checks.mustContain` entry, and each answer line has grade 1), D-003 and D-004. D-005 rejected that rule after its development test. The order of D-001 stays (row 21) | Planner decision of 2026-11-03, path (a) "convert with the old method" (state store D-006, story STORY-45). It follows D-001, D-003, D-004 and D-005. No PRD or bar change. STORY-78 spec, row 2 ("The score can differ a little from the result of the earlier bench harness"); knowledge page `concepts/evaluation-method-of-the-earlier-search-tool` (PR #284, open; a researcher study of 2026-11-03, veracity 60) |
| 23 | The development gate | The practice test runs `jag-bench` once on the converted development case file (74 cases with answers, 9 no-answer cases). `jag-bench` computes both scores: the score of a tool is the mean of its ndcg@10 over the cases with an answer (`docs/bench.md`, "How the tool scores a case"). The pass rule is written here before the run, and nothing is tuned on the result. 1. Label check (blind grep): the blind grep score must be 0.10 or less. The old score was 0.06. Blind grep uses no model, so the prompt cannot move this score. It tests the labels and the metric. 2. Bar check (jag): the jag score must be 0.70 or more, which is the release bar of DECISION-70 and CRITERION-73. It uses the fixed prompt (`jag-p1`) and threshold of `jag-bench`. A jag score above the old score (0.73) is not a failure. Because check 1 passed, a jag score of 0.70 or more is also higher than blind grep, so the `bar` line of `jag-bench` says `bar pass`. Outcome: if check 1 fails, the labels are wrong: stop, there is no private run, and the planner chooses the next path. If check 1 passes and check 2 fails, there is no private run, and a fix story comes first (the planner writes it). If both pass, freeze the rule and the converter (the rule hash and the converter hash as chores recorded them, row 21), convert the private set, and do the one private run. The 2 checks and the conversion rule are frozen before the practice scores are seen, and they do not change after the scores | human:404pilo, 2026-11-03 (D-008, hitl card "How close the practice scores must be", option "Two checks, written before the run", no condition); planner decision of 2026-11-03 (state store D-006, story STORY-45); D-001 and D-005 (frozen before the scores). The old scores: `prd:/metrics/METRIC-01.md` on `main` (jag about 0.72, blind grep about 0.06); the PR #284 page (open: jag 0.73, blind 0.06) |
| 24 | The fixed values of the old run | Before the development run, chores checks the fixed values of `jag-bench` against the old run: the prompt, the threshold 0.4, and no per-file cap. Chores reports each difference to the orchestrator before the run. Row 25 lists the differences that are known on 2026-11-03 | Planner decision of 2026-11-03 (state store D-006, story STORY-45); `src/scorer.rs` (`PROMPT_VERSION`, `THRESHOLD`) |
| 25 | The differences from the old run | 1. The prompt: `jag-bench` uses `jag-p1`. The old run used `noul-b-v1`. 2. The threshold 0.4 and no per-file cap: the same in both. 3. The pre-rank: jag pre-ranks to 200 results when a search has more (`DEFAULT_COST_LIMIT`). The old run scored all results. 4. The credit: `jag-bench` gives credit to a labeled matching line, computes the ideal score from all labeled lines cut at 10, and counts each blind grep line as 1 result. The old tool gave credit to merged windows (snippets) and computed the ideal score over the snippets that grep found. The 2 checks of row 23 allow for these differences: check 1 uses no model, and check 2 is the release bar | Chores, the check of 2026-11-03, and the planner (orchestrator TASK of 2026-11-03); `src/scorer.rs`; `src/config/limit.rs` (`DEFAULT_COST_LIMIT`); `docs/bench.md`, "How the tool scores a case"; the PR #284 page |
| 26 | A failed development gate | No held-out set is converted and no run occurs. Chores sends `BLOCKED` with the tag `decision` to the orchestrator, with the name of the failed check and no number. If check 1 fails, the planner chooses the next path, for example a `jag-bench` story that scores snippets, through a card. If only check 2 fails, the planner writes a fix story | D-008 (the outcome rules of row 23); planner decision of 2026-11-03 (state store D-006, story STORY-45: "if the dev gate fails again, a snippet-score story with a card"); row 17 (no number in the message) |

## Design

### The run steps

Chores gives each command to the user, one at a time, and checks the result
before the next step. `<clone>` is the clone of jag. `<work>` is a folder
outside the clone, on the maintainer's machine. Chores writes each command
with the placeholders `<work>`, `<dev set>`, `<dev file>`, `<old set>` and
`<case file>`, and the user puts in the real paths. `<dev set>` is the
development set and `<old set>` is the held-out set, both in the old
format. So no path of either set goes into the context of chores (row 5).

1. **The run commit.** In `<clone>`, check out the run commit (row 7) and
   check that `git status --porcelain` is empty. Record the full commit
   hash.
2. **Build.** `cargo build --release --features bench --bin jag-bench`.
3. **The settings check (row 8).** Copy `tests/bench/` to a new temp folder
   outside the clone, and run `jag-bench` on the copy of `cases.yaml`.
   Chores reads the 4th line. The ndcg@10 values of this check have no
   meaning, because the invented cases are small.
4. **The development test (rows 21 to 26).**
   1. The fixed values (row 24). Chores compares the prompt and the
      threshold 0.4 in the 4th line of step 3 with the old run. The 4th
      line does not show a per-file cap: `jag-bench` has none
      (`src/bench.rs`). Chores reports each difference to the orchestrator
      before the next step, also the known differences of row 25.
   2. The converter. Chores writes a converter in `<work>` that does "The
      conversion rule". To learn the old format, it prints only the field
      names and the type of each field, never a value.
   3. The conversion. The user gives `<dev set>` and `<work>/<dev file>` to
      the converter as arguments. The converter prints only the counts of
      "The counts of the converter". The development set must give 74 cases
      with an answer and 9 cases with no answer. If it gives other counts,
      find the cause before step 4.4. If the cause is an error in the
      converter, fix it and convert again. Else stop and send `BLOCKED`
      with the tag `decision` and the 2 counts. No score is seen yet, so
      this is not tuning.
   4. The freeze (row 21). Chores records the rule hash and the SHA-256 of
      the converter, and prints only the first 12 characters of each.
   5. The cost (row 9). The user says yes, or stops.
   6. The development run. `<clone>/target/release/jag-bench
      <work>/<dev file>`, with no `--detail`. Chores reads the exit code and
      the 4 summary lines.
   7. The gate (row 23). Chores applies the 2 checks. If both pass, go to
      step 5. Else do row 26 and stop.
5. **The conversion of the held-out set (row 3).** Chores checks that the
   rule hash and the SHA-256 of the converter are the frozen values. The user gives `<old set>`
   and `<work>/<case file>` to the converter. The converter prints the
   same counts. The spec gives no expected count for the held-out set
   (row 9).
6. **The cost (row 9).** Chores gives the expected cost. The user says yes,
   or stops.
7. **The run.** `<clone>/target/release/jag-bench <work>/<case file>`, with
   no `--detail`. Chores reads the exit code, the 4 summary lines, and the
   failure lines. On exit 2, fix the case file or the command and run again
   (row 11). On exit 3 to 6 or 130, fix the cause and run again.
8. **The result (row 12).** On exit 0, go to the public record. On exit 1,
   do row 17.

### The conversion rule

The converter makes 1 case of the case file for each case of the old set
(row 22). It uses the field names of the old format only to find the data:
`expected.documents`, the `form` of each subject, the expected values, and
`checks.mustContain`. It never prints a value.

1. **The term.** Use the term rule of the earlier tool on the text of the
   question:
   1. Split the text at each character that is not a letter or a digit.
   2. Remove the short tokens, the question words and the function words.
   3. Change a simple plural to its stem.
   4. Keep the first 12 unique terms.

   Escape each term as a literal in the pattern syntax of jag, and join the
   terms with `|` into 1 pattern. The case gets `ignore_case: true`, and
   `fixed_strings` stays `false`. The minimum length, the list of words and
   the plural rule are those of the earlier tool (see "Assumptions").
2. **The evidence lines.** Use the labels v1 of the earlier tool:
   1. The gold documents are the documents in `expected.documents` of each
      subject whose `form` is not `absent`.
   2. The needles are the expected values of each subject, split at `;`.
      The converter looks for such a needle only in the documents of that
      subject.
   3. The entries of `checks.mustContain` are also needles. A list entry
      gives each of its strings. The converter looks for such a needle in
      all gold documents.
   4. Keep a needle only if it has a digit or 5 characters or more.
   5. A needle that matches more than 5 lines of a document marks no line
      of that document.
   6. A needle that holds a line break marks no line.

   An evidence line is a line of a gold document that a kept needle marks.
3. **The labels.** Map the snippet credit of the earlier tool to lines:
   - Grade 2: each line of a gold document that matches the term of step 1
     and is at most 3 lines from an evidence line of the same document.
   - Grade 1: each other line of a gold document that matches the term.

   A line gets 1 label only, with the higher grade. An evidence line that
   does not match the term gets no label, because `jag-bench` gives credit
   only to a matching line (`docs/bench.md`, "How the tool scores a case").
   Line numbers start at 1.
4. **No answer.** A case with no label after step 3 gets `answers: []`. It
   stays in the case file and is not in the mean.
5. **An empty term.** A case whose term is empty after step 1 gets
   `answers: []` and the term `no-term` (see "The counts of the converter").

### The counts of the converter

The converter prints only these counts:

- the cases in the old set, and the cases in the case file (they must be
  equal)
- the cases with an answer, and the cases with no answer (the development
  set: 74 with an answer and 9 with no answer, step 4.3)
- the labeled lines of grade 2, and of grade 1
- the needles that it kept, and the needles that it removed because they
  were short, matched more than 5 lines of a document, or held a line break
- the cases whose term is empty after step 1 of "The conversion rule"

A case whose term is empty after step 1 of "The conversion rule" has no
matching line, so it is a case with no answer. The converter writes
`answers: []` and the fixed term `no-term`, because `jag-bench` needs a term
that is not empty for each case (`docs/bench.md`, "The case file").
`jag-bench` does not search for the term of a case with no answer. This
count is only for information.

### The public text

The README line (row 14). `<N>`, `<J>`, `<G>`, `<P>`, `<T>`, `<M>` and
`<version>` are the values of the 4 summary lines:

```
On a private held-out set of <N> cases, jag <version> got an ndcg@10 of <J>, and a blind grep got <G> (prompt <P>, threshold <T>, model <M>).
```

The section of `docs/release-readiness.md` (row 13):

````
## Held-out evaluation

Criterion: CRITERION-73 (STORY-45). Rule: DECISION-70.

- Run commit: <full hash>
- Date: <date>
- Result: bar pass

```
<the 4 summary lines of jag-bench, as printed>
```
````

The section of the release notes (row 15). It goes before the generated
notes:

````
## Held-out evaluation

Before this release, the maintainer ran jag-bench one time on a private held-out set (DECISION-70). The summary:

```
<the 4 summary lines, copied from docs/release-readiness.md>
```
````

### The notes step at U6

When the job `github-draft` has made the draft `v<version>`, chores gives
these commands. `<work>` is outside the clone:

```
$ gh release view v<version> --json body -q .body > <work>/generated.md
$ cat <work>/held-out.md <work>/generated.md > <work>/notes.md
$ gh release edit v<version> --notes-file <work>/notes.md
$ gh release view v<version> --json body -q .body
```

`<work>/held-out.md` holds the section of the release notes. The last
command shows the result. The user checks that the section is there and
that the generated notes follow it, and then approves `github-publish`.

### How each criterion is proved

| Criterion | Proof |
|---|---|
| CRITERION-73 | The 4 summary lines of the run show `bar pass` and the values of row 8. The record in `docs/release-readiness.md` holds them with the run commit. `git diff --name-only <run commit> <tag commit>` shows only `README.md` and files under `docs/`. The docs PR passes the `private-data` check and a review. After U6, `gh release view v<version>` shows the section of the release notes |

## Build order

1. The orchestrator sends a chore request with this spec to chores, when
   `main` has the release version and the other open work for release 1 is
   merged.
2. Chores guides the run steps 1 to 8.
3. On `bar pass`: chores opens the docs PR of row 13. A reviewer reviews
   it. After the approval, the orchestrator merges it with squash. STORY-45 is then
   `done`. The notes step of row 15 is a step of U6, which STORY-51 runs,
   because STORY-51 waits for STORY-45.
4. Chores does the list "Before the first tag" on the new commit, as
   `scripts/release/README.md` says.
5. U4 (npm part), U5 and U6 follow (STORY-51). At U6, chores does the notes
   step.

## Assumptions

- Row 4: the chores session can run on the maintainer's machine and talk
  to the user, as for U1 to U6 (`scripts/release/README.md`).
- Row 5: the converter can learn the old format from field names and types
  alone. If it cannot, chores stops and sends `BLOCKED` with the tag
  `decision`, and does not read a value.
- Row 22: the minimum token length, the list of question and function
  words, and the plural rule of the term come from the code of the earlier
  tool. Chores reads that code only as section 1 of `docs/adlc.md` says
  (the row "Extra context files" and the repository rules). If chores
  cannot read it, it stops and sends `BLOCKED` with the tag `decision`.
- Row 22: the field names of the old format in "The conversion rule" are
  names only. They show no value of a case.
- Row 7: no code change is necessary between the run and the tag. The
  readiness list has no item that changes code.
- Row 8: the invented cases of `tests/bench/` run against the real model
  service, because `jag-bench` reads the key and the settings as jag does
  (`docs/bench.md`).
- Row 9: the cost of a held-out case is near the cost of a development
  question (METRIC-05).
- Row 10: DECISION-70 lets the per-case results stay on the machine. It does
  not ask for them.
- Row 15: the job `github-publish` waits for the approval of the user,
  because the environment `release` has a required reviewer.

## Risks

1. The conversion is wrong, and the score is wrong. Signal: the case counts
   of "The counts of the converter" are not equal, the development set does
   not give 74 cases with an answer and 9 with no answer, or
   `jag-bench` exits with 2. Mitigation: fix the converter before the
   development run. Then the development gate (row 23) checks the
   converter before the held-out set is converted. No held-out score is
   seen, so the run is still the one run.
2. The development gate fails again. Signal: the development scores do
   not pass the 2 checks of row 23. Mitigation: row 26. The held-out set
   stays closed.
3. A setting changes the model. Signal: the 4th line of step 3 names a
   model that is not `DEFAULT_MODEL`. Mitigation: row 8, before any held-out
   case runs.
4. Case text goes into the context of a session. Signal: a command prints a
   value of the held-out set. Mitigation: row 5; the converter prints only
   counts. If it happens, chores tells the user at once, and no text of that
   context goes into a public place.
5. Code changes between the run and the tag. Signal: the diff of row 7
   shows a file outside `README.md` and `docs/`. Mitigation: the orchestrator
   holds such PRs. If a change is necessary, the orchestrator asks the user
   through hitl what to do, because a second run is not in DECISION-70.
6. The cost is higher than expected. Signal: the cost on the account of the
   user after the run. Mitigation: row 9; `DEFAULT_COST_LIMIT` limits each
   case.
7. The bar fails. Signal: exit 1. Mitigation: row 17. Release 1 waits for
   the fix story.
8. The section is not in the release notes. Signal: the last command of the
   notes step. Mitigation: the notes of an immutable release stay editable,
   so the step can follow the publish.

## Out of scope

- A new held-out run after a fix: the fix story asks the user through hitl
  what that run must show.
- The tag and the publish: STORY-51 and U4 to U6.
- A held-out run for a later release: DECISION-70 asks for one run before
  the first tag.
- Changes to `jag-bench`: STORY-78 is done.
- A decoy measure (METRIC-07).

## For the architect

- This spec adds no code, no option and no interface. `release.yml` does
  not change.
- The release notes step is a manual step of U6. It needs no ADR.
- The no-tag rule is the rule of DECISION-70. No release code checks it, and
  0010-ship-jag-on-github-releases-crates-io-and-npm.md does not change.
