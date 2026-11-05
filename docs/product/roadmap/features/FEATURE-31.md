---
type: Feature
title: Hold private text back from the model
description: Before each model call, jag checks the question and each snippet against
  a list of private patterns, and does not send a match.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-05T22:52:34Z'
state: backlog
github:
  number: 361
  node_id: I_kwDOVKMnL88AAAABc5FnwQ
  url: https://github.com/ai-ready-future/jag/issues/361
  pushed_hash: 72fa31
  synced:
    state: backlog
parent: /epics/EPIC-07.md
milestone: /milestones/MILESTONE-04.md
traces:
- to: prd:/capabilities/CAPABILITY-14.md
  hash: fdaf5e
---

# FEATURE-31: Hold private text back from the model

Before each model call, jag checks the question and each snippet against a list of private patterns, and does not send a match (CAPABILITY-14). The check is on by default for each model service.

The stories of this feature:
- STORY-91: the built-in patterns, and what jag does with a match (DECISION-66, DECISION-67, DECISION-68, DECISION-77, DECISION-79).
- STORY-92: the patterns of the user in the config file (DECISION-68, DECISION-78).
- STORY-93: one setting turns the check off (DECISION-80).
- STORY-94: the preview of the text that leaves the machine lists the held-back lines (DECISION-81).

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [STORY-91: Hold back each snippet that holds a key, a token or a password](../stories/STORY-91.md) | backlog | - | - | - | - |
| [STORY-92: Add private patterns of the user in the config file](../stories/STORY-92.md) | backlog | - | - | STORY-91 | - |
| [STORY-93: Turn the private-pattern check off with one setting](../stories/STORY-93.md) | backlog | - | - | STORY-91, STORY-84, STORY-85 | - |
| [STORY-94: The preview lists the lines that the check holds back](../stories/STORY-94.md) | backlog | - | - | STORY-91 | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [EPIC-07: Local model option](../epics/EPIC-07.md)
- milestone: [MILESTONE-04: Local model option](../milestones/MILESTONE-04.md)
- traces: [CAPABILITY-14: Hold private text back from the model](../../prd/capabilities/CAPABILITY-14.md)
- child: [STORY-91: Hold back each snippet that holds a key, a token or a password](../stories/STORY-91.md)
- child: [STORY-92: Add private patterns of the user in the config file](../stories/STORY-92.md)
- child: [STORY-93: Turn the private-pattern check off with one setting](../stories/STORY-93.md)
- child: [STORY-94: The preview lists the lines that the check holds back](../stories/STORY-94.md)
<!-- LINKS:END -->
