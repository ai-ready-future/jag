---
title: "A maintainer approves the private-data check of a fork pull request: one approval for each fork run"
feature: FEATURE-28
stories: [STORY-77]
kind: build
gh_issue: 246
lifecycle_state: draft
session: 4acb9f7a-4603-41fb-8e74-395e5117e288
---

# A maintainer approves the private-data check of a fork pull request: one approval for each fork run

## Problem

The check `private-data` compares the files, the commits, the title, and
the body of a pull request with a private list of patterns. For a pull
request from a fork, the check runs on each event, also on an edit of the
title or the body with no new push (decision 12 of the STORY-47 spec). The
result of the check is public. A public pass or fail of a check against
private patterns can confirm a guess about private data (security finding
F1, low). This spec describes the problem only as a class of problem.

DECISION-57 (the user, 2026-10-26) changes decision 12 of the STORY-47
spec: "the forks pull requests cannot run or see the results of this
specific check .. this is only required for merges to main and it is
something that an approve needs to do before merging to main."

STORY-77: "As an agent builder (PERSONA-01), I want the private-data check
of a fork pull request to run only after a maintainer approves the run, so
that the public result of the check cannot confirm guesses about private
data."

The criteria:

- CRITERION-136: a fork pull request shows no private-data result before a
  maintainer approves the run.
- CRITERION-137: after the approval, the scan runs, and the required check
  stops the merge until it passes.
- CRITERION-138: each new push or edit of a fork pull request needs a new
  approval.
- CRITERION-139: pull requests of this repository and pushes to `main` keep
  the automatic scan. The job id stays `private-data`, so the ruleset does
  not change.

The criteria 136 to 138 replace the fork test of issue #99 (FEATURE-28).

## Decisions

Row 1 comes from the user (DECISION-57). The rows with the source "this
spec" follow from row 1 and from the GitHub docs that the row names.

