---
type: Story
title: Hold back each snippet that holds a key, a token or a password
description: As an operator, I want jag to hold back each snippet that holds a key,
  a token or a password, so that no secret goes to a model service.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-05T22:52:34Z'
state: backlog
parent: /features/FEATURE-31.md
milestone: /milestones/MILESTONE-04.md
traces:
- to: prd:/capabilities/CAPABILITY-14.md
  hash: fdaf5e
---

# STORY-91: Hold back each snippet that holds a key, a token or a password

As an operator (PERSONA-02), I want jag to hold back each snippet that holds a key, a token or a password, so that no secret goes to a model service.

Source: DECISION-66, DECISION-67, DECISION-68, DECISION-77 and DECISION-79 (stable). The planner plan of SCOPE-04, 2026-11-05, after the answer of the user through hitl ("A: now, as backlog").

## Scope

1. jag has a built-in list of patterns for keys, tokens and passwords (DECISION-68).
2. The check is on by default for each model service: TypeSafe, an AI gateway, and a Clef server. jag does not turn it off by itself (DECISION-66).
3. The check examines all text that jag sends to the model: the question, and each result with its lines around the match (DECISION-77).
4. jag does not send or score a result that matches. The result lists the line as not scored, with the reason. The search goes on with the other results (DECISION-67).
5. When the question matches, jag stops before any model call, with a clear message (DECISION-77).
6. Held-back lines add no exit code. The report counts them. The cost-limit code still wins (DECISION-79, DECISION-47).
7. A term with no question makes no model call, so jag does not check it (DECISION-81, DECISION-32).

The spec decides the built-in patterns.

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [CRITERION-164: A result that matches a built-in pattern is not sent](../criteria/CRITERION-164.md) | - | - | - | - | - |
| [CRITERION-165: A question that matches stops before any model call](../criteria/CRITERION-165.md) | - | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [FEATURE-31: Hold private text back from the model](../features/FEATURE-31.md)
- milestone: [MILESTONE-04: Local model option](../milestones/MILESTONE-04.md)
- traces: [CAPABILITY-14: Hold private text back from the model](../../prd/capabilities/CAPABILITY-14.md)
- criterion or run of: [CRITERION-164: A result that matches a built-in pattern is not sent](../criteria/CRITERION-164.md)
- criterion or run of: [CRITERION-165: A question that matches stops before any model call](../criteria/CRITERION-165.md)
- blocks: [STORY-92: Add private patterns of the user in the config file](STORY-92.md)
- blocks: [STORY-93: Turn the private-pattern check off with one setting](STORY-93.md)
- blocks: [STORY-94: The preview lists the lines that the check holds back](STORY-94.md)
<!-- LINKS:END -->
