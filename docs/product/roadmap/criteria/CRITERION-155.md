---
type: AcceptanceCriterion
title: A search calls the gateway base URL with no key when no gateway key is set
description: With a base URL that is not TypeSafe and no gateway key, a search calls
  the base URL with no Authorization header, and no TypeSafe key goes to it.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-05T21:35:49Z'
item: /stories/STORY-86.md
---

# CRITERION-155: A search calls the gateway base URL with no key when no gateway key is set

With a base URL that is not TypeSafe, and with no `JAG_GATEWAY_API_KEY` and no `config.gateway-api-key`, a search calls the base URL with no `Authorization` header and does not stop before the call. A server that answers the call gives the normal result of the search. A TypeSafe key in `JAG_TYPESAFE_API_KEY`, `TYPESAFE_API_KEY` or `config.api-key` is not sent to that base URL.

<!-- LINKS:START (generated, do not edit) -->

## Links

- item: [STORY-86: Call the gateway base URL with no key when the user sets no gateway key](../stories/STORY-86.md)
<!-- LINKS:END -->
