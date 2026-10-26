---
type: Question
title: Which exit code applies when the private-pattern check skips lines?
description: A search can skip lines for private patterns, leave out results, and
  reach the cost limit; each has or can have its own exit code.
status: draft
about:
- /capabilities/CAPABILITY-14.md
- /decisions/DECISION-13.md
- /decisions/DECISION-47.md
---

# QUESTION-06: Which exit code applies when the private-pattern check skips lines?

The cost-limit code and the left-out code already compete ([DECISION-47](../decisions/DECISION-47.md)). Does a search that skips lines for private patterns get its own exit code, and which code wins?

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-14: Hold private text back from the model](../capabilities/CAPABILITY-14.md)
- about: [DECISION-13: Reaching the cost limit has its own exit code](../decisions/DECISION-13.md)
- about: [DECISION-47: The cost-limit exit code wins over the left-out exit code](../decisions/DECISION-47.md)
<!-- LINKS:END -->
