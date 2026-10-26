---
type: Decision
title: A maintainer approves the private-data run of a fork pull request before the
  merge
description: The private-data check of a fork pull request runs only after a maintainer
  approves each run. A fork pull request merges to main only after an approved run
  passes.
status: stable
decided_at: '2026-10-26'
alternatives:
- 'No guard: the check of a fork pull request runs on each event (STORY-47 spec decision
  12, 2026-10-14)'
about:
- /constraints/CONSTRAINT-01.md
---

# DECISION-57: A maintainer approves the private-data run of a fork pull request before the merge

Principle: the forks pull requests cannot run or see the results of this specific check .. this is only required for merges to main and it is something that an approve needs to do before merging to main.

This decision revisits decision 12 of the STORY-47 spec (FEATURE-19), "Guard against guesses". On 2026-10-14 that decision was "No guard", and STORY-59 (CRITERION-98), which held the guard, was cancelled. Today the check of a fork pull request runs on each event, also on an edit with no new push. The public pass or fail of the check can confirm guesses about private data (security finding F1, low).

The rules that follow from the principle:

1. For a pull request from a fork, the private-data job does not run until a maintainer approves that run. Before the approval, the pull request shows no result of the scan. The expected method is a protected GitHub environment with a required reviewer. The specifier decides the details.
2. Each new push to the pull request and each edit of it needs a new approval before the scan runs again.
3. The required check `private-data` on `main` stays. A fork pull request cannot merge until an approved run passes.
4. A pull request from a branch of this repository and a push to `main` keep the automatic scan, with no approval.
5. The job id stays `private-data`, because the ruleset of `main` names it.

STORY-60 (CRITERION-99) stays true: the check still runs on `pull_request_target`. The approval is a gate before the job, not a change of the event.

Source: the user, 2026-10-26: "the forks pull requests cannot run or see the results of this specific check .. this is only required for merges to main and it is something that an approve needs to do before merging to main". This answer changes decision 12 of the STORY-47 spec. FEATURE-28 builds it.

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CONSTRAINT-01: The public release holds no private data](../constraints/CONSTRAINT-01.md)
- traced by: [FEATURE-28: A maintainer approves the private-data check of a fork pull request](../../roadmap/features/FEATURE-28.md)
<!-- LINKS:END -->
