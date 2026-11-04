---
title: "The private-data test: case wf-h allows only the keys name, if, env and run in the merge ref step"
epic: EPIC-05
stories: [STORY-82]
kind: build
gh_issue: 321
lifecycle_state: draft
session: 251aa6d8-5726-4f8b-97c5-0160ecc8a60a
---

# The private-data test: case wf-h allows only the keys name, if, env and run in the merge ref step

## Problem

STORY-82: "As a maintainer, I want the test of the private-data workflow to
fail when the merge ref step gets a key that is not `name`, `if`, `env` or
`run`, so that a later change cannot make that step run in a different way
without a test failure."

The security review of PR #320 (STORY-80) found this, with low severity
(issue #321). Case `wf-h` of `scripts/private-data/test.sh` checks the `env:`
keys, the `run:` line, and that the step `Check the merge ref` has no `uses:`,
secret or token. It does not check the other keys of the step, for example
`shell:` or `working-directory:`.

There is a second gap. Cases `wf-g` and `wf-h` cut the step at the first
blank line or at the first comment line with 6 spaces. In YAML, a blank line
or a comment line does not end a step. So a key after such a line is a key of
the step, but the cases do not see it. A check on 2026-11-04 with a YAML
parser (Ruby `YAML.load_file`) showed this: a step with a blank line and a
comment line between its keys has all its keys.

CRITERION-149: case `wf-h` fails on a copy of the workflow whose merge ref
step has a key other than `name`, `if`, `env` or `run`, and passes on the
workflow of `main`.

## Decisions

| # | Decision | Choice | Source |
|---|---|---|---|
| 1 | The rule | Case `wf-h` fails when the step `Check the merge ref` has a key that is not `name`, `if`, `env` or `run`, or has one of them 2 times. The workflow does not change | STORY-82 page; CRITERION-149; issue #321 |
| 2 | The keys of the step | A new function `merge_ref_step_keys <file>` in `test.sh` prints the keys of the step, 1 on each line. The step starts at the line `      - name: Check the merge ref`, which gives the key `name`. In the step, each line that is not blank and not a comment and has 7, 8 or 9 spaces before its first character is a key line. Its key is the text before the first `:`. The step ends at the first line that is not blank, not a comment, and has 6 spaces or fewer. Lines with 10 spaces or more are values (the `env:` entries) | `.github/workflows/private-data.yml` lines 47 to 53 (the keys of a step have 8 spaces). Inferred |
| 3 | The end of the step | A blank line or a comment line does not end the step (row 2). So a key after such a line is also checked | YAML: a blank line or a comment line does not end a block mapping. The check of 2026-11-04 with Ruby `YAML.load_file` (see Problem) |
| 4 | The check | `wf-h` gets the assert "the step has only the keys name, if, env and run". It passes when the sorted keys of row 2 are exactly `env`, `if`, `name` and `run` | STORY-82 page, scope item 1 |
| 5 | The changed copies | `wf-h` makes 3 copies of the workflow in `$T`, the temp folder of `test.sh`. Each copy adds 1 key to the step, after the line `        run: base/scripts/private-data/merge-ref.sh`. For each copy, 1 assert passes only when the check of row 4 fails on that copy | STORY-82 page, scope item 2; CRITERION-149 |
| 6 | Why a local copy | A `pull_request_target` run uses the workflow file of `main`, so a pull request cannot test a change of that file on GitHub. The cases read the workflow file as text, and the copies are local text files | 0027-approve-each-fork-run-of-the-private-data-check-through-a.md; `test.sh` lines 948 to 950 |
| 7 | Text, not a YAML parser | The function reads the file as text with `awk`, as the other workflow cases do. The runner of `private-data-tests` has no YAML parser step | `test.sh` line 948 ("The cases read .github/workflows/private-data.yml as text"); `.github/workflows/private-data-tests.yml` |
| 8 | Portable `awk` | Write the indentation tests with literal spaces, not with `{7,9}`, so that each `awk` reads them the same way | Inferred: `test.sh` (the other `awk` programs use literal spaces) |
| 9 | What does not change | `.github/workflows/private-data.yml`; the other asserts of `wf-g` and `wf-h`; the cases `wf-a` to `wf-f` and `mr-a` to `mr-e` | STORY-82 page, scope item 3 |
| 10 | The case name | `wf-h: the merge ref step has only the keys name, if, env and run, 3 inputs, no secret, and no token` | Derived from row 4 |
| 11 | Where the test runs | The job `private-data-tests` on each pull request (`.github/workflows/private-data-tests.yml` line 44). On a machine: `GITLEAKS=/path/to/gitleaks FILTER_REPO=/path/to/git-filter-repo scripts/private-data/test.sh` | `.github/workflows/private-data-tests.yml`; `scripts/private-data/README.md` line 153 |
| 12 | The copies hold no secret | The copies are copies of the public workflow with 1 invented line. They hold no secret, no token and no private word | CONSTRAINT-01; `scripts/private-data/CLAUDE.md` ("tests ... with invented values") |
| 13 | Reviews | A reviewer and security review the code PR, because the test guards a workflow that holds a secret | Orchestrator TASK of 2026-11-04; STORY-80 spec, row 16 |
| 14 | Documentation | No change. `scripts/private-data/README.md` does not list the cases | `scripts/private-data/README.md` |
| 15 | ADR | No ADR. A test change is cheap to undo, and ADR 0027 does not change | `docs/adr/CLAUDE.md`; 0027-approve-each-fork-run-of-the-private-data-check-through-a.md |
| 16 | Done | STORY-82 is `done` when the code PR merges. The job `private-data-tests` on that PR proves CRITERION-149 | STORY-82 page; CRITERION-149 |
| 17 | The labels of the page | The STORY-82 page gets the `tags` `security` and `type:story`, the labels of issue #321, so that the sync tool sees no label drift | Orchestrator TASK of 2026-11-04; `prdlib/ghmodel.py` of the product plugin (`labels_of` reads `tags`) |
| 18 | The state of STORY-82 | `ready`: this PR merges the spec, STORY-82 has no feature and no `blocked_by` list, and no card is open. `prd.py` refuses an agent for `backlog` to `ready`, so the change records `human:404pilo` for the session jag-specifier | `docs/adlc.md`, section 2, rules 1, 2 and 4; D-013; D-014 |
| 19 | Spec group | One spec for STORY-82 | Orchestrator TASK of 2026-11-04 |

## Design

### The function

A sketch. The implementer can change the form, but not the rules of row 2:

```bash
# The keys of the step "Check the merge ref" of a workflow file, 1 on each line.
merge_ref_step_keys() { # file
  awk '
    $0 == "      - name: Check the merge ref" { found = 1; print "name"; next }
    !found { next }
    /^ *$/ || /^ *#/ { next }
    !/^       / { exit }
    /^       [^ ]/ || /^        [^ ]/ || /^         [^ ]/ {
      key = $0; sub(/^ +/, "", key); sub(/:.*/, "", key); print key
    }
  ' "$1"
}
merge_ref_step_keys_ok() { # file
  test "$(merge_ref_step_keys "$1" | sort | tr '\n' ' ')" = "env if name run "
}
```

### The copies

Each copy inserts its lines after the line
`        run: base/scripts/private-data/merge-ref.sh`:

| Copy | Inserted lines | What it proves |
|---|---|---|
| 1 | `        shell: bash` | An extra key fails the case |
| 2 | A blank line, then `        working-directory: pr` | A blank line does not hide a key (row 3) |
| 3 | `      # note`, then `        continue-on-error: true` | A comment line does not hide a key (row 3) |

If a later change of the workflow changes the `run:` line, the copies get no
inserted line. Then the check passes on each copy, and the asserts of the
copies fail. So the copies cannot pass with no extra key.

### How the criterion is proved

| Criterion | Proof |
|---|---|
| CRITERION-149 | The job `private-data-tests` on the code PR passes: the assert of row 4 passes on the workflow file, and the 3 asserts of row 5 pass because the check fails on each copy. A prototype of the function on 2026-11-04 gave this result on the workflow of `main` (commit 29c715a) and on the 3 copies, and Ruby `YAML.load_file` showed the extra key in the step of each copy |

## Build order

1. The implementer adds the function, the assert of row 4 and the 3 copies of
   row 5 to case `wf-h`, and changes the case name (row 10).
2. The job `private-data-tests` passes on the PR. A reviewer and security
   review it (row 13). The merger merges it with squash.

## Assumptions

- Row 2: the keys of a step in this workflow have 8 spaces, and a step starts
  with 6 spaces and `- `. Source: `.github/workflows/private-data.yml`.
- Row 8: the `awk` of the runner reads literal spaces the same way as the
  `awk` of macOS. Source: the other `awk` programs of `test.sh` run on both.

## Risks

1. The function misses a key form. Signal: a review finds a YAML form that
   puts a key in the step with a different indentation. Mitigation: a line
   with 7 to 9 spaces is a key line, so an odd indentation fails the case
   and does not pass it.
2. A later change adds a key on purpose. Signal: `wf-h` fails. Mitigation:
   that change also changes `wf-h`, and the review of that PR sees it.

## Out of scope

- A key check of the other steps of the workflow: issue #321 names only the
  merge ref step.
- A change of the block cut of `wf-g` and of the other asserts of `wf-h`: the
  key check of row 4 fails on a step with a `uses:` key after a blank line, so
  that gap is closed by row 4.
- A YAML parser in the runner: row 7.
