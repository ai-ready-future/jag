---
type: Story
title: Keep the rg exit codes
description: As a shell power user (PERSONA-04), I want the rg exit codes 0, 1, and
  2, so that my scripts that test rg exit codes keep working.
status: draft
state: done
actual_end_date: '2026-10-14'
github:
  number: 63
  node_id: I_kwDOVKMnL88AAAABXTMbug
  url: https://github.com/ai-ready-future/jag/issues/63
  pushed_hash: ac13db
  synced:
    state: done
spec: docs/superpowers/specs/2026-10-14-STORY-35-common-grep-options-and-exit-codes-design.md
parent: /features/FEATURE-13.md
milestone: /milestones/MILESTONE-02.md
traces:
- to: prd:/decisions/DECISION-05.md
  hash: b8f39e
---

# STORY-38: Keep the rg exit codes

As a shell power user (PERSONA-04), I want jag to exit with 0 when it returns a result, 1 when it finds no match, and 2 for a usage error, so that my scripts that test the rg exit codes keep working.

Source of the change from grep to rg: the user, hitl answer of 2026-10-15 (FEATURE-23, Q1 option 1: rg is the reference for jag with no question).

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [CRITERION-58: jag keeps the rg exit codes](../criteria/CRITERION-58.md) | - | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [FEATURE-13: Common rg options and exit codes](../features/FEATURE-13.md)
- milestone: [MILESTONE-02: Make it good](../milestones/MILESTONE-02.md)
- traces: [DECISION-05: jag keeps rg exit codes and adds one code for each model failure](../../prd/decisions/DECISION-05.md)
- criterion or run of: [CRITERION-58: jag keeps the rg exit codes](../criteria/CRITERION-58.md)
<!-- LINKS:END -->
