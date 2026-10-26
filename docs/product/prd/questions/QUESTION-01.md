---
type: Question
title: Does jag serve users whose text must not leave the machine?
description: jag sends text to an outside model service; some operators and knowledge
  seekers cannot send their text out.
status: stable
about:
- /constraints/CONSTRAINT-03.md
- /personas/PERSONA-02.md
- /personas/PERSONA-03.md
---

# QUESTION-01: Does jag serve users whose text must not leave the machine?

jag sends each candidate snippet to an outside model service to score it. Some users cannot send their text off the machine. The Scope lens must decide whether jag serves these users, and how.

**Answer (2026-10-06):** Not in release 1. [DECISION-02](../decisions/DECISION-02.md) records the limit. A later local model option can serve these users.

**Answer (2026-10-26):** Yes, in the release after release 1. [DECISION-57](../decisions/DECISION-57.md) replaces DECISION-02. The user scores with a Clef server that the user runs ([CAPABILITY-13](../capabilities/CAPABILITY-13.md)).

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CONSTRAINT-03: A TypeSafe key or a Clef server is enough to start](../constraints/CONSTRAINT-03.md)
- about: [PERSONA-02: Operator](../personas/PERSONA-02.md)
- about: [PERSONA-03: Knowledge seeker](../personas/PERSONA-03.md)
- subject of: [DECISION-02: Release 1 does not serve users whose text must stay on the machine](../decisions/DECISION-02.md)
- subject of: [DECISION-57: The release after release 1 serves users whose text must stay on the machine](../decisions/DECISION-57.md)
<!-- LINKS:END -->
