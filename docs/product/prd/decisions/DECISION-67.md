---
type: Decision
title: jag skips a snippet that matches a private pattern
description: jag does not send or score a snippet that matches a private pattern,
  and the result lists the line as not scored, with the reason.
status: stable
decided_at: '2026-10-26'
alternatives:
- Hide the matched text and still score the rest
- Stop the search with a clear message and its own exit code
about:
- /capabilities/CAPABILITY-14.md
---

# DECISION-67: jag skips a snippet that matches a private pattern

jag does not send a snippet that matches a private pattern, and it does not score it. The result lists the line as not scored and gives the reason. The search goes on with the other snippets.

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-14: Hold private text back from the model](../capabilities/CAPABILITY-14.md)
- subject of: [DECISION-77: The private-pattern check examines all text that jag sends](DECISION-77.md)
<!-- LINKS:END -->
