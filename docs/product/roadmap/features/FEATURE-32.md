---
type: Feature
title: Say which text leaves the machine
description: Before a search, jag shows the text that it sends to the model service,
  and after each search it reports which text left the machine.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-06T06:43:51Z'
state: backlog
github:
  number: 378
  node_id: I_kwDOVKMnL88AAAABc-Emfg
  url: https://github.com/ai-ready-future/jag/issues/378
  pushed_hash: 5491f6
  synced:
    state: backlog
parent: /epics/EPIC-07.md
milestone: /milestones/MILESTONE-04.md
traces:
- to: prd:/capabilities/CAPABILITY-09.md
  hash: 1e8d02
---

# FEATURE-32: Say which text leaves the machine

jag tells the user which text it sends to the model service, so that the user can decide before a search (CAPABILITY-09). CAPABILITY-09 is in SCOPE-01 (could) and SCOPE-02 (must).

Source: the answer of the user through hitl, 2026-11-05: "A: now, in Local model option". This feature is in MILESTONE-04, in backlog, before STORY-89 and STORY-94. No build starts before release 1.

The stories of this feature:
- STORY-95: the preview shows the text that a search sends, and makes no model call. STORY-94 adds the held-back lines to this preview (DECISION-81).
- STORY-96: the report of each search that calls the model tells which text left the machine. STORY-89 adds the host to this report (DECISION-74).

The PRD does not decide these items. The spec decides them:
- The flag or the command that shows the preview.
- If the report shows by default, or only on request.
- Where the report goes (standard output or standard error), and its field in the structured output (CAPABILITY-05).

For the specifier: the text that jag sends is the question and each result with the lines around its match (DECISION-77), in calls of 32 results (ADR-0002). INTERFACE-06 and FEATURE-04 can tell more about the result text that goes to the scorer.

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [STORY-95: See the text that a search sends before jag sends it](../stories/STORY-95.md) | backlog | - | - | - | - |
| [STORY-96: The report of each search tells which text left the machine](../stories/STORY-96.md) | backlog | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [EPIC-07: Local model option](../epics/EPIC-07.md)
- milestone: [MILESTONE-04: Local model option](../milestones/MILESTONE-04.md)
- traces: [CAPABILITY-09: Say which text leaves the machine](../../prd/capabilities/CAPABILITY-09.md)
- child: [STORY-95: See the text that a search sends before jag sends it](../stories/STORY-95.md)
- child: [STORY-96: The report of each search tells which text left the machine](../stories/STORY-96.md)
<!-- LINKS:END -->
