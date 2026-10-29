---
type: Decision
title: The readiness check of a server with no key reports the server items
description: For a server with no key, the readiness check reports whether the server
  is reachable and answers a scoring call; the key items say not needed, and credit
  says not available.
status: stable
decided_at: '2026-10-29'
alternatives:
- Keep the key items, which fail with no key
- No readiness check for a Clef server
about:
- /questions/QUESTION-04.md
- /capabilities/CAPABILITY-13.md
- /decisions/DECISION-04.md
- /decisions/DECISION-48.md
---

# DECISION-75: The readiness check of a server with no key reports the server items

For a server with no key, the readiness check reports two server items: the server is reachable, and the server answers a scoring call. The key items say "not needed". The credit item says "not available". A key item that is not needed does not fail the check.

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [QUESTION-04: What does the readiness check report for a server with no key?](../questions/QUESTION-04.md)
- about: [CAPABILITY-13: Score with a Clef server that the user runs](../capabilities/CAPABILITY-13.md)
- about: [DECISION-04: The health check examines five items with one free call](DECISION-04.md)
- about: [DECISION-48: The readiness check exits 0 or with the code of the first cause](DECISION-48.md)
<!-- LINKS:END -->
