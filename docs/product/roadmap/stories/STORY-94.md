---
type: Story
title: The preview lists the lines that the check holds back
description: As a knowledge seeker, I want the preview of the text that leaves the
  machine to list each line that the check holds back, so that I see what jag does
  not send.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-06T06:43:57Z'
state: backlog
github:
  number: 369
  node_id: I_kwDOVKMnL88AAAABc5GO-w
  url: https://github.com/ai-ready-future/jag/issues/369
  synced:
    state: backlog
  pushed_hash: 26eac8
parent: /features/FEATURE-31.md
blocked_by:
- /stories/STORY-91.md
- /stories/STORY-95.md
milestone: /milestones/MILESTONE-04.md
traces:
- to: prd:/capabilities/CAPABILITY-14.md
  hash: fdaf5e
---

# STORY-94: The preview lists the lines that the check holds back

As a knowledge seeker (PERSONA-03), I want the preview of the text that leaves the machine to list each line that the check holds back, with the reason, so that I see what jag does not send.

Source: DECISION-81 (stable). The planner plan of SCOPE-04, 2026-11-05, after the answer of the user through hitl ("A: now, as backlog").

## Scope

1. The preview lists each line that the private-pattern check holds back, with the reason (DECISION-81).
2. A term with no question makes no model call, so the preview has nothing to list for it (DECISION-81, DECISION-32).

This story adds the held-back lines to the preview of STORY-95 (FEATURE-32, CAPABILITY-09 "Say which text leaves the machine"), so STORY-95 blocks it. The user chose this plan through hitl, 2026-11-05: "A: now, in Local model option".

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [CRITERION-169: The preview lists each held-back line with the reason](../criteria/CRITERION-169.md) | - | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [FEATURE-31: Hold private text back from the model](../features/FEATURE-31.md)
- blocked by: [STORY-91: Hold back each snippet that holds a key, a token or a password](STORY-91.md)
- blocked by: [STORY-95: See the text that a search sends before jag sends it](STORY-95.md)
- milestone: [MILESTONE-04: Local model option](../milestones/MILESTONE-04.md)
- traces: [CAPABILITY-14: Hold private text back from the model](../../prd/capabilities/CAPABILITY-14.md)
- criterion or run of: [CRITERION-169: The preview lists each held-back line with the reason](../criteria/CRITERION-169.md)
<!-- LINKS:END -->
