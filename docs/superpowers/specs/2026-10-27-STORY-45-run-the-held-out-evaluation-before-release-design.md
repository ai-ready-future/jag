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
| 3 | The case file | The documented case file of `docs/bench.md`. Before the run, the guiding session converts the held-out set to this file on the maintainer's machine | STORY-78 spec, row 2 (the user, hitl card of 2026-10-27, option 1) |
| 4 | The guiding session | The chores session. It guides the user one step at a time, gives the exact commands, checks the result of each step, and asks the user directly (rule 4) | Inferred: `docs/adlc.md`, roles table and the chores details; `scripts/release/README.md` ("The chores role guides the user through these steps") |
| 5 | What the guiding session sees | Only field names, counts, exit codes, the 4 summary lines, and the progress and failure lines of `jag-bench` (these name a case by number and field). The session never prints or reads a question, a term, a file name, a line of a case folder, or a path of the held-out set into its context. Its converter prints only counts. It never reads or prints the files in `~/.config/jag/` | Inferred: DECISION-70; CONSTRAINT-01; README, "When jag cannot run" ("Never read or print the files in `~/.config/jag/`"); STORY-78 spec, row 17 |
| 6 | The place of the data | The held-out set, the converter, the case file and each copy stay outside the clone of jag. Nothing of them goes into a commit, a PR, an issue, a message to another session, or a log | DECISION-70; STORY-78 spec, row 19 (exit 2 for a case file in the source folder) |
| 7 | The run commit | A commit of `main` whose `Cargo.toml` has the release version. The checkout is clean (`git status --porcelain` is empty). Between the run commit and the tag commit, only `README.md` and files under `docs/` change. From the run to the tag, the orchestrator merges no PR that changes another file | Inferred: `scripts/release/README.md`, "The version" and "Before the first tag"; `docs/bench.md` (the 4th line names the jag version); DECISION-70 (frozen prompt and threshold) |
| 8 | The settings check | Before chores opens the held-out set, the user runs `jag-bench` on a temp copy of the invented cases of `tests/bench/`. Its 4th line must name `PROMPT_VERSION` and `THRESHOLD` of `src/scorer.rs`, `DEFAULT_MODEL` of `src/config/model.rs`, and the release version. If the model is different, the user removes the model setting and runs the check again | Inferred: DECISION-70 ("no setting change"); `docs/bench.md`, "Output"; `tests/bench/cases.yaml` |
| 9 | The cost | Before the run, chores counts the cases with an answer on the machine, and tells the user the expected cost: about $0.014 for each case. The upper limit: each case scores at most `DEFAULT_COST_LIMIT` (200) results. The user says yes before the run. The spec gives no count | METRIC-05 ($1.13 for 83 questions); `src/config/limit.rs`; README, "The cost of a search" |
| 10 | Per-case results | The run uses no `--detail` file. Nobody reads a per-case result | Inferred: DECISION-70 ("nothing is tuned on the result"). A per-case result has no other use |
| 11 | One time | The run with exit 0 or 1 is the one run. Exit 2 (before the first model call), 3 to 6, or 130 prints no score, so a new run after the fix of the cause is still the one run | STORY-78 spec, row 21; `docs/bench.md`, "Exit codes" |
| 12 | The bar | `bar pass` (exit 0): the work goes on to row 13. `bar fail` (exit 1): row 17 | DECISION-70; CRITERION-73; `docs/bench.md` (the bar of `jag-bench` is the bar of CRITERION-73) |
| 13 | The public record | After a `bar pass`, chores opens 1 docs PR. It adds the README line of row 14, and the section "Held-out evaluation" to `docs/release-readiness.md` with the run commit, the date, and the 4 summary lines. A reviewer reviews it, and the `private-data` check scans it | Inferred: DECISION-70; STORY-78 spec, row 16 and "The output" ("These 4 lines are the public-safe summary"); `docs/release-readiness.md` (the record of the checks before the first publish) |
| 14 | The README line | One line at the end of the section "What jag is", with the template of "The public text". The section "Examples" does not change, because `tests/readme_examples.rs` reads it | DECISION-70 ("one README line"); STORY-78 spec, row 16 |
| 15 | The release notes | At the release (U6 of `scripts/release/README.md`), after the job `github-draft` makes the draft, chores guides the user: get the generated notes, put the section of "The public text" before them, and write the result back. `gh release edit --notes-file` replaces the full text, so the first step gets the old text. Do this before the user approves `github-publish`. If the release is published already, the same commands work. `release.yml` does not change | Inferred: `.github/workflows/release.yml` line 404 (`--generate-notes`); GitHub docs, "Immutable releases" ("You can still edit the title and release notes of a published immutable release") |
| 16 | The order | The run, then the docs PR of row 13, then the list "Before the first tag" on the new commit, then U4 (npm part), U5 and U6. The run comes before the npm part of U4, because a new npm trusted publisher expires when no publish uses it in 2 days | Inferred: `scripts/release/README.md`, U4 to U6 and "Before the first tag" |
| 17 | A failed bar | No tag. The numbers stay on the machine: no README line, no release notes, no record. Chores sends `BLOCKED` with the tag `decision` to the orchestrator, with the words "bar fail" and no number. The planner writes a fix story | DECISION-70 ("A fix story comes first"); `docs/adlc.md`, rule 14 |
| 18 | Who does the work | No implementer. The orchestrator sends a chore request with this spec to chores. Chores guides the run, opens the docs PR of row 13, and later does the notes step of row 15 at U6 | Inferred: `docs/adlc.md`, roles table (chores: "A manual step is necessary") |
| 19 | Spec group | One spec for STORY-45 | Orchestrator TASK of 2026-10-27 |
| 20 | The state of STORY-45 | `ready`: its spec is in this PR, STORY-78 is `done`, and no card is open | `docs/adlc.md`, rule 11 |

