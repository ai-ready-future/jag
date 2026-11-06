---
type: Story
title: Show the cost and the failure of a Clef server correctly
description: As an operator, I want jag to mark the cost of a Clef search as not available,
  and to give the base URL and the status when the server fails, so that I see the
  truth.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-05T23:32:18Z'
state: backlog
github:
  number: 363
  node_id: I_kwDOVKMnL88AAAABc5Fwfw
  url: https://github.com/ai-ready-future/jag/issues/363
  pushed_hash: 1fb9b7
  synced:
    state: backlog
parent: /features/FEATURE-30.md
milestone: /milestones/MILESTONE-04.md
traces:
- to: prd:/capabilities/CAPABILITY-13.md
  hash: 79ed61
---

# STORY-88: Show the cost and the failure of a Clef server correctly

As an operator (PERSONA-02), I want jag to mark the cost of a search on my Clef server as not available, and to give the base URL and the status when the server gives an answer that jag cannot read, so that the report and the error are true for my server.

Source: DECISION-64 and DECISION-73 (stable). The planner plan of SCOPE-04, 2026-11-05, after the answer of the user through hitl ("A: now, as backlog").

## Scope

1. A Clef server sends no cost figure. jag shows the count of results that it scored, and marks the cost as not available (DECISION-64, DECISION-45).
2. When the server answers with an error, or with an answer that jag cannot read, jag fails with the service-down code of DECISION-05 (exit 6). The message gives the base URL and the status that the server sent (DECISION-73).

## Open points (for the spec)

- The cost on the code on `main`: `price_of` in `src/config/model.rs` gives the Jev price for the default model name on each route. With no `--model`, jag sends the default model name (DECISION-72), so a search on a Clef server shows a cost. jag does not know if a base URL is a Clef server or a gateway that calls Jev (DECISION-60, DECISION-61). The spec decides the rule for the cost on a base URL that is not TypeSafe.
- The message on `main` ("the gateway gave a response that jag cannot read.") gives neither the base URL nor the status. A code comment in `src/scorer.rs` says that no failure text holds an address. DECISION-73 is stronger, and the address rule of jag refuses a user name and a password in a base URL.

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [CRITERION-160: The cost of a Clef search is not available](../criteria/CRITERION-160.md) | - | - | - | - | - |
| [CRITERION-161: An answer that jag cannot read names the base URL and the status](../criteria/CRITERION-161.md) | - | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [FEATURE-30: Score with a Clef server that the user runs](../features/FEATURE-30.md)
- milestone: [MILESTONE-04: Local model option](../milestones/MILESTONE-04.md)
- traces: [CAPABILITY-13: Score with a Clef server that the user runs](../../prd/capabilities/CAPABILITY-13.md)
- criterion or run of: [CRITERION-160: The cost of a Clef search is not available](../criteria/CRITERION-160.md)
- criterion or run of: [CRITERION-161: An answer that jag cannot read names the base URL and the status](../criteria/CRITERION-161.md)
<!-- LINKS:END -->
