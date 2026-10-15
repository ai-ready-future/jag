---
type: Decision
title: Color works as rg --color=auto
description: jag colors the output only in a terminal, with the rg color codes, and
  --color=never or always changes that.
status: stable
decided_at: '2026-10-15'
alternatives:
- No color in release 1
- Color works as grep --color=auto, with the grep color codes
about:
- /capabilities/CAPABILITY-06.md
---

# DECISION-38: Color works as rg --color=auto

jag colors the output only when the output is a terminal, as `rg --color=auto` does: the path, the line number, and the matching text, with the rg color codes. The `--color=never` and `--color=always` options change this.

Source of the change from grep to rg: the user, hitl answer of 2026-10-15 (FEATURE-23, Q1 option 1: rg is the reference for jag with no question).

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-06: Feels like grep](../capabilities/CAPABILITY-06.md)
- traced by: [FEATURE-15: Good behavior in a terminal and in a pipe](../../roadmap/features/FEATURE-15.md)
- traced by: [STORY-42: Color the matches as grep does](../../roadmap/stories/STORY-42.md)
<!-- LINKS:END -->