## Design

### The run steps

Chores gives each command to the user, one at a time, and checks the result
before the next step. `<clone>` is the clone of jag. `<work>` is a folder
outside the clone, on the maintainer's machine. Chores writes each command
with the placeholders `<work>`, `<old set>` and `<case file>`, and the user
puts in the real paths. So no path of the held-out set goes into the
context of chores (row 5).

1. **The run commit.** In `<clone>`, check out the run commit (row 7) and
   check that `git status --porcelain` is empty. Record the full commit
   hash.
2. **Build.** `cargo build --release --features bench --bin jag-bench`.
3. **The settings check (row 8).** Copy `tests/bench/` to a new temp folder
   outside the clone, and run `jag-bench` on the copy of `cases.yaml`.
   Chores reads the 4th line. The ndcg@10 values of this check have no
   meaning, because the invented cases are small.
4. **The conversion (row 3).** Chores writes a converter in `<work>`. To
   learn the old format, it prints only the field names and the type of each
   field, never a value. The user gives the path of the old set to the
   converter as an argument (`<old set>`). The converter writes the case
   file in `<work>` and prints only these counts:
   - the cases in the old set, and the cases in the new file
   - the labeled lines in the old set, and in the new file
   - the labeled lines that hold a match of the term of their case

   The first two pairs must be equal. A large gap in the third count shows a
   line number that is off by one (0-based against 1-based). Fix the
   converter and convert again.
5. **The cost (row 9).** Chores gives the expected cost. The user says yes,
   or stops.
6. **The run.** `<clone>/target/release/jag-bench <work>/<case file>`, with
   no `--detail`. Chores reads the exit code, the 4 summary lines, and the
   failure lines. On exit 2, fix the case file or the command and run again
   (row 11). On exit 3 to 6 or 130, fix the cause and run again.
7. **The result (row 12).** On exit 0, go to the public record. On exit 1,
   do row 17.

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
2. Chores guides the run steps 1 to 7.
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

1. The conversion is wrong, and the score is wrong. Signal: a pair of counts
   in step 4 is not equal, many labeled lines hold no match of their term,
   or `jag-bench` exits with 2. Mitigation: fix the converter before the run.
   No score is seen, so the run is still the one run.
2. A setting changes the model. Signal: the 4th line of step 3 names a
   model that is not `DEFAULT_MODEL`. Mitigation: row 8, before any held-out
   case runs.
3. Case text goes into the context of a session. Signal: a command prints a
   value of the held-out set. Mitigation: row 5; the converter prints only
   counts. If it happens, chores tells the user at once, and no text of that
   context goes into a public place.
4. Code changes between the run and the tag. Signal: the diff of row 7
   shows a file outside `README.md` and `docs/`. Mitigation: the orchestrator
   holds such PRs. If a change is necessary, the orchestrator asks the user
   through hitl what to do, because a second run is not in DECISION-70.
5. The cost is higher than expected. Signal: the cost on the account of the
   user after the run. Mitigation: row 9; `DEFAULT_COST_LIMIT` limits each
   case.
6. The bar fails. Signal: exit 1. Mitigation: row 17. Release 1 waits for
   the fix story.
7. The section is not in the release notes. Signal: the last command of the
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
