---
type: Decision
title: The release after release 1 serves users whose text must stay on the machine
description: The local model option moves to the release after release 1, before release
  2, so users whose text must not leave the machine can use jag with a Clef server.
status: stable
decided_at: '2026-10-26'
alternatives:
- Keep the local model option after release 3
- Ship the local model option with release 2
about:
- /questions/QUESTION-01.md
- /scope/SCOPE-04.md
- /capabilities/CAPABILITY-13.md
supersedes: /decisions/DECISION-02.md
---

# DECISION-57: The release after release 1 serves users whose text must stay on the machine

The local model option ships in the release after release 1, before release 2 and release 3. Users whose text must not leave the machine use jag with a Clef server that they run. This decision replaces DECISION-02.

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [QUESTION-01: Does jag serve users whose text must not leave the machine?](../questions/QUESTION-01.md)
- about: [SCOPE-04: After release 1: local model option](../scope/SCOPE-04.md)
- about: [CAPABILITY-13: Score with a Clef server that the user runs](../capabilities/CAPABILITY-13.md)
- supersedes: [DECISION-02: Release 1 does not serve users whose text must stay on the machine](DECISION-02.md)
<!-- LINKS:END -->
