---
type: Feature
title: A maintainer approves the private-data check of a fork pull request
description: The private-data check of a fork pull request runs only after a maintainer
  approves each run, and the required check stops the merge until an approved run
  passes.
status: draft
state: ready
github:
  number: 245
  node_id: I_kwDOVKMnL88AAAABaEr8kg
  url: https://github.com/ai-ready-future/jag/issues/245
  pushed_hash: 9f1969
  synced:
    state: ready
parent: /epics/EPIC-05.md
milestone: /milestones/MILESTONE-02.md
traces:
- to: prd:/constraints/CONSTRAINT-01.md
  hash: 1970e3
- to: prd:/decisions/DECISION-57.md
  hash: 6fbf8b
---

# FEATURE-28: A maintainer approves the private-data check of a fork pull request

The private-data check of a pull request from a fork runs only after a maintainer approves the run. Before the approval, the pull request shows no result of the scan. Each new push or edit needs a new approval. The required check `private-data` on `main` stays, so a fork pull request cannot merge until an approved run passes. A pull request from a branch of this repository and a push to `main` keep the automatic scan. The job id stays `private-data`.

Decision (DECISION-57): this revisits decision 12 of the STORY-47 spec ("No guard"). A protected GitHub environment with a required reviewer is the expected method. The specifier decides the details.

The criteria of STORY-77 replace the fork test of issue #99. That test expects a run with no approval wait, and DECISION-57 changes that.

Source: the user, 2026-10-26: "the forks pull requests cannot run or see the results of this specific check .. this is only required for merges to main and it is something that an approve needs to do before merging to main".

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [STORY-77: Approve the private-data check of a fork pull request before it runs](../stories/STORY-77.md) | ready | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [EPIC-05: Safe public release](../epics/EPIC-05.md)
- milestone: [MILESTONE-02: Make it good](../milestones/MILESTONE-02.md)
- traces: [CONSTRAINT-01: The public release holds no private data](../../prd/constraints/CONSTRAINT-01.md)
- traces: [DECISION-57: A maintainer approves the private-data run of a fork pull request before the merge](../../prd/decisions/DECISION-57.md)
- child: [STORY-77: Approve the private-data check of a fork pull request before it runs](../stories/STORY-77.md)
<!-- LINKS:END -->
