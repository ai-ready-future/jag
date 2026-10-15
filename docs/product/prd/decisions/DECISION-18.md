---
type: Decision
title: Piped text has the label <stdin>
description: A result from piped text uses the rg label <stdin> and the line numbers
  of the piped text.
status: stable
decided_at: '2026-10-15'
alternatives:
- Show no file name, only the line number
- Do not accept piped text for a question search
- The grep label (standard input)
about:
- /capabilities/CAPABILITY-03.md
---

# DECISION-18: Piped text has the label <stdin>

Text from a pipe has no file. A result from that text uses the same label as rg, `<stdin>`, and it gives the line numbers in the piped text.

Source of the change from grep to rg: the user, hitl answer of 2026-10-15 (FEATURE-23, Q1 option 1: rg is the reference for jag with no question). Before this answer, the label was the grep label `(standard input)`.

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-03: File and line for each result](../capabilities/CAPABILITY-03.md)
- subject of: [DECISION-35: With no path, jag searches the current folder or reads the pipe](DECISION-35.md)
- traced by: [FEATURE-08: File, line, and snippet for each result](../../roadmap/features/FEATURE-08.md)
- traced by: [STORY-24: Label piped text](../../roadmap/stories/STORY-24.md)
<!-- LINKS:END -->
