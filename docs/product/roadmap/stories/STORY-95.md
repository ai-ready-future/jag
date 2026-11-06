---
type: Story
title: See the text that a search sends before jag sends it
description: As a knowledge seeker, I want to see the text that a search sends to
  the model service before jag sends it, so that I can decide whether to run the search.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-06T06:43:51Z'
state: backlog
github:
  number: 379
  node_id: I_kwDOVKMnL88AAAABc-EwHw
  url: https://github.com/ai-ready-future/jag/issues/379
  pushed_hash: 2ed953
  synced:
    state: backlog
parent: /features/FEATURE-32.md
milestone: /milestones/MILESTONE-04.md
traces:
- to: prd:/capabilities/CAPABILITY-09.md
  hash: 1e8d02
---

# STORY-95: See the text that a search sends before jag sends it

As a knowledge seeker (PERSONA-03), I want to see the text that a search sends to the model service before jag sends it, so that I can decide whether to run the search (CAPABILITY-09).

Source: the answer of the user through hitl, 2026-11-05: "A: now, in Local model option".

## Scope

1. The preview shows the question and the text of each result that the search sends to the model service (DECISION-77).
2. The preview makes no model call.
3. A search with no question makes no model call (DECISION-32), so the preview tells that no text leaves the machine (DECISION-81).
4. The spec decides the flag or the command of the preview, and its form in the text output and in the structured output.

STORY-94 adds the lines that the private-pattern check holds back to this preview (DECISION-81).

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [CRITERION-170: The preview shows the text that the search sends](../criteria/CRITERION-170.md) | - | - | - | - | - |
| [CRITERION-171: A search with no question has nothing to preview](../criteria/CRITERION-171.md) | - | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [FEATURE-32: Say which text leaves the machine](../features/FEATURE-32.md)
- milestone: [MILESTONE-04: Local model option](../milestones/MILESTONE-04.md)
- traces: [CAPABILITY-09: Say which text leaves the machine](../../prd/capabilities/CAPABILITY-09.md)
- criterion or run of: [CRITERION-170: The preview shows the text that the search sends](../criteria/CRITERION-170.md)
- criterion or run of: [CRITERION-171: A search with no question has nothing to preview](../criteria/CRITERION-171.md)
- blocks: [STORY-94: The preview lists the lines that the check holds back](STORY-94.md)
<!-- LINKS:END -->