| # | Decision | Choice | Source |
|---|---|---|---|
| 1 | The rule | A fork run of the job `private-data` does not start until a maintainer approves that run. Each new push and each edit needs a new approval. The required check stays. Same-repo pull requests and pushes to `main` keep the automatic scan. The job id stays `private-data`. The event stays `pull_request_target` | DECISION-57 (rules 1 to 5, and the STORY-60 note); CRITERION-136 to CRITERION-139 |
| 2 | The approval method | A protected GitHub environment with a required reviewer, on the job `private-data`, for fork runs only | ADR 0027-approve-each-fork-run-of-the-private-data-check-through-a.md (proposed); DECISION-57 rule 1 ("The expected method is a protected GitHub environment with a required reviewer") |
| 3 | The job | One job. The id and the name stay `private-data`. No second job, and no job-level `if:` | DECISION-57 rule 5; GitHub docs, "Troubleshooting required status checks": "A job is skipped by a conditional: The job reports "Success"". A second job with the same check name, or a skip, can satisfy the required check with no scan |
| 4 | The environment of a run | `environment.name` is an expression: `private-data-fork` for a fork run, `private-data-auto` for each other run. The fork test is the same text as `FORK_PR` (line 58 of `private-data.yml`), with the guard `github.event_name == 'pull_request_target'`. Without the guard, a push to `main` counts as a fork run, because `github.event.pull_request` is empty on `push` | Inferred: `.github/workflows/private-data.yml` line 58; GitHub docs, workflow syntax `jobs.<job_id>.environment`: "The value of `name` can be an expression. Allowed expression contexts: `github`, `inputs`, `vars`, `needs`, `strategy`, and `matrix`" |
| 5 | Two named environments, not an empty name | Both environments exist. Only `private-data-fork` has a rule. An expression that gives an empty name for a same-repo run is not used: the docs do not say what an empty environment name does, and `pull_request_target` runs the file of `main`, so a pull request cannot test it before the merge | Inferred: GitHub docs (context7 `/github/docs`, `deployments-and-environments.md` and the workflow syntax page give no empty-name rule); "Deploy to environment": "Requires the environment to be created before running the workflow" |
| 6 | The settings of `private-data-fork` | Required reviewer `404pilo`. "Prevent self-review" off. "Allow administrators to bypass configured protection rules" off. No wait timer. Deployment branches: `main` only. One environment variable, `PRIVATE_DATA_FORK_GATE` = `on`. No environment secret | DECISION-57 rule 1; GitHub docs, "Managing environments for deployment" (required reviewers, prevent self-review, admin bypass, deployment branches). Self-review stays off, because `404pilo` is the only reviewer and the owner of the test fork `404pilo/jag-fork-test` |
| 7 | The settings of `private-data-auto` | No protection rule. No branch rule, so a same-repo pull request into any branch runs as before. No variable, no secret | DECISION-57 rule 4; CRITERION-139 |
| 8 | A missing or wrong environment fails closed | The first step of the job is a gate step. On a fork run, it stops the job with exit 1 when `vars.PRIVATE_DATA_FORK_GATE` is not `on`. Only `private-data-fork` sets the variable. So a fork run in a wrong environment fails, with no checkout and no scan | This spec. Reason: when the environment is missing or has a different name, a fork run can start with no reviewer (risk 1). GitHub docs, `deployments-and-environments.md`: environment secrets and variables reach a job only after its protection rules pass |
| 9 | The secret | `PRIVATE_PATTERNS` stays a repository secret, with no change. Both environments need it | FEATURE-17 spec, decision 6; DECISION-57 rule 4 |
| 10 | The trigger | No change: `pull_request_target` with `[opened, synchronize, reopened, edited]`, and `push` to `main` | DECISION-57 (STORY-60, CRITERION-99, stays true); FEATURE-17 spec, decision 15 |
| 11 | A new approval for each run | Each event makes a new run, and each run waits for its own approval. No setting carries an approval to a later run | DECISION-57 rule 2; GitHub docs, "Reviewing deployments": a reviewer approves or rejects the waiting job of one run |
| 12 | The required check | The ruleset of `main` does not change. A waiting run has no result, so the required check blocks the merge. A rejected run fails the check | DECISION-57 rule 3; GitHub docs, "Reviewing deployments" (a rejected job fails the workflow) |
| 13 | The fork output | No change. A fork run prints only `private-data: private data found`, with no place (`FORK_PR`) | FEATURE-17 spec, decision 16 |
| 14 | Concurrency | No change. Same-repo runs stay as before. A fork pull request can have more than one waiting run. The maintainer rejects the older ones and approves the newest | This spec, from DECISION-57 rule 4 ("keep the automatic scan"). A concurrency group would also change same-repo runs |
| 15 | Who approves | Only the user, as `404pilo`, by hand on GitHub. No session approves or rejects a waiting run, also not through the API (`pending_deployments`). The sessions use the token of the same account, so GitHub cannot tell a session from the user | DECISION-57 ("an approve needs to do"); GitHub docs, REST "Review pending deployments for a workflow run" |
| 16 | What the maintainer examines | The list in "Before you approve" | Orchestrator TASK of 2026-10-26 (item 2); DECISION-57 |
| 17 | The result after an approval | Public. The spec says this plainly in "The result is public after the approval" | Orchestrator TASK of 2026-10-26 (item 2); DECISION-57 |
| 18 | Tests in CI | New text cases `wf-a` to `wf-f` in `scripts/private-data/test.sh`. They read `.github/workflows/private-data.yml` | Inferred: `scripts/private-data/test.sh` (cases `hist-*` and `clean-*` with the `assert_*` helpers); `.github/workflows/private-data-tests.yml` runs it on each pull request |
| 19 | Tests on GitHub | Steps L1 to L5 of "Test plan": the first push run after the merge, the next same-repo pull request, and one test fork pull request on or after 2026-11-02 | This spec. `pull_request_target` runs the workflow file of `main`, so the code pull request cannot run its own change. Issue #99: "GitHub changes the default of the Actions event policy on 2026-11-02" |
| 20 | The order of the work | The user makes the two environments first. chores checks them through the API. Then the code pull request merges | This spec, from row 8 and risk 1 |
| 21 | When STORY-77 is done | After L1 to L5 pass. A code merge alone does not finish STORY-77 | This spec; the STORY-47 spec, decision 25 (the same rule for STORY-60) |
| 22 | The state of STORY-77 | `ready`, in this spec pull request. No ASK is open for FEATURE-28 | docs/adlc.md, rule 11 |
| 23 | Decision 12 of the STORY-47 spec | Marked as revisited by DECISION-57, in this pull request. The notes of that spec that call an approval wait a bug get a short note too | DECISION-57; Orchestrator TASK of 2026-10-26 (item 4) |
| 24 | Issue #99 | It stays closed, with the reason "not planned" and one comment that names its replacement. See "Issue #99" | Planner recommendation in the body of PR #244; Orchestrator TASK of 2026-10-26 (item 6) |
| 25 | The ADR | One ADR, `status: proposed`, for the trust gate of fork runs. No ADR covered the private-data CI gate | Project `CLAUDE.md`; `docs/adr/CLAUDE.md`; Orchestrator TASK of 2026-10-26 (item 1) |

