---
type: Decision
title: With --ask in text mode, the cost line goes to the error stream after each
  search
description: With --ask, text mode writes a cost line to the error stream after each
  search, also in a pipe; structured mode has a cost field; with no question, text
  mode writes no cost line.
status: stable
decided_at: '2026-10-15'
alternatives:
- Show the cost line only in a terminal, or when the user gives an option
- Show the cost only when the user gives an option, in both modes
- Write the cost line after each search in text mode, also with no question
about:
- /capabilities/CAPABILITY-07.md
- /decisions/DECISION-29.md
---

# DECISION-41: With --ask in text mode, the cost line goes to the error stream after each search

In text mode with `--ask`, jag writes a cost line to the error stream after each search, also when the output goes to a pipe. In structured mode, the cost is always a field. In a search with no `--ask` and no `--json`, jag writes no cost line, as rg does.

Source: the user, hitl answer of 2026-10-15 (FEATURE-23, card B option 1: with no question, jag writes no notice and has no limit that rg does not have).

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-07: Show the cost of a search](../capabilities/CAPABILITY-07.md)
- about: [DECISION-29: In text mode, notices go to the error stream](DECISION-29.md)
- traced by: [FEATURE-20: Show the cost of a search](../../roadmap/features/FEATURE-20.md)
<!-- LINKS:END -->
