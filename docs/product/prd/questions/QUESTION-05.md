---
type: Question
title: What happens when one snippet alone is too long for the server?
description: jag sends smaller calls when a call is too long, but one snippet can
  be too long by itself.
status: stable
about:
- /capabilities/CAPABILITY-13.md
---

# QUESTION-05: What happens when one snippet alone is too long for the server?

When a call is too long for a Clef server, jag sends smaller calls. One snippet can still be too long by itself. Does jag skip that snippet, cut it, or fail the search?

**Answer (2026-10-29):** jag skips it. The result lists the line as not scored, with the reason "too long for the server". See [DECISION-76](../decisions/DECISION-76.md).

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-13: Score with a Clef server that the user runs](../capabilities/CAPABILITY-13.md)
- subject of: [DECISION-76: jag skips a snippet that is too long for the server by itself](../decisions/DECISION-76.md)
<!-- LINKS:END -->