## Design

### The workflow (CRITERION-136, CRITERION-137, CRITERION-139)

`.github/workflows/private-data.yml` gets two changes. The rest of the
file stays the same.

1. The job `private-data` gets an `environment`.
2. The job gets a new first step, the gate step.

The job, after the change. The comment lines are part of the change.

```yaml
jobs:
  private-data:
    name: private-data
    runs-on: ubuntu-latest
    # A fork pull request waits for a maintainer approval (FEATURE-28).
    # Only the environment private-data-fork has a required reviewer.
    environment:
      name: ${{ (github.event_name == 'pull_request_target' && github.event.pull_request.head.repo.id != github.repository_id) && 'private-data-fork' || 'private-data-auto' }}
    steps:
      # A fork run stops here when its environment is not private-data-fork.
      - name: Check the approval gate
        if: github.event_name == 'pull_request_target' && github.event.pull_request.head.repo.id != github.repository_id
        env:
          GATE: ${{ vars.PRIVATE_DATA_FORK_GATE }}
        run: |
          if [[ "$GATE" != on ]]; then
            echo "private-data: the approval gate of fork pull requests is not set" >&2
            exit 1
          fi

      # The code of main (on push: the pushed commit). check.sh comes from here only.
      - uses: actions/checkout@...   # no change from here on
```

Add one sentence to the header comment of the file: "Do not remove the
environment or the gate step: a fork run must wait for a maintainer
approval (FEATURE-28)."

How each run goes:

| Run | Environment | Before the steps | Gate step |
|---|---|---|---|
| Fork pull request | `private-data-fork` | Waits for the approval of `404pilo`. The check shows "Waiting". The run has no result, so the ruleset blocks the merge | Passes: the variable is `on` |
| Fork pull request, environment missing or wrong | Another environment, with no rule | Starts at once | Fails with exit 1. No checkout and no scan run |
| Same-repo pull request | `private-data-auto` | Starts at once | Skipped |
| Push to `main` | `private-data-auto` | Starts at once | Skipped |

A run of a deleted fork has an empty `head.repo`. The fork test is then
true, so the run waits for an approval. This fails closed.

A fork pull request into a branch other than `main` fails before the
steps: the branch rule of `private-data-fork` allows only `main`. This is
acceptable, because only a merge to `main` needs the check (DECISION-57).

Each run with an environment makes a GitHub deployment. The pull request
page shows a line for each one. This is not a failure.

### The GitHub settings (for chores and the user)

The user makes these settings on GitHub before the code pull request
merges. chores guides the user and then checks the result. A session makes
no setting without the go of the user.

1. Make the environment `private-data-fork` (Settings, Environments, New
   environment):
   - Required reviewers: on, with `404pilo`.
   - Prevent self-review: off.
   - Wait timer: off.
   - Allow administrators to bypass configured protection rules: off.
   - Deployment branches and tags: "Selected branches and tags", with the
     branch rule `main`.
   - Environment secrets: none.
   - Environment variables: `PRIVATE_DATA_FORK_GATE` with the value `on`.
2. Make the environment `private-data-auto`: no protection rule, no branch
   rule, no secret, no variable.
