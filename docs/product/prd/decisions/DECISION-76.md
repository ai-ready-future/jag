---
type: Decision
title: jag skips a snippet that is too long for the server by itself
description: When one snippet alone is too long for the server, jag does not score
  it and lists it as not scored, with the reason too long for the server.
status: stable
decided_at: '2026-10-29'
alternatives:
- Stop the search with an error that names the line
- Cut the snippet and score the rest
about:
- /questions/QUESTION-05.md
- /capabilities/CAPABILITY-13.md
- /decisions/DECISION-65.md
---

# DECISION-76: jag skips a snippet that is too long for the server by itself

When one snippet alone is too long for the server, jag does not score it. The result lists the line as not scored, with the reason "too long for the server". jag never cuts the snippet ([DECISION-65](DECISION-65.md)). The search goes on with the other snippets.

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [QUESTION-05: What happens when one snippet alone is too long for the server?](../questions/QUESTION-05.md)
- about: [CAPABILITY-13: Score with a Clef server that the user runs](../capabilities/CAPABILITY-13.md)
- about: [DECISION-65: A search still works when one call is too long for the server](DECISION-65.md)
<!-- LINKS:END -->
