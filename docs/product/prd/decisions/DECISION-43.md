---
type: Decision
title: A search with no model call reports a cost of zero and the reason
description: With no question, jag makes no model call; with --json, it reports a
  cost of zero and says why.
status: stable
decided_at: '2026-10-15'
alternatives:
- Report a cost of zero with no reason
- Report no cost
- Report a cost of zero and the reason also in text mode with no question
about:
- /capabilities/CAPABILITY-07.md
- /decisions/DECISION-32.md
---

# DECISION-43: A search with no model call reports a cost of zero and the reason

When a search makes no model call, jag reports a cost of zero and gives the reason, such as "no question": in the cost line of text mode with `--ask`, and in the cost fields of `--json`. In a search with no `--ask` and no `--json`, jag writes no cost line (DECISION-41). jag does not reuse scores in release 1 (DECISION-46).

Source: the user, hitl answer of 2026-10-15 (FEATURE-23, card B option 1: with no question, jag writes no notice and has no limit that rg does not have).

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-07: Show the cost of a search](../capabilities/CAPABILITY-07.md)
- about: [DECISION-32: A term with no question works as plain rg](DECISION-32.md)
- traced by: [FEATURE-20: Show the cost of a search](../../roadmap/features/FEATURE-20.md)
<!-- LINKS:END -->
