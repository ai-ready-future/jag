---
type: Story
title: Turn the private-pattern check off with one setting
description: As an operator, I want one setting that turns the private-pattern check
  off, for example for a Clef server that I trust, so that jag scores each snippet.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-05T22:52:35Z'
state: backlog
parent: /features/FEATURE-31.md
blocked_by:
- /stories/STORY-91.md
- /stories/STORY-84.md
- /stories/STORY-85.md
milestone: /milestones/MILESTONE-04.md
traces:
- to: prd:/capabilities/CAPABILITY-14.md
  hash: fdaf5e
---

# STORY-93: Turn the private-pattern check off with one setting

As an operator (PERSONA-02), I want one setting that turns the private-pattern check off, for example for a Clef server that I trust, so that jag scores each snippet.

Source: DECISION-80 and DECISION-66 (stable). The planner plan of SCOPE-04, 2026-11-05, after the answer of the user through hitl ("A: now, as backlog").

## Scope

1. One setting turns the check off. It turns off the built-in list and the list of the user (DECISION-80).
2. The report of each search then says that the check is off (DECISION-80).
3. The setting has an option and a `JAG_` variable. By the rule of the user in STORY-84, it is also a config file key, and `jag --setup` shows it as a comment line with its default (on, STORY-85).

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [CRITERION-168: One setting turns off both lists, and the report says so](../criteria/CRITERION-168.md) | - | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [FEATURE-31: Hold private text back from the model](../features/FEATURE-31.md)
- blocked by: [STORY-91: Hold back each snippet that holds a key, a token or a password](STORY-91.md)
- blocked by: [STORY-84: Set each non-secret setting in the config file](STORY-84.md)
- blocked by: [STORY-85: jag --setup shows each missing setting as a comment line with its default](STORY-85.md)
- milestone: [MILESTONE-04: Local model option](../milestones/MILESTONE-04.md)
- traces: [CAPABILITY-14: Hold private text back from the model](../../prd/capabilities/CAPABILITY-14.md)
- criterion or run of: [CRITERION-168: One setting turns off both lists, and the report says so](../criteria/CRITERION-168.md)
<!-- LINKS:END -->
