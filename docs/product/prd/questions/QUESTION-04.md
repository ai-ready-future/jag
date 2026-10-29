---
type: Question
title: What does the readiness check report for a server with no key?
description: The readiness check examines five items, and most are about the key;
  a Clef server can have no key.
status: stable
about:
- /capabilities/CAPABILITY-13.md
- /capabilities/CAPABILITY-01.md
- /decisions/DECISION-04.md
---

# QUESTION-04: What does the readiness check report for a server with no key?

The readiness check examines five items, and most of them are about the key ([DECISION-04](../decisions/DECISION-04.md)). A Clef server can have no key and sends no bill. Which items does the check report for such a server?

**Answer (2026-10-29):** The server items: the server is reachable, and it answers a scoring call. The key items say "not needed", and the credit item says "not available". See [DECISION-75](../decisions/DECISION-75.md).

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-13: Score with a Clef server that the user runs](../capabilities/CAPABILITY-13.md)
- about: [CAPABILITY-01: Guided key setup and health check](../capabilities/CAPABILITY-01.md)
- about: [DECISION-04: The health check examines five items with one free call](../decisions/DECISION-04.md)
- subject of: [DECISION-75: The readiness check of a server with no key reports the server items](../decisions/DECISION-75.md)
<!-- LINKS:END -->
