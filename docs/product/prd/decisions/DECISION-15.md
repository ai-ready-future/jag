---
type: Decision
title: A result path has the same form as in rg
description: The path of each result follows the form of the folder that the user
  gave.
status: stable
decided_at: '2026-10-15'
alternatives:
- Always give absolute paths
- Always give paths relative to the folder that the user searched
- The same form as in grep
about:
- /capabilities/CAPABILITY-03.md
---

# DECISION-15: A result path has the same form as in rg

The path of each result has the same form as in rg. A relative folder gives relative paths, and an absolute folder gives absolute paths.

Source of the change from grep to rg: the user, hitl answer of 2026-10-15 (FEATURE-23, Q1 option 1: rg is the reference for jag with no question).

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-03: File and line for each result](../capabilities/CAPABILITY-03.md)
- traced by: [FEATURE-08: File, line, and snippet for each result](../../roadmap/features/FEATURE-08.md)
- traced by: [STORY-21: See the file, the line, and the snippet of a result](../../roadmap/stories/STORY-21.md)
<!-- LINKS:END -->
