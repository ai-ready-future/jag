---
type: Feature
title: The config file holds each non-secret setting, and setup shows each default
description: The user sets each non-secret setting of jag in the config file one time,
  and jag --setup shows each missing setting as a comment line with its default.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-05T20:31:09Z'
state: backlog
github:
  number: 348
  node_id: I_kwDOVKMnL88AAAABc2medg
  url: https://github.com/ai-ready-future/jag/issues/348
  pushed_hash: 406add
  synced:
    state: backlog
parent: /epics/EPIC-01.md
milestone: /milestones/MILESTONE-02.md
traces:
- to: prd:/capabilities/CAPABILITY-01.md
  hash: 295cfa
---

# FEATURE-29: The config file holds each non-secret setting, and setup shows each default

The user sets each non-secret setting of jag in the config file `~/.config/jag/config.yml` one time, and not in each shell or job. After `jag --setup`, the file shows each non-secret setting with its default value, as a comment line.

Source: request REQ-config-timeout, 2026-11-05, from the requests session, and the answer of the user through hitl, 2026-11-05.

The rule of the user: each non-secret setting that jag accepts as a parameter (an option or a variable) is also a config file setting. The keys stay out of what setup writes.

- STORY-84: the config file holds each non-secret setting.
- STORY-85: `jag --setup` shows each missing setting as a comment line with its default.

This feature changes the contract of the config file. ADR-0015 says that jag writes no other setting to the file, and DF-1 of the system design keeps the times and the limits out of the file. The architect changes ADR-0015 and DF-1 before the build. The 2 stories stay in `backlog` until that ADR change merges.

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [STORY-84: Set each non-secret setting in the config file](../stories/STORY-84.md) | ready | - | - | - | - |
| [STORY-85: jag --setup shows each missing setting as a comment line with its default](../stories/STORY-85.md) | blocked | - | - | STORY-84 | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [EPIC-01: Setup and readiness](../epics/EPIC-01.md)
- milestone: [MILESTONE-02: Make it good](../milestones/MILESTONE-02.md)
- traces: [CAPABILITY-01: Guided key setup and health check](../../prd/capabilities/CAPABILITY-01.md)
- child: [STORY-84: Set each non-secret setting in the config file](../stories/STORY-84.md)
- child: [STORY-85: jag --setup shows each missing setting as a comment line with its default](../stories/STORY-85.md)
<!-- LINKS:END -->
