---
type: AcceptanceCriterion
title: A server that refuses the call for a key gives the key-not-accepted failure
description: With no gateway key, HTTP 401 or 403 from the server gives the key-not-accepted
  exit code, and the message names JAG_GATEWAY_API_KEY.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-05T21:35:49Z'
item: /stories/STORY-86.md
---

# CRITERION-156: A server that refuses the call for a key gives the key-not-accepted failure

With a base URL that is not TypeSafe and no gateway key, when the server answers `HTTP 401` or `HTTP 403`, jag exits with the key-not-accepted exit code of DECISION-05, and the message names `JAG_GATEWAY_API_KEY`. The `GatewayKeyMissing` stop before the call (exit 3) is gone for this route.

<!-- LINKS:START (generated, do not edit) -->

## Links

- item: [STORY-86: Call the gateway base URL with no key when the user sets no gateway key](../stories/STORY-86.md)
<!-- LINKS:END -->