3. Change nothing else: not the ruleset of `main`, not the repository
   secret `PRIVATE_PATTERNS`, not the Actions event policy 7541, and not
   the fork approval setting (`all_external_contributors`).

When the user tells chores to make the settings through the API, use these
commands. `<id>` is the output of `gh api users/404pilo --jq .id`.

```
$ gh api -X PUT repos/ai-ready-future/jag/environments/private-data-fork --input fork-env.json
$ gh api -X POST repos/ai-ready-future/jag/environments/private-data-fork/deployment-branch-policies -f name=main -f type=branch
$ gh api -X POST repos/ai-ready-future/jag/environments/private-data-fork/variables -f name=PRIVATE_DATA_FORK_GATE -f value=on
$ gh api -X PUT repos/ai-ready-future/jag/environments/private-data-auto
```

`fork-env.json`:

```json
{
  "wait_timer": 0,
  "prevent_self_review": false,
  "reviewers": [{"type": "User", "id": <id>}],
  "can_admins_bypass": false,
  "deployment_branch_policy": {"protected_branches": false, "custom_branch_policies": true}
}
```

chores checks the result with these read-only commands:

```
$ gh api repos/ai-ready-future/jag/environments/private-data-fork --jq '{bypass: .can_admins_bypass, rules: [.protection_rules[] | {type, prevent_self_review, reviewers: [.reviewers[]?.reviewer.login]}], branch: .deployment_branch_policy}'
$ gh api repos/ai-ready-future/jag/environments/private-data-fork/deployment-branch-policies --jq '[.branch_policies[] | {name, type}]'
$ gh api repos/ai-ready-future/jag/environments/private-data-fork/variables/PRIVATE_DATA_FORK_GATE --jq .value
$ gh api repos/ai-ready-future/jag/environments/private-data-auto --jq '{rules: [.protection_rules[].type], branch: .deployment_branch_policy}'
$ gh api repos/ai-ready-future/jag/actions/variables --jq '[.variables[].name]'
$ gh api repos/ai-ready-future/jag/actions/organization-variables --jq '[.variables[].name]'
```

The expected result:

- `private-data-fork`: `bypass` is `false`. One rule has the type
  `required_reviewers`, `prevent_self_review` `false`, and the reviewers
  `["404pilo"]`. No rule has the type `wait_timer`. `branch` has
  `custom_branch_policies` `true`.
- The branch rules are `[{"name":"main","type":"branch"}]`.
- The variable is `on`.
- `private-data-auto`: no rule of the type `required_reviewers`, and
  `branch` is `null`.
- No repository variable and no organization variable has the name
  `PRIVATE_DATA_FORK_GATE`. A variable of that name at a different level
  makes the gate step pass in a wrong environment.

### Who approves

Only the user approves or rejects a waiting run, as `404pilo`, by hand on
GitHub: the run page, "Review deployments", then "Approve and deploy" or
"Reject". No session approves or rejects a waiting run, also not through
the API. A session can read the waiting state, for example with
`gh api repos/ai-ready-future/jag/actions/runs/<run>/pending_deployments`.

The code pull request adds this rule to `scripts/private-data/README.md`
(section "Fork pull requests"), with the list of "Before you approve".

### Before you approve

Before you approve a waiting run of a fork pull request, examine these
items on the pull request page:

1. The diff of all files. The change must have a clear purpose for jag.
2. The title, the body, and the commit messages. Look for text with no
   purpose in the change, for example names, words, addresses, or strings
   that look like keys or guesses.
3. The history of the pull request. Many small pushes or edits that each
   change only a word or a short string are a probe, mainly after an
   earlier approved run failed.
4. The run. Approve only the newest waiting run. Reject the older waiting
   runs of the same pull request first.

When an item is not clear, reject the run and ask the author in a comment.
A rejected run fails the check and tells nothing about private data. Close
a pull request that probes, with no merge.

### The result is public after the approval

After the user approves a run, the scan runs, and each person can see the
result: a pass, or a fail with the line `private-data: private data found`.
The fork output names no file and no line (FEATURE-17 spec, decision 16).
But the pass or the fail itself is public. So each approved run gives one
public yes-or-no answer about the text of the pull request. The approval
does not hide that answer. It lets the maintainer decide which text gets an
answer, and how many answers a person gets.

