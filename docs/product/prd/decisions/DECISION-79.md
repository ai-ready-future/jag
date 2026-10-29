---
type: Decision
title: Held-back lines add no exit code
description: A search that holds lines back gets the exit code that it gets without
  them; the report counts the held-back lines, and the cost-limit code still wins.
status: stable
decided_at: '2026-10-29'
alternatives:
- A new exit code for a search that holds lines back
about:
- /questions/QUESTION-06.md
- /capabilities/CAPABILITY-14.md
- /decisions/DECISION-47.md
- /decisions/DECISION-13.md
- /decisions/DECISION-26.md
---

# DECISION-79: Held-back lines add no exit code

jag adds no exit code for lines that the private-pattern check holds back. The search exits with the code that it gets without them, for example 0 or 1. The report counts the held-back lines. The order of DECISION-47 does not change: the cost-limit code still wins.

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [QUESTION-06: Which exit code applies when the private-pattern check skips lines?](../questions/QUESTION-06.md)
- about: [CAPABILITY-14: Hold private text back from the model](../capabilities/CAPABILITY-14.md)
- about: [DECISION-47: The cost-limit exit code wins over the left-out exit code](DECISION-47.md)
- about: [DECISION-13: Reaching the cost limit has its own exit code](DECISION-13.md)
- about: [DECISION-26: Leaving out results has its own exit code](DECISION-26.md)
<!-- LINKS:END -->
