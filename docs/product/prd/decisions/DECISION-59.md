---
type: Decision
title: A Clef server needs no key
description: On a base URL that is not TypeSafe, a key is optional; jag sends a key
  only when the user sets one, and a TypeSafe key never goes to another host.
status: stable
decided_at: '2026-10-26'
alternatives:
- A key for each server, also a server with no key check
- No key for each host that is not TypeSafe, with no rule for a key that the user
  sets
about:
- /capabilities/CAPABILITY-13.md
- /constraints/CONSTRAINT-03.md
---

# DECISION-59: A Clef server needs no key

On a base URL that is not TypeSafe, a key is optional. jag sends a key only when the user sets one for that server, for example for a server with a key check. jag never sends a TypeSafe key to another host.

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-13: Score with a Clef server that the user runs](../capabilities/CAPABILITY-13.md)
- about: [CONSTRAINT-03: A TypeSafe key or a Clef server is enough to start](../constraints/CONSTRAINT-03.md)
<!-- LINKS:END -->
