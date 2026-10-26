---
type: adr
title: Approve each fork run of the private-data check through a protected environment
description: Open this when you change how the private-data check runs for a fork pull request, its GitHub environments, or who approves a fork run.
area: deployment
status: proposed
date: 2026-10-26
supersedes: null
superseded_by: null
amends: null
refines: null
deviates_from: null
related: []
parent_spec: docs/superpowers/specs/2026-10-26-STORY-77-approve-fork-private-data-runs-design.md
model:
  status: out-of-scope
  reason: a CI gate of the repository is not an element of the system design
---

# ADR-0027: Approve each fork run of the private-data check through a protected environment

## Context

The check `private-data` compares a pull request with a private list of
patterns. `.github/workflows/private-data.yml` runs it on
`pull_request_target` (opened, synchronize, reopened, edited) and on a push
to `main`. It has one job, `private-data`, and the repository secret
`PRIVATE_PATTERNS`. The ruleset of `main` requires the check
`private-data`. The fork approval setting of the repository does not hold
`pull_request_target` runs.

The result of the check is public. A public pass or fail of a check
against private patterns can confirm a guess about private data (security
finding F1, low).

DECISION-57 (the user, 2026-10-26): "the forks pull requests cannot run or
see the results of this specific check .. this is only required for merges
to main and it is something that an approve needs to do before merging to
main." It revisits decision 12 of the STORY-47 spec, "No guard".

## Decision

- The job `private-data` names a GitHub environment with an expression.
  A fork run uses `private-data-fork`. Each other run uses
  `private-data-auto`. The fork test is
  `github.event_name == 'pull_request_target' &&
  github.event.pull_request.head.repo.id != github.repository_id`.
- `private-data-fork` has a required reviewer, `404pilo`, with
  "Prevent self-review" off, admin bypass off, no wait timer, and the
  deployment branch `main` only. It holds the variable
  `PRIVATE_DATA_FORK_GATE` = `on`. `private-data-auto` has no rule.
- So each fork run waits for one approval. A new push or an edit makes a
  new run that waits again. Same-repo pull requests and pushes to `main`
  run at once.
- The first step of the job is a gate step. On a fork run, it fails the job
  when `vars.PRIVATE_DATA_FORK_GATE` is not `on`. A fork run in a missing
  or wrong environment then fails with no scan.
- The job id, the job name, the trigger, the secret, and the ruleset do not
  change.
- Only the user approves or rejects a waiting run, by hand on GitHub. No
  session approves or rejects one, also not through the API.

## Why

- A required reviewer on an environment stops the job before its first
  step, and GitHub keeps the approval to one run. This gives each rule of
  DECISION-57 with no new code.
- One job keeps the required check true. A job that a condition skips
  reports "Success" and does not block a merge (GitHub docs, "Troubleshooting
  required status checks").
- Two named environments use only documented behavior. A run of
  `pull_request_target` uses the workflow file of `main`, so a pull request
  cannot test a change of this file before the merge.
- The gate variable makes a wrong environment fail closed.
- The sessions use the token of `404pilo`. GitHub cannot tell an approval
  of a session from an approval of the user.

## Alternatives considered

- **An empty environment name for same-repo runs.** Rejected: GitHub does
  not document what an empty name does, and no pull request can test it
  before the merge.
- **A second job for fork runs, with a job-level `if:`.** Rejected: the
  skipped job reports "Success", so a check with the same name can pass
  with no scan.
- **No guard (decision 12 of the STORY-47 spec).** Rejected by DECISION-57.

## Consequences

- The user makes the two environments and the variable before the code
  merges. A later change of their settings changes the gate with no code
  change. chores checks them through the API.
- After an approval, the pass or the fail of the run is public. The
  maintainer examines the diff, the title, the body, and the history of the
  pull request before an approval.
- A fork pull request into a branch other than `main` fails the check.
- Each run makes a GitHub deployment, and the pull request page shows it.
- A fork pull request can have more than one waiting run. The maintainer
  rejects the older ones and approves the newest.
