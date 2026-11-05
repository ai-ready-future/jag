---
type: Story
title: A search works when a call is too long for the server
description: As an operator, I want a search to work when a call is too long for my
  Clef server, so that a server that reads less text in one call still gives a result.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-05T22:52:33Z'
state: backlog
parent: /features/FEATURE-30.md
milestone: /milestones/MILESTONE-04.md
traces:
- to: prd:/capabilities/CAPABILITY-13.md
  hash: 79ed61
---

# STORY-87: A search works when a call is too long for the server

As an operator (PERSONA-02), I want a search to work when a call is too long for my Clef server, so that a server that reads less text in one call still gives a result.

Source: DECISION-65 and DECISION-76 (stable). The planner plan of SCOPE-04, 2026-11-05, after the answer of the user through hitl ("A: now, as backlog").

## Scope

1. When the server refuses a call because it is too long, jag sends the same snippets in smaller calls, and the search still works (DECISION-65).
2. When one snippet alone is too long for the server, jag does not score it. The result lists the line as not scored, with the reason "too long for the server". The search goes on with the other snippets (DECISION-76).
3. jag never cuts a snippet, and it never accepts a score of text that the server cut (DECISION-65).

The spec decides how jag knows that the server refused a call because it is too long.

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [CRITERION-158: A call that is too long goes again in smaller calls](../criteria/CRITERION-158.md) | - | - | - | - | - |
| [CRITERION-159: A snippet that is too long by itself is not scored](../criteria/CRITERION-159.md) | - | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [FEATURE-30: Score with a Clef server that the user runs](../features/FEATURE-30.md)
- milestone: [MILESTONE-04: Local model option](../milestones/MILESTONE-04.md)
- traces: [CAPABILITY-13: Score with a Clef server that the user runs](../../prd/capabilities/CAPABILITY-13.md)
- criterion or run of: [CRITERION-158: A call that is too long goes again in smaller calls](../criteria/CRITERION-158.md)
- criterion or run of: [CRITERION-159: A snippet that is too long by itself is not scored](../criteria/CRITERION-159.md)
<!-- LINKS:END -->
