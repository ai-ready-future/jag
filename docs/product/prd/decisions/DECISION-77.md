---
type: Decision
title: The private-pattern check examines all text that jag sends
description: The check examines the question and each result with its lines around
  the match; when the question matches, jag stops before any call with a clear message.
status: stable
decided_at: '2026-10-29'
alternatives:
- Check only the matching line
- Check the results, not the question
about:
- /capabilities/CAPABILITY-14.md
- /decisions/DECISION-67.md
---

# DECISION-77: The private-pattern check examines all text that jag sends

The private-pattern check examines all text that jag sends to the model: the question, and each result with its lines around the match. A result that matches is skipped ([DECISION-67](DECISION-67.md)). When the question matches, jag stops before any model call with a clear message.

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-14: Hold private text back from the model](../capabilities/CAPABILITY-14.md)
- about: [DECISION-67: jag skips a snippet that matches a private pattern](DECISION-67.md)
<!-- LINKS:END -->
