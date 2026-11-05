---
type: Convention
title: Each failure gives its cause, its fix, and its exit code
description: Each failure gives its cause, its fix, and its exit code of IF-4, in the error stream or in the JSON document.
status: draft
rule: 'Each failure gives its cause, its fix, and its exit code of IF-4. In text mode, the message goes to the error stream. In structured mode, the error is a JSON document on standard output (conventions-1, DECISION-01, DECISION-05, DECISION-07). The one exception is a failure before jag starts: the npm launcher writes one line with the cause and the fix to the error stream, also with `--json`, and exits with 126 or 127 (IF-4). It writes no document, and IF-5 says that such an output means that jag stopped. In a search with no `--ask` and no `--json`, jag gives the error text of rg, which has no fix, with the name `jag` in place of `rg` (user answer, hitl, 2026-10-15, FEATURE-23 Q1 option 1 and card A option 1). jag leaves out each rg hint sentence that names an option that jag does not have; the first part of the text and the exit code are the same as in rg. For an unknown option, jag suggests similar options from its own list, with the rule of rg. These are recorded differences (card
  E option 1).'
aliases:
- CV-1
generated:
  by: claude-code/opus-5.5
  at: '2026-11-05T21:21:02Z'
---

# CONVENTION-01: Each failure gives its cause, its fix, and its exit code

Each failure gives its cause, its fix, and its exit code of IF-4. In text mode, the message goes to the error stream. In structured mode, the error is a JSON document on standard output (conventions-1, DECISION-01, DECISION-05, DECISION-07). The one exception is a failure before jag starts: the npm launcher writes one line with the cause and the fix to the error stream, also with `--json`, and exits with 126 or 127 (IF-4). It writes no document, and IF-5 says that such an output means that jag stopped. In a search with no `--ask` and no `--json`, jag gives the error text of rg, which has no fix, with the name `jag` in place of `rg` (user answer, hitl, 2026-10-15, FEATURE-23 Q1 option 1 and card A option 1). jag leaves out each rg hint sentence that names an option that jag does not have; the first part of the text and the exit code are the same as in rg. For an unknown option, jag suggests similar options from its own list, with the rule of rg. These are recorded differences (card E option 1).

<!-- LINKS:START (generated, do not edit) -->

## Links

- governs: [COMPONENT-01: Command line](../components/COMPONENT-01.md)
- governs: [COMPONENT-06: Output](../components/COMPONENT-06.md)
- governs: [COMPONENT-07: Readiness check](../components/COMPONENT-07.md)
<!-- LINKS:END -->
