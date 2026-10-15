---
type: Decision
title: A default size limit applies with --ask or --json
description: With --ask or --json, jag applies a default size limit for an agent;
  the user can raise it or turn it off. With no question, no default limit applies.
status: stable
decided_at: '2026-10-15'
alternatives:
- No limit until the user sets one, as in grep
- A default limit only in structured output
- A default limit in each mode, also with no question
about:
- /capabilities/CAPABILITY-04.md
---

# DECISION-20: A default size limit applies with --ask or --json

With `--ask` or `--json`, when the user sets no limit, jag applies a default limit that is sized for the context of an agent. The user can raise the limit or turn it off.

In a search with no `--ask` and no `--json`, no default limit applies, as in rg. When the user gives `--max-chars N`, that limit applies, with its notice and exit code 8. jag accepts `--no-max-chars`, and it changes nothing.

Source: the user, hitl answer of 2026-10-15 (FEATURE-23, card B option 1: with no question, jag writes no notice and has no limit that rg does not have).

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-04: Limit on the text that jag returns](../capabilities/CAPABILITY-04.md)
- traced by: [FEATURE-09: Size limit on the text that jag returns](../../roadmap/features/FEATURE-09.md)
- traced by: [STORY-25: Stay inside a default size limit](../../roadmap/stories/STORY-25.md)
<!-- LINKS:END -->
