---
type: adr
title: Call a base URL with no key when the user sets no gateway key
description: Open this when you change what jag does on the gateway route with no gateway key, or how it reports a server that refuses a call for a key.
area: contracts
status: accepted
date: 2026-10-29
supersedes: 0017-use-a-separate-key-for-an-ai-gateway.md # partial
superseded_by: null
amends: null
refines: null
deviates_from: null
related: [0015-read-the-base-url-and-the-model-name-from-config-base-url.md]
parent_spec: null
model:
  status: modeled
  element: C-config
  reason: C-config finds the key for the route of each call
---

# ADR-0028: Call a base URL with no key when the user sets no gateway key

## Context

ADR-0017 stops a search on the gateway route with exit 3 when jag finds no gateway key. CAPABILITY-13 lets the user set the base URL to a Clef or Clef-Flash server that the user runs. Such a server often has no key check: `clef_mlx.py serve` has none, and `llama-server` has one only with `--api-key`. DECISION-59 says that a key is optional on a base URL that is not TypeSafe. DECISION-71 says that jag calls with no key when the user sets none, and that a refusal for a key gives the key-not-accepted failure. The stop rule of ADR-0017 blocks each Clef server with no key.

## Decision

On the gateway route, when jag finds no gateway key in `JAG_GATEWAY_API_KEY` or in `config.gateway-api-key`, jag calls the base URL with no `Authorization` header. It does not stop before the call. When the server answers `HTTP 401` or `HTTP 403`, jag fails with the key-not-accepted exit code of DECISION-05. The message names `JAG_GATEWAY_API_KEY`.

The other rules of ADR-0017 stay: the gateway key sources and their order, the direct route, and the rule that jag never sends a TypeSafe key to a gateway.

## Why

- A Clef server with no key check works with no extra setting.
- A gateway that needs a key still gives a clear error, after one call, that names the setting to fix.
- A TypeSafe key still never goes to another host.

## Alternatives considered

- An empty gateway key means no key: the stop rule stays for an unset key. But each Clef user must set an empty value, which is hard to find and easy to get wrong.
- Keep the stop rule of ADR-0017: no Clef server with no key check can work.

## Consequences

- A user who forgets the gateway key for a gateway that needs one sees the error after one call, not before. The call sends no key, so no secret leaves the machine.
- `--check` on a server with no key reports the server items, and the key items say "not needed" (DECISION-75).
- The end-to-end tests need a case for the gateway route with no gateway key.
