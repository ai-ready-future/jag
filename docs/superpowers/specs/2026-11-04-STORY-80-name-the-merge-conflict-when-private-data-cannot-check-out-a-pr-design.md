---
title: "The private-data check names the merge conflict: one step that finds the merge ref before the checkout of the pull request"
epic: EPIC-05
stories: [STORY-80]
kind: build
gh_issue: 311
lifecycle_state: draft
session: 251aa6d8-5726-4f8b-97c5-0160ecc8a60a
---

# The private-data check names the merge conflict: one step that finds the merge ref before the checkout of the pull request

## Problem

STORY-80: "As a maintainer, I want the private-data check to say that a
pull request conflicts with main when it cannot check it out, so that I fix
the conflict and not the check."

When a pull request conflicts with its base branch, GitHub makes no merge
ref (`refs/pull/<n>/merge`). The second checkout of
`.github/workflows/private-data.yml` (line 49, on `pull_request_target`)
then stops with `fatal: couldn't find remote ref refs/pull/<n>/merge`
(exit 128). The required check is red, and the message does not give the
cause. PR #307 had this failure 3 times on 2026-11-04 (runs 41838226461,
41838578798 and 41838594649). The runs passed after main was merged into
the branch.

The criteria:

- CRITERION-144: when a pull request conflicts with main and GitHub has no
  merge ref for it, the check fails before the checkout with the message
  that the pull request conflicts with main and that main must be merged
  into the branch.
- CRITERION-145: when the merge ref exists, the check checks out and scans
  the pull request as before. The new step runs, sources and reads no file
  of the pull request.
- CRITERION-148 (new in this PR): when GitHub makes the merge ref during the
  wait of the new step, the check checks out and scans the pull request as
  before.

## Decisions

No row is open, so this spec sends no card. Each row has a source that a
reviewer can open.

