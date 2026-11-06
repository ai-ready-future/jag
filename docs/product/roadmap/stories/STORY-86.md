---
type: Story
title: Call the gateway base URL with no key when the user sets no gateway key
description: As an operator, I want jag to call my model server with no key when I
  set no gateway key, so that a server with no key check works with no extra setting.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-05T21:35:49Z'
state: ready
github:
  number: 356
  node_id: I_kwDOVKMnL88AAAABc4T9Bw
  url: https://github.com/ai-ready-future/jag/issues/356
  pushed_hash: 2e8b4c
  synced:
    state: ready
spec: docs/superpowers/specs/2026-11-05-STORY-86-call-the-gateway-base-url-with-no-key-design.md
parent: /features/FEATURE-21.md
milestone: /milestones/MILESTONE-02.md
traces:
- to: prd:/capabilities/CAPABILITY-13.md
  hash: 79ed61
---

# STORY-86: Call the gateway base URL with no key when the user sets no gateway key

As an operator (PERSONA-02), I want jag to call my model server with no key when I set no gateway key, so that a Clef server or another server with no key check works with no extra setting.

Source: a finding of the reviewer, 2026-11-05, sent by the orchestrator. ADR-0028 (accepted 2026-10-29) partly supersedes ADR-0017, but the code on `main` does not follow it yet: `find_key` in `src/config.rs` stops the gateway route with `GatewayKeyMissing` (exit 3) before the call. No roadmap item named ADR-0028, DECISION-59, DECISION-71 or DECISION-75.

## Scope

The gateway route is a base URL that is not TypeSafe (ADR-0015, ADR-0017).

1. When jag finds no gateway key in `JAG_GATEWAY_API_KEY` or in `config.gateway-api-key`, jag calls the base URL with no `Authorization` header. It does not stop before the call (ADR-0028, DECISION-71).
2. When the server answers `HTTP 401` or `HTTP 403` to a call with no key, jag fails with the key-not-accepted exit code of DECISION-05. The message names `JAG_GATEWAY_API_KEY` (ADR-0028).
3. jag never sends a TypeSafe key (`JAG_TYPESAFE_API_KEY`, `TYPESAFE_API_KEY`, `config.api-key`) to the base URL of the gateway route (ADR-0017, DECISION-59). This rule stays.
4. `jag --check` on the gateway route with no gateway key reports the 2 server items: the server is reachable, and the server answers a scoring call. The key items say "not needed", and the credit item says "not available". A key item that is not needed does not fail the check (DECISION-75, ADR-0028 Consequences).
5. The end-to-end tests have a case for the gateway route with no gateway key (ADR-0028 Consequences).

The other rules of ADR-0017 stay: the sources of the gateway key and their order, and the direct route. The direct route to TypeSafe still stops with exit 3 when jag finds no TypeSafe key.

Not in scope: the other parts of CAPABILITY-13 (DECISION-65 and DECISION-76: a call or a snippet that is too long for the server; DECISION-74: the report names the host) and CAPABILITY-14 (private patterns). They are in SCOPE-04 and have no roadmap item yet. The planner asks the human about the plan of SCOPE-04 separately.

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [CRITERION-155: A search calls the gateway base URL with no key when no gateway key is set](../criteria/CRITERION-155.md) | - | - | - | - | - |
| [CRITERION-156: A server that refuses the call for a key gives the key-not-accepted failure](../criteria/CRITERION-156.md) | - | - | - | - | - |
| [CRITERION-157: The readiness check of a server with no key reports the server items](../criteria/CRITERION-157.md) | - | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [FEATURE-21: Send the model calls through an AI gateway](../features/FEATURE-21.md)
- milestone: [MILESTONE-02: Make it good](../milestones/MILESTONE-02.md)
- traces: [CAPABILITY-13: Score with a Clef server that the user runs](../../prd/capabilities/CAPABILITY-13.md)
- criterion or run of: [CRITERION-155: A search calls the gateway base URL with no key when no gateway key is set](../criteria/CRITERION-155.md)
- criterion or run of: [CRITERION-156: A server that refuses the call for a key gives the key-not-accepted failure](../criteria/CRITERION-156.md)
- criterion or run of: [CRITERION-157: The readiness check of a server with no key reports the server items](../criteria/CRITERION-157.md)
<!-- LINKS:END -->
