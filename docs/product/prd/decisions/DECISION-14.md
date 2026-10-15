---
type: Decision
title: A folder search works as rg does and skips noise files
description: jag searches each subfolder and skips binary, hidden, and ignored files,
  as rg does; only --ask and --json say how many files it skipped.
status: stable
decided_at: '2026-10-15'
alternatives:
- Search every file in every subfolder and skip only binary files
- Search only the top level of the folder unless the user asks for subfolders
- Say how many files jag skipped in each mode, also with no question
about:
- /capabilities/CAPABILITY-02.md
---

# DECISION-14: A folder search works as rg does and skips noise files

jag searches each subfolder of the folder, as rg does. It skips binary files, hidden files, and files that the ignore files name, as rg does. With `--ask` or `--json`, jag says how many files it skipped. In a search with no `--ask` and no `--json`, jag writes no count of skipped files, as rg does.

Source of the change from grep -r to rg: the user, hitl answer of 2026-10-15 (FEATURE-23, Q1 option 1: rg is the reference for jag with no question). Source: the user, hitl answer of 2026-10-15 (FEATURE-23, card B option 1: with no question, jag writes no notice and has no limit that rg does not have).

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-02: Term and question search](../capabilities/CAPABILITY-02.md)
- subject of: [DECISION-36: The user can give any mix of files and folders](DECISION-36.md)
- traced by: [FEATURE-05: Search a folder as grep -r does](../../roadmap/features/FEATURE-05.md)
<!-- LINKS:END -->
