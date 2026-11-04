---
type: Story
title: Limit the keys of the merge ref step in the private-data test
description: As a maintainer, I want the private-data test to fail when the merge
  ref step gets a key other than name, if, env or run, so that the step cannot change
  without a test failure.
status: draft
generated:
  by: process:github
  at: '2026-11-04T15:27:02Z'
state: backlog
github:
  number: 321
  node_id: I_kwDOVKMnL88AAAABceH68w
  url: https://github.com/ai-ready-future/jag/issues/321
  pushed_hash: 2cb948
  synced:
    state: backlog
    owner: human:404pilo
parent: /epics/EPIC-05.md
milestone: /milestones/MILESTONE-02.md
owner: human:404pilo
discovered_from: /stories/STORY-80.md
traces:
- to: prd:/constraints/CONSTRAINT-01.md
  hash: 1970e3
---

# STORY-82: Limit the keys of the merge ref step in the private-data test

As a maintainer, I want the test of the private-data workflow to fail when the merge ref step gets a key that is not `name`, `if`, `env` or `run`, so that a later change cannot make that step run in a different way without a test failure.

The security review of PR #320 (STORY-80) found this, with low severity (issue #321). In `scripts/private-data/test.sh`, case `wf-h` checks the `env:` keys, the `run:` line, and that the step `Check the merge ref` has no `uses:`, secret or token. It does not check the other keys of the step. So a later change could add `working-directory:` or `shell:` to the step, and the case would still pass. PR #320 is safe: the step runs before the checkout of the pull request, and it runs a file of `base`.

The scope:

- In case `wf-h`, check that the step `Check the merge ref` has only the keys `name`, `if`, `env` and `run`.
- Test the case with a changed copy of the workflow that has 1 extra key. The case must fail on that copy.
- The workflow does not change.

The GitHub issue of this story is #321. The tracker links it, and does not open a second issue.

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [CRITERION-149: The test fails on an extra key of the merge ref step](../criteria/CRITERION-149.md) | - | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [EPIC-05: Safe public release](../epics/EPIC-05.md)
- milestone: [MILESTONE-02: Make it good](../milestones/MILESTONE-02.md)
- discovered from: [STORY-80: Name the merge conflict when private-data cannot check out a PR](STORY-80.md)
- traces: [CONSTRAINT-01: The public release holds no private data](../../prd/constraints/CONSTRAINT-01.md)
- criterion or run of: [CRITERION-149: The test fails on an extra key of the merge ref step](../criteria/CRITERION-149.md)
<!-- LINKS:END -->