| # | Decision | Choice | Source |
|---|---|---|---|
| 1 | The rule | 1 step before the checkout of the pull request finds out if the merge ref exists. If it does not exist after a short wait, the step fails with a message that names the pull request and the conflict. The step does not check out, run, source or read a file of the pull request. The rules at the top of the workflow stay. A pull request with no conflict runs as before | `roadmap:/stories/STORY-80.md` (planner, PR #308); CRITERION-144; CRITERION-145 |
| 2 | The place of the step | A new step, `Check the merge ref`, after the first checkout (`base`) and before the checkout of the pull request (`pr`). It has the same condition as that checkout: `if: github.event_name == 'pull_request_target'`. On a push to `main` it does not run | Derived from row 1. `.github/workflows/private-data.yml` lines 39 to 54 |
| 3 | The code of the step | A new script, `scripts/private-data/merge-ref.sh`. The step runs `base/scripts/private-data/merge-ref.sh`, so the script comes from `main`, as `check.sh` does. The test script can run it with a local repository (row 13) | STORY-46 spec, decision 15 ("The workflow, `check.sh`, and the gitleaks configs come from `main`"); `scripts/private-data/test.sh` |
| 4 | How the step finds the merge ref | `git ls-remote --exit-code "$REPO_URL" "refs/pull/$PR_NUMBER/merge"`. It sends no token: the repository is public, and the step needs no new permission. It reads only the names and the commits of the refs on GitHub. It fetches no commit and reads no file | Story scope ("It finds out if the merge ref exists"); `git help ls-remote` (`--exit-code`: "Exit with status "2" when no matching refs are found"); STORY-46 spec, decision 15 (`permissions: contents: read`, no token to files of the pull request) |
| 5 | The wait | The first check runs at once. When the ref is not there, the step waits 10 seconds and checks again, at most 6 checks in all (at most 50 seconds of wait). `MERGE_REF_TRIES` (default 6) and `MERGE_REF_PAUSE` (default 10, in seconds) change the values. The workflow sets neither. The tests set them | Story scope ("GitHub can make the merge ref some seconds after a push. ... The spec gives the wait"); STORY-46 spec, design step 0 ("GitHub makes `refs/pull/<number>/merge` a short time after a push"); GitHub docs, "Using the REST API to interact with your Git database" (a background job makes the test merge commit; the docs give no time) |
| 6 | The message of a conflict | After the last check, when `git ls-remote` gave exit 2 each time: the line `::error::private-data: PR #<n> conflicts with <base>: merge <base> into the branch`, and exit 1. `<n>` is the number of the pull request. `<base>` is the base branch of the pull request (`main` for a pull request into `main`). The `::error::` line shows the message on the checks page of the pull request | Story ("PR #<n> conflicts with main: merge main into the branch"); CRITERION-144; STORY-77 spec, row 7 (a same-repo pull request into any branch runs the check); GitHub docs, workflow commands (`::error::`) |
| 7 | A different git failure | Any exit of `git ls-remote` other than 0 and 2 (for example 128: no network, or a bad URL) stops the step at once with exit 2 and the line `::error::private-data: cannot read the refs of the repository (git exit <code>). Run the job again.` The step never gives the conflict message for such a failure | `git help ls-remote` (exit 2 only for "no matching refs"); CONVENTION-01 (each failure gives its cause and its fix) |
| 8 | The inputs of the step | Only through `env:`: `PR_NUMBER: ${{ github.event.pull_request.number }}`, `REPO_URL: ${{ github.server_url }}/${{ github.repository }}.git` and `BASE_REF: ${{ github.event.pull_request.base.ref }}`. The `run:` line holds no `${{`. The step gets no secret and no token. The script stops with exit 2 when `PR_NUMBER` is not a whole number, or when `REPO_URL` or `BASE_REF` is empty | `.github/workflows/private-data.yml` lines 66 to 76 (each input of `check.sh` comes through `env:`); STORY-46 spec, decision 15 |
| 9 | What the step reads | No file of the pull request: the step runs before the checkout of `pr`, and `git ls-remote` reads only the ref list | CRITERION-145; the header of `private-data.yml` ("No step may run, source, or read as config a file of the folder "pr"") |
| 10 | What does not change | The first checkout (no `ref:`), the job id and the job name `private-data`, the environment expression, the gate step as the first step, the trigger, `permissions: contents: read`, the secret, the checkout of `pr`, and `check.sh` | ADR 0027-approve-each-fork-run-of-the-private-data-check-through-a.md ("The job id, the job name, the trigger, the secret, and the ruleset do not change"); `prd:/decisions/DECISION-57.md` (rules 1 to 5: the gate, the required check, the job id); the header of `private-data.yml`; STORY-46 spec, decision 15; CRITERION-145 |
| 11 | An old merge ref | `check.sh` step 0 stays the guard for a merge ref that exists but is old (its second parent is not `HEAD_SHA`: "the merge commit of the pull request is not ready; run the job again", exit 2). The new step covers only a merge ref that does not exist | STORY-46 spec, design step 0; `scripts/private-data/check.sh` |
| 12 | A fork pull request | The same step and the same check. The merge ref of a fork pull request is in the base repository. The gate step and the environment stay before it | ADR 0027; `prd:/decisions/DECISION-57.md`; row 2 |
| 13 | Tests in CI | New cases in `scripts/private-data/test.sh`: `mr-a` to `mr-e` run `merge-ref.sh` against a local bare repository, and `wf-g` and `wf-h` read the workflow as text. The cases `wf-a` to `wf-f` pass with no change | STORY-77 spec, rows 18 and 19 (a pull request cannot run its own change of a `pull_request_target` workflow, so text cases prove it in CI); `.github/workflows/private-data-tests.yml` |
| 14 | Tests on GitHub | Steps L1 and L2 of "Test plan", after the merge | STORY-77 spec, rows 19 and 21 |
| 15 | Documentation | `scripts/private-data/README.md` gets the section "A pull request that conflicts with its base branch": the message, the cause, the fix, and the wait | `scripts/private-data/README.md` (each failure of the check has its section, for example "Fork pull requests") |
| 16 | The reviews of the code PR | Reviewer, verifier, code-quality, and security. Security reviews it because the workflow holds the secret `PRIVATE_PATTERNS` and runs on `pull_request_target` | Orchestrator TASK of 2026-11-04 ("The workflow holds a secret, so the code PR goes to security too"); `docs/adlc.md`, section 3 (security: "A code PR is open") |
| 17 | CRITERION-148 | A new criterion of STORY-80, "A late merge ref runs as before", with the text of the reviewer note on PR #308 | Reviewer note on PR #308 ("When GitHub makes the merge ref during the wait of the new step, the check also runs as before"); orchestrator TASK of 2026-11-04 (the planner agrees) |
| 18 | ADR | None. A step of a CI workflow is cheap to undo. The decision of ADR 0027 does not change | `docs/adr/CLAUDE.md` ("If it is cheap to change your mind, do not write an ADR"); row 10 |
| 19 | When STORY-80 is done | After L1 and L2 pass. A code merge alone does not finish STORY-80, because a pull request cannot run its own change of this workflow. The planner sets STORY-80 `done` with `--actor human:404pilo`, and its commit and PR body name the planner session and D-013 | STORY-77 spec, row 21 (the same rule for STORY-77); `docs/adlc.md`, section 2, rule 4 |
| 20 | The state of STORY-80 | `ready`: this PR merges the spec, STORY-80 has no feature and no `blocked_by` list, and no card is open. `prd.py` refuses an agent for `backlog` to `ready`, so the session jag-specifier records `human:404pilo` | `docs/adlc.md`, section 2, rules 1, 2 and 4; D-013 |
| 21 | Spec group | One spec for STORY-80 | Orchestrator TASK of 2026-11-04 |

## Design

### The step

After the first checkout and before the checkout of `pr`:

```yaml
      # The merge ref of the pull request. GitHub makes none when the pull
      # request conflicts with its base branch. Reads no file of "pr".
      - name: Check the merge ref
        if: github.event_name == 'pull_request_target'
        env:
          PR_NUMBER: ${{ github.event.pull_request.number }}
          REPO_URL: ${{ github.server_url }}/${{ github.repository }}.git
          BASE_REF: ${{ github.event.pull_request.base.ref }}
        run: base/scripts/private-data/merge-ref.sh
```

### The script

`scripts/private-data/merge-ref.sh`:

1. `set -euo pipefail`. Never `set -x`. Read each input with `${VAR:-}`.
2. Check the inputs (row 8). A bad input: a line on standard error that
   names the variable, and exit 2.
3. Up to `MERGE_REF_TRIES` times: run
   `git ls-remote --exit-code "$REPO_URL" "refs/pull/$PR_NUMBER/merge"`,
   with its standard output sent to `/dev/null`. Keep the exit code with
   `rc=0; git ls-remote ... || rc=$?`, so that `set -e` does not stop the
   script.
   - Exit 0: print `private-data: the merge ref of PR #<n> exists`, and
     exit 0.
   - Exit 2: when a check is left, print
     `private-data: no merge ref for PR #<n> yet; next check in <s> s`, and
     wait `MERGE_REF_PAUSE` seconds.
   - Any other exit: the message of row 7, and exit 2.
4. After the last check: the message of row 6, and exit 1.

The script prints no ref list and no commit. Each line holds only fixed
text, the number of the pull request, the base branch, and a number of
seconds or an exit code.

### Test plan

CI cases in `scripts/private-data/test.sh`. They need no secret and no
network. They make a local bare repository and give its path as
`REPO_URL`:

| Case | It checks | Criterion |
|---|---|---|
| `mr-a` | The bare repository has `refs/pull/7/merge`: exit 0, and the output has `the merge ref of PR #7 exists` | CRITERION-145 |
| `mr-b` | No merge ref, `MERGE_REF_TRIES=2`, `MERGE_REF_PAUSE=0`: exit 1, and the output has `::error::private-data: PR #7 conflicts with main: merge main into the branch` | CRITERION-144 |
| `mr-c` | No merge ref at the start. A background job makes `refs/pull/7/merge` after 2 seconds. `MERGE_REF_PAUSE=1`, `MERGE_REF_TRIES=6`: exit 0, and the output has no `conflicts with` | CRITERION-148 |
| `mr-d` | `REPO_URL` is a folder that does not exist: exit 2, the output has `cannot read the refs of the repository`, and it has no `conflicts with` | Row 7 |
| `mr-e` | `PR_NUMBER=7x`, then an empty `BASE_REF`: exit 2 each time, and no `git ls-remote` runs (the bare repository has the ref, and the output has no `exists`) | Row 8 |
| `wf-g` | The workflow has 1 step `Check the merge ref`. It is after the first checkout and before the checkout with `path: pr`. Its condition is `if: github.event_name == 'pull_request_target'`. Its `run:` line is `run: base/scripts/private-data/merge-ref.sh` | Rows 2 and 3 |
| `wf-h` | The `env:` of that step has only `PR_NUMBER`, `REPO_URL` and `BASE_REF`. The step has no `secrets.`, no `github.token`, no `GITHUB_TOKEN`, and no `${{` on its `run:` line | Rows 8 and 9 |
| `wf-a` to `wf-f` | Pass with no change: the job id and the job name, the trigger, the environment, the gate step as the first step, no `ref:` on the first checkout, 1 job and no job-level `if:` | Row 10 |

CI runs no `shellcheck`. The implementer runs `shellcheck` on
`merge-ref.sh` and `test.sh` before the PR, as the `# shellcheck` notes in
`test.sh` show.

Steps on GitHub, after the merge. The orchestrator gives them to the
verifier with a `TASK`:

- **L1 (CRITERION-145):** the next same-repo pull request with no conflict.
  The `private-data` run passes, and the log of `Check the merge ref` has
  `the merge ref of PR #<n> exists`.
- **L2 (CRITERION-144):** a throwaway same-repo pull request that holds
  only invented text and conflicts with `main` on purpose. The run fails at
  `Check the merge ref`, and the checks page shows
  `private-data: PR #<n> conflicts with main: merge main into the branch`.
  Then close the pull request with no merge, and delete its branch.

CRITERION-148 has no GitHub step, because nobody can make GitHub late on
purpose. `mr-c` proves it.

## Build order

1. `scripts/private-data/merge-ref.sh` and the cases `mr-a` to `mr-e`.
2. The step in `.github/workflows/private-data.yml`, and the cases `wf-g`
   and `wf-h`. Each case of `test.sh` passes.
3. `scripts/private-data/README.md`: the new section (row 15).
4. After the merge: L1 and L2. Then the planner sets STORY-80 `done`
   (row 19).

One code pull request for steps 1 to 3.

## Assumptions

- Row 4: the repository is public, so `git ls-remote` of
  `https://github.com/ai-ready-future/jag.git` needs no token. A test on
  2026-11-04 gave exit 0 for `refs/pull/307/head` and exit 2 for
  `refs/pull/99999/merge`, with no token in the command.
- Row 4: when a pull request conflicts with its base branch, GitHub has no
  `refs/pull/<n>/merge`. The 3 failed runs of PR #307 show it
  (`couldn't find remote ref`).
- Row 6: a git branch name cannot hold `:` or a line break
  (`git check-ref-format`), so `<base>` cannot change the `::error::`
  command.
- Row 13: `test.sh` already makes local repositories (`new_repo`), so a
  bare repository with `git update-ref refs/pull/7/merge` needs no new
  tool.

## Risks

1. The merge ref comes later than the wait. Signal: GitHub shows the pull
   request as mergeable, and the check gives the conflict message. The
   GitHub docs say that merge refs can fall out of date until a person or
   an API call views the pull request. Mitigation: run the job again. If
   it happens more than one time, make the wait longer (row 5 is 2 values
   in 1 script).
2. A network failure looks like a conflict. Signal: the conflict message
   on a pull request with no conflict. Mitigation: row 7 and `mr-d`.
3. The repository becomes private. Signal: each pull request fails with
   `cannot read the refs of the repository`. Mitigation: none in this
   spec. A private repository needs a token in the step, and that is a
   change of STORY-46 spec decision 15.
4. A fault in the new step blocks each pull request after the merge,
   also the pull request that reverts it, because `pull_request_target`
   runs the file of `main`. Signal: L1 fails. Mitigation: `mr-a` to `mr-e`,
   `wf-g` and `wf-h` in CI before the merge, and L1 at once after the
   merge.
5. The new step changes the scan. Signal: a case of `test.sh` other than
   the new ones fails, or `wf-a` to `wf-f` fail. Mitigation: row 10.

## Out of scope

- A poll of the REST field `mergeable` (`GET /repos/{owner}/{repo}/pulls/{n}`),
  which the GitHub docs recommend. It needs `pull-requests: read` and a
  token in the workflow, and STORY-46 spec decision 15 (a decision of the
  user) gives the workflow only `contents: read`.
- A fix for an old merge ref: `check.sh` step 0 already stops it (row 11).
- A new run of the job by itself after a conflict.
- `scripts/private-data/CLAUDE.md`: the router line for `merge-ref.sh` is a
  file of the designer. The code PR sends a `TASK` suggestion for it.

## For the architect

- The step adds no permission, no secret, no token and no trigger. ADR 0027
  does not change.
- So this spec writes no ADR.
