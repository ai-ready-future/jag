---
type: Decision
title: The preview of the text that leaves the machine lists the held-back lines
description: The preview lists each held-back line with its reason; a term with no
  question makes no model call, so jag does not check it.
status: stable
decided_at: '2026-10-29'
alternatives:
- The preview shows only a count of the held-back lines
about:
- /capabilities/CAPABILITY-09.md
- /capabilities/CAPABILITY-14.md
- /decisions/DECISION-32.md
---

# DECISION-81: The preview of the text that leaves the machine lists the held-back lines

The preview of which text leaves the machine lists each line that the check holds back, with the reason. A term with no question works as plain rg ([DECISION-32](DECISION-32.md)) and makes no model call. jag does not check such a search.

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-09: Say which text leaves the machine](../capabilities/CAPABILITY-09.md)
- about: [CAPABILITY-14: Hold private text back from the model](../capabilities/CAPABILITY-14.md)
- about: [DECISION-32: A term with no question works as plain rg](DECISION-32.md)
<!-- LINKS:END -->
