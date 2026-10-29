---
type: Decision
title: A search still works when one call is too long for the server
description: When a call is too long for a Clef server, jag sends smaller calls, so
  that the search still works and does not fail.
status: stable
decided_at: '2026-10-26'
alternatives:
- jag fails the search with a clear message and an exit code
about:
- /capabilities/CAPABILITY-13.md
---

# DECISION-65: A search still works when one call is too long for the server

A Clef server reads a limited length of text in one call. When a call is too long for the server, jag sends smaller calls, and the search still works. jag never accepts a score of text that the server cut.

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-13: Score with a Clef server that the user runs](../capabilities/CAPABILITY-13.md)
- subject of: [DECISION-76: jag skips a snippet that is too long for the server by itself](DECISION-76.md)
<!-- LINKS:END -->
