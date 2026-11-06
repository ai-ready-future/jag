---
type: Story
title: The README tells how to use a Clef server
description: As an operator, I want the README to tell how to use jag with a Clef
  server, and what jag does not promise for it, so that I set it up and know the limits.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-05T22:52:34Z'
state: backlog
github:
  number: 365
  node_id: I_kwDOVKMnL88AAAABc5F5qQ
  url: https://github.com/ai-ready-future/jag/issues/365
  pushed_hash: a9ea18
  synced:
    state: backlog
parent: /features/FEATURE-30.md
milestone: /milestones/MILESTONE-04.md
traces:
- to: prd:/capabilities/CAPABILITY-13.md
  hash: 79ed61
---

# STORY-90: The README tells how to use a Clef server

As an operator (PERSONA-02), I want the README to tell how to use jag with a Clef server, and what jag does not promise for it, so that I set it up and know the limits.

Source: DECISION-58, DECISION-60, DECISION-61, DECISION-62 and DECISION-63 (stable). The planner plan of SCOPE-04, 2026-11-05, after the answer of the user through hitl ("A: now, as backlog").

## Scope

The README has a section for a Clef server. It says:

1. Set the base URL to the address of the server. jag has no separate setting for a Clef server (DECISION-58). A server with no key check needs no key (DECISION-59, STORY-86).
2. jag works with a Clef server or a Clef-Flash server in the same way, and does not check which model the server runs (DECISION-61).
3. jag does not check where the server runs. The user must make sure that the server keeps the data inside the limits that apply, such as HIPAA (DECISION-60).
4. The quality numbers of jag are for Jev only. A user who wants a number for Clef can measure jag on their own questions (DECISION-62, CAPABILITY-12).
5. The 5-second target is for Jev only. A Clef server can be slower, and the user can raise the timeout (DECISION-63).

This story is a document change.

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [CRITERION-163: The README has a section for a Clef server](../criteria/CRITERION-163.md) | - | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [FEATURE-30: Score with a Clef server that the user runs](../features/FEATURE-30.md)
- milestone: [MILESTONE-04: Local model option](../milestones/MILESTONE-04.md)
- traces: [CAPABILITY-13: Score with a Clef server that the user runs](../../prd/capabilities/CAPABILITY-13.md)
- criterion or run of: [CRITERION-163: The README has a section for a Clef server](../criteria/CRITERION-163.md)
<!-- LINKS:END -->
