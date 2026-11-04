---
type: Story
title: Name the merge conflict when private-data cannot check out a PR
description: As a maintainer, I want the private-data check to say that a pull request
  conflicts with main when it cannot check it out, so that I fix the conflict and
  not the check.
status: draft
generated:
  by: prd/6.2.0
  at: '2026-11-04T13:46:59Z'
state: ready
github:
  number: 311
  node_id: I_kwDOVKMnL88AAAABcc6mMg
  url: https://github.com/ai-ready-future/jag/issues/311
  pushed_hash: 088ec7
  synced:
    state: ready
    owner: human:404pilo
spec: docs/superpowers/specs/2026-11-04-STORY-80-name-the-merge-conflict-when-private-data-cannot-check-out-a-pr-design.md
parent: /epics/EPIC-05.md
milestone: /milestones/MILESTONE-02.md
owner: human:404pilo
traces:
- to: prd:/constraints/CONSTRAINT-01.md
  hash: 1970e3
---

# STORY-80: Name the merge conflict when private-data cannot check out a PR

As a maintainer, I want the private-data check to say "PR #<n> conflicts with main: merge main into the branch" when it cannot check out the pull request, so that I fix the conflict and do not look for a fault in the check.

When a pull request conflicts with main, GitHub makes no merge ref (`refs/pull/<n>/merge`). The second checkout of the `private-data` workflow (`.github/workflows/private-data.yml`, on `pull_request_target`) then stops with `fatal: couldn't find remote ref refs/pull/<n>/merge` (exit 128). The required check is red, and the message does not give the cause. PR #307 had this failure 3 times on 2026-11-04 (runs 41838226461, 41838578798 and 41838594649). The runs passed after main was merged into the branch. Two roles of the team spent time on the cause.

The scope:

- 1 step before the checkout of the pull request. It finds out if the merge ref exists. If the merge ref does not exist, the step fails with the message above, with the number of the pull request.
- The step does not check out, run, source, or read a file of the pull request. The rules at the top of the workflow stay: no `ref:` on the first checkout, the same job name, the environment and the gate step (ADR 0027, DECISION-57).
- GitHub can make the merge ref some seconds after a push. The step waits a short time before it fails, so that a new push does not fail by mistake. The spec gives the wait.
- A pull request with no conflict runs as before.

No page of the knowledge graphs names this failure. ADR 0027 lists the other known failures of the check.

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [CRITERION-144: A conflicting pull request fails with a clear message](../criteria/CRITERION-144.md) | - | - | - | - | - |
| [CRITERION-145: A pull request with no conflict runs as before](../criteria/CRITERION-145.md) | - | - | - | - | - |
| [CRITERION-148: A late merge ref runs as before](../criteria/CRITERION-148.md) | - | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [EPIC-05: Safe public release](../epics/EPIC-05.md)
- milestone: [MILESTONE-02: Make it good](../milestones/MILESTONE-02.md)
- traces: [CONSTRAINT-01: The public release holds no private data](../../prd/constraints/CONSTRAINT-01.md)
- criterion or run of: [CRITERION-144: A conflicting pull request fails with a clear message](../criteria/CRITERION-144.md)
- criterion or run of: [CRITERION-145: A pull request with no conflict runs as before](../criteria/CRITERION-145.md)
- criterion or run of: [CRITERION-148: A late merge ref runs as before](../criteria/CRITERION-148.md)
- discovered: [STORY-82: Limit the keys of the merge ref step in the private-data test](STORY-82.md)
<!-- LINKS:END -->
