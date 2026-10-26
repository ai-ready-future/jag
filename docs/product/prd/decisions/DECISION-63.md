---
type: Decision
title: jag has no speed target with a Clef server
description: The 5-second search target is for Jev only; with a slow Clef server,
  the user can raise the timeout.
status: stable
decided_at: '2026-10-26'
alternatives:
- The same 5-second target with Clef, with fewer results scored
about:
- /capabilities/CAPABILITY-13.md
- /constraints/CONSTRAINT-06.md
---

# DECISION-63: jag has no speed target with a Clef server

The 5-second target of [CONSTRAINT-06](../constraints/CONSTRAINT-06.md) applies to Jev only. A Clef server can be slower, and it can answer one call at a time. The user can raise the timeout for a slow server.

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-13: Score with a Clef server that the user runs](../capabilities/CAPABILITY-13.md)
- about: [CONSTRAINT-06: A typical search is one search on a medium code repository](../constraints/CONSTRAINT-06.md)
<!-- LINKS:END -->
