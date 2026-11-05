---
type: AcceptanceCriterion
title: The readiness check of a server with no key reports the server items
description: jag --check with no gateway key reports the 2 server items; the key items
  say not needed, the credit item says not available.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-05T21:35:50Z'
item: /stories/STORY-86.md
---

# CRITERION-157: The readiness check of a server with no key reports the server items

`jag --check` with a base URL that is not TypeSafe and no gateway key reports whether the server is reachable and whether it answers a scoring call. The key items say "not needed", and the credit item says "not available". With a server that is reachable and answers, the check passes.

<!-- LINKS:START (generated, do not edit) -->

## Links

- item: [STORY-86: Call the gateway base URL with no key when the user sets no gateway key](../stories/STORY-86.md)
<!-- LINKS:END -->
