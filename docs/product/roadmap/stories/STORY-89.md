---
type: Story
title: The report of the text that leaves the machine names the host
description: As a knowledge seeker, I want the report of the text that leaves the
  machine to name the host that gets it, so that I know where my text goes.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-06T06:43:56Z'
state: backlog
github:
  number: 364
  node_id: I_kwDOVKMnL88AAAABc5F0ow
  url: https://github.com/ai-ready-future/jag/issues/364
  synced:
    state: backlog
  pushed_hash: '196959'
parent: /features/FEATURE-30.md
blocked_by:
- /stories/STORY-96.md
milestone: /milestones/MILESTONE-04.md
traces:
- to: prd:/capabilities/CAPABILITY-13.md
  hash: 79ed61
---

# STORY-89: The report of the text that leaves the machine names the host

As a knowledge seeker (PERSONA-03), I want the report of the text that leaves the machine to name the host that gets it, so that I know if my text goes to TypeSafe, to a gateway, or to my own server.

Source: DECISION-74 (stable). The planner plan of SCOPE-04, 2026-11-05, after the answer of the user through hitl ("A: now, as backlog").

## Scope

1. The report of the text that leaves the machine always names the host of the base URL: TypeSafe, an AI gateway, or a Clef server (DECISION-74).
2. The report names the host only, not the full base URL (DECISION-74).

This story adds the host to the report of STORY-96 (FEATURE-32, CAPABILITY-09 "Say which text leaves the machine"), so STORY-96 blocks it. The user chose this plan through hitl, 2026-11-05: "A: now, in Local model option".

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [CRITERION-162: The report names the host that gets the text](../criteria/CRITERION-162.md) | - | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [FEATURE-30: Score with a Clef server that the user runs](../features/FEATURE-30.md)
- blocked by: [STORY-96: The report of each search tells which text left the machine](STORY-96.md)
- milestone: [MILESTONE-04: Local model option](../milestones/MILESTONE-04.md)
- traces: [CAPABILITY-13: Score with a Clef server that the user runs](../../prd/capabilities/CAPABILITY-13.md)
- criterion or run of: [CRITERION-162: The report names the host that gets the text](../criteria/CRITERION-162.md)
<!-- LINKS:END -->
