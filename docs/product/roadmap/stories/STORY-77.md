---
type: Story
title: Approve the private-data check of a fork pull request before it runs
description: As an agent builder (PERSONA-01), I want a maintainer to approve each
  private-data run of a fork pull request, so that the public result cannot confirm
  guesses.
status: draft
state: ready
github:
  number: 246
  node_id: I_kwDOVKMnL88AAAABaEsC2Q
  url: https://github.com/ai-ready-future/jag/issues/246
  pushed_hash: e25fe8
  synced:
    state: backlog
spec: docs/superpowers/specs/2026-10-26-STORY-77-approve-fork-private-data-runs-design.md
parent: /features/FEATURE-28.md
milestone: /milestones/MILESTONE-02.md
---

# STORY-77: Approve the private-data check of a fork pull request before it runs

As an agent builder (PERSONA-01), I want the private-data check of a fork pull request to run only after a maintainer approves the run, so that the public result of the check cannot confirm guesses about private data.

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [CRITERION-136: A fork pull request shows no private-data result before a maintainer approves the run](../criteria/CRITERION-136.md) | - | - | - | - | - |
| [CRITERION-137: After the approval, the scan runs and the required check stops the merge until it passes](../criteria/CRITERION-137.md) | - | - | - | - | - |
| [CRITERION-138: Each new push or edit of a fork pull request needs a new approval](../criteria/CRITERION-138.md) | - | - | - | - | - |
| [CRITERION-139: Pull requests of this repository and pushes to main keep the automatic scan](../criteria/CRITERION-139.md) | - | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [FEATURE-28: A maintainer approves the private-data check of a fork pull request](../features/FEATURE-28.md)
- milestone: [MILESTONE-02: Make it good](../milestones/MILESTONE-02.md)
- criterion or run of: [CRITERION-136: A fork pull request shows no private-data result before a maintainer approves the run](../criteria/CRITERION-136.md)
- criterion or run of: [CRITERION-137: After the approval, the scan runs and the required check stops the merge until it passes](../criteria/CRITERION-137.md)
- criterion or run of: [CRITERION-138: Each new push or edit of a fork pull request needs a new approval](../criteria/CRITERION-138.md)
- criterion or run of: [CRITERION-139: Pull requests of this repository and pushes to main keep the automatic scan](../criteria/CRITERION-139.md)
<!-- LINKS:END -->
