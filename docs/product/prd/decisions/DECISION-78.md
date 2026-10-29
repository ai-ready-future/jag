---
type: Decision
title: A pattern of the user that is not valid stops jag with exit 2
description: When a pattern in the list of the user is not valid, jag stops before
  any model call with exit 2 and names the pattern.
status: stable
decided_at: '2026-10-29'
alternatives:
- Show a warning, ignore the pattern, and go on
about:
- /capabilities/CAPABILITY-14.md
- /decisions/DECISION-68.md
- /decisions/DECISION-25.md
---

# DECISION-78: A pattern of the user that is not valid stops jag with exit 2

When a pattern in the list of the user is not valid, jag stops before any model call. It exits with 2, the usage error code, and the message names the pattern. jag never runs a search with a check that is weaker than the user asked for.

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-14: Hold private text back from the model](../capabilities/CAPABILITY-14.md)
- about: [DECISION-68: The private patterns are a built-in list plus a list of the user](DECISION-68.md)
- about: [DECISION-25: A size limit that is not valid is a usage error](DECISION-25.md)
<!-- LINKS:END -->
