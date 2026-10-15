---
type: Decision
title: The user can give any mix of files and folders
description: jag accepts several paths, and each path can be a file or a folder, as
  in rg.
status: stable
decided_at: '2026-10-15'
alternatives:
- Release 1 accepts only one folder
about:
- /capabilities/CAPABILITY-06.md
- /decisions/DECISION-14.md
---

# DECISION-36: The user can give any mix of files and folders

The user can give several paths. Each path can be a file or a folder, as in rg, for example `src/*.py`.

Source of the change from grep to rg: the user, hitl answer of 2026-10-15 (FEATURE-23, Q1 option 1: rg is the reference for jag with no question).

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-06: Feels like grep](../capabilities/CAPABILITY-06.md)
- about: [DECISION-14: A folder search works as rg does and skips noise files](DECISION-14.md)
- traced by: [FEATURE-14: Files, folders, and piped text](../../roadmap/features/FEATURE-14.md)
- traced by: [STORY-40: Search a mix of files and folders](../../roadmap/stories/STORY-40.md)
<!-- LINKS:END -->
