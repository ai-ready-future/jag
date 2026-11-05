---
type: Feature
title: The config file holds the call timeout, and setup writes each setting
description: The user sets the time of one model call in the config file one time,
  and jag --setup writes each non-secret setting that the answer to the open points
  of STORY-84 names, with its default value.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-05T20:26:06Z'
state: backlog
parent: /epics/EPIC-01.md
milestone: /milestones/MILESTONE-02.md
traces:
- to: prd:/capabilities/CAPABILITY-01.md
  hash: 295cfa
---

# FEATURE-29: The config file holds the call timeout, and setup writes each setting

The user sets the time of one model call in the config file `~/.config/jag/config.yml` one time, and not in each shell or job. After `jag --setup`, the user sees each non-secret setting that the answer to the open points of STORY-84 names in the file, with its default value.

Source: request REQ-config-timeout, 2026-11-05, from the requests session.

This feature changes the contract of the config file. ADR-0015 says that jag writes no other setting to the file, and DF-1 of the system design keeps the times out of the file. So the spec needs an amend or a supersede of ADR-0015, and a change of DF-1, before the build.

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [STORY-84: Set the call timeout in the config file, and let setup write each setting with its default](../stories/STORY-84.md) | backlog | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [EPIC-01: Setup and readiness](../epics/EPIC-01.md)
- milestone: [MILESTONE-02: Make it good](../milestones/MILESTONE-02.md)
- traces: [CAPABILITY-01: Guided key setup and health check](../../prd/capabilities/CAPABILITY-01.md)
- child: [STORY-84: Set the call timeout in the config file, and let setup write each setting with its default](../stories/STORY-84.md)
<!-- LINKS:END -->
