---
type: Decision
title: The term is a regular expression, and -F makes it fixed text
description: The term is a regular expression, as in rg; the -F option makes it fixed
  text.
status: stable
decided_at: '2026-10-15'
alternatives:
- The term is always fixed text
- Fixed text by default, and an option makes it a regular expression
- A regular expression as in grep
about:
- /capabilities/CAPABILITY-06.md
- /capabilities/CAPABILITY-02.md
---

# DECISION-33: The term is a regular expression, and -F makes it fixed text

The term is a regular expression, as in rg: it uses the pattern rules of rg. The `-F` option makes the term fixed text.

Source of the change from grep to rg: the user, hitl answer of 2026-10-15 (FEATURE-23, Q1 option 1: rg is the reference for jag with no question).

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-06: Feels like grep](../capabilities/CAPABILITY-06.md)
- about: [CAPABILITY-02: Term and question search](../capabilities/CAPABILITY-02.md)
- traced by: [FEATURE-04: Rank matching lines by a question](../../roadmap/features/FEATURE-04.md)
- traced by: [FEATURE-13: Common rg options and exit codes](../../roadmap/features/FEATURE-13.md)
- traced by: [STORY-13: Rank matching lines by a question](../../roadmap/stories/STORY-13.md)
- traced by: [STORY-35: Use the common rg options](../../roadmap/stories/STORY-35.md)
<!-- LINKS:END -->