### Test plan

CI text cases in `scripts/private-data/test.sh`. They read
`.github/workflows/private-data.yml` as text, with no secret:

| Case | It checks |
|---|---|
| `wf-a` | The job id line `  private-data:` and the line `    name: private-data` occur one time each (CRITERION-139) |
| `wf-b` | The trigger lines: `types: [opened, synchronize, reopened, edited]` under `pull_request_target`, and `branches: [main]` under `push` (rows 10 and 11) |
| `wf-c` | The `environment` `name:` line is the exact text of the Design, with `'private-data-fork'`, `'private-data-auto'`, and the guard `github.event_name == 'pull_request_target'` (rows 4 and 5) |
| `wf-d` | The first step of the job is the gate step. Its `if:` holds the same fork test as the `environment` line and as `FORK_PR`. It reads `vars.PRIVATE_DATA_FORK_GATE` and exits 1 when the value is not `on` (row 8) |
| `wf-e` | The first checkout has no `ref:` (FEATURE-17 spec, decision 15) |
| `wf-f` | The job has no `if:` and the file has no second job (row 3) |

Tests on GitHub. Record each result (run URL, event, environment, result)
in a comment on issue #246:

| Step | When | What | Criterion |
|---|---|---|---|
| L1 | The push to `main` of the code merge | The run starts with no wait, in `private-data-auto`, and passes | CRITERION-139 |
| L2 | The next same-repo pull request | The run starts with no wait, in `private-data-auto`, and passes | CRITERION-139 |
| L3 | On or after 2026-11-02 | chores opens one test fork pull request from `404pilo/jag-fork-test`, with a small change of a doc. The run waits in `private-data-fork`: `pending_deployments` lists it, the check shows "Waiting" with no result, and `gh pr view <n> --json mergeStateStatus` gives `BLOCKED` | CRITERION-136, CRITERION-137 |
| L4 | After L3 | The user approves the run. The scan runs and passes with `private-data: no findings`. The ruleset of `main` still requires `private-data` (`gh api repos/ai-ready-future/jag/rulesets/<id>`) | CRITERION-137 |
| L5 | After L4 | chores pushes one more commit to the test fork pull request, and then edits its title. Each event makes a new run that waits. The earlier approval does not apply. The user rejects these runs, and the check fails. chores closes the pull request with no merge | CRITERION-138 |

When a run in L3 to L5 does not start, starts with no wait, or does not
pass after the approval, chores opens a bug issue and tells the
orchestrator. The test also shows that the Actions event policy of
2026-11-02 lets `pull_request_target` run (the purpose of issue #99).

## Build order

1. The user makes the settings of "The GitHub settings". chores guides
   the user, then runs the read-only commands. Until the result is as
   expected, the code pull request does not merge (a user step).
2. The builder opens one code pull request:
   - the workflow change of the Design;
   - the cases `wf-a` to `wf-f` in `scripts/private-data/test.sh`;
   - the section "Fork pull requests" in `scripts/private-data/README.md`
     (who approves, and "Before you approve").
   The check `private-data` of this pull request runs the file of `main`,
   so it does not test the change. The cases `wf-a` to `wf-f` do.
3. The orchestrator merges with squash after step 1 is complete.
4. chores records L1, then L2.
5. On or after 2026-11-02, chores runs L3 to L5 with the user, and records
   them. Then STORY-77 is done.
6. Keep `404pilo/jag-fork-test` until L5 passes.

## Issue #99

Issue #99 is closed with the reason "completed", since 2026-10-26. The
merge of PR #244 closed it, because the body of #244 held a closing
keyword for #99. Its test did not run.

What #99 must become: it stays closed, with the reason "not planned", and
one comment that names its replacement. chores does this after this spec
merges:

```
$ gh issue reopen 99 --repo ai-ready-future/jag
$ gh issue close 99 --repo ai-ready-future/jag --reason "not planned" --comment "Replaced by STORY-77 (#246): CRITERION-136 to CRITERION-138 and steps L3 to L5 of docs/superpowers/specs/2026-10-26-STORY-77-approve-fork-private-data-runs-design.md. From DECISION-57, a fork run waits for a maintainer approval, so the expectation 'no approval wait' of this issue is not true any more."
```

## Assumptions

- Row 4: the fork test is the text of `FORK_PR`
  (`.github/workflows/private-data.yml` line 58), with the event guard. A
  run of a deleted fork has an empty `head.repo`, and the test is true.
- Row 5: GitHub does not document an empty environment name. Two named
  environments avoid it (GitHub docs, workflow syntax and
  `deployments-and-environments.md`).
- Row 8: environment variables reach the `vars` context of a job only in
  the environment that holds them, and only after its protection rules pass
  (GitHub docs, `deployments-and-environments.md`). An environment variable
  takes the place of a repository variable of the same name.
- Row 11: each event of `pull_request_target` makes a new run, and each run
  waits for its own approval (GitHub docs, "Reviewing deployments").
- Row 12: a waiting run has no result, so a required check stays pending
  and blocks the merge. A rejected job fails the workflow (GitHub docs,
  "Reviewing deployments").
- The branch rule `main` matches a `pull_request_target` run of a pull
  request into `main`, because `GITHUB_REF` of that event is the base
  branch (GitHub docs, "Events that trigger workflows").
- The job token needs no new permission for an environment. The deployment
  is made by GitHub, not by the job.
- Row 18: the text cases follow the form of `scripts/private-data/test.sh`
  (one id for each case, the `assert_*` helpers).

## Risks

1. The environment is missing, or has a different name, when the code
   merges. Signal: a fork run starts with no wait, or
   `gh api repos/ai-ready-future/jag/environments/private-data-fork` gives
   404. Mitigation: build order step 1; the gate step fails the run with
   no scan; case `wf-c` checks the name.
2. A person removes the reviewer, or sets admin bypass on, after the
   check of step 1. The gate variable does not see this. Signal: a fork run
   starts with no wait. Mitigation: chores runs the read-only commands
   again before L3, and before each later fork test.
3. A session approves a run through the API. Signal: an approval with no
   ANSWER of the user. Mitigation: the rule of "Who approves", in
   `scripts/private-data/README.md`.
4. A same-repo run waits for an approval (a wrong expression). Signal: a
   same-repo run in the state "Waiting". Mitigation: cases `wf-c` and
   `wf-d`; L1 and L2.
5. Two runs of one fork pull request report on the same commit, for
   example after an edit. Signal: the check shows a fail after the approved
   newest run passed. Mitigation: reject the older runs first, then approve
   the newest. A re-run waits for a new approval.
6. Many waiting runs on one fork pull request. Signal: a long list of
   waiting runs. Mitigation: the user rejects them. A waiting job that
   nobody reviews fails after 30 days (GitHub docs,
   `deployments-and-environments.md`).
7. The event policy of 2026-11-02 stops `pull_request_target` runs. Then a
   fork pull request gets no run, and the required check blocks the merge.
   Signal: no run on the test fork pull request of L3. Mitigation: chores
   opens a bug issue (L3 to L5).
8. The job needs a token permission for the environment. Signal: the run
   of L1 fails before the first step. Mitigation: the builder adds the
   permission that the error names, in a new pull request.

## Out of scope

- A change of `scripts/private-data/check.sh`, of the fork output, or of
  the secret (FEATURE-17).
- A change of the ruleset of `main`, of the Actions event policy, or of
  the fork approval setting (STORY-47 spec, decisions 22 and 23).
- A gate for `.github/workflows/private-data-tests.yml` and the other
  `pull_request` workflows. They hold no secret, and the fork approval
  setting holds their fork runs (STORY-47 spec, decision 23).
- A concurrency group (row 14).
- The update of `docs/product/system-design.md` (deployment-3). The ADR
  register lists the new ADR.

## For the orchestrator

- The rule of "Who approves" binds each session of the team. This spec
  puts it in `scripts/private-data/README.md`. A line in `docs/adlc.md`
  is the choice of the orchestrator.
- The ADR gate job fails while the ADR is `proposed`. It is not a required
  check.
