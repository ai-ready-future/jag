---
type: Convention
title: On Ctrl-C, jag starts no new model call and exits with 130
description: On Ctrl-C, jag starts no new model call and exits with 130. When the reader closes the pipe, jag exits with no message.
status: draft
rule: On Ctrl-C, jag starts no new model call and exits with 130. With `--ask` or `--json`, it writes no partial result. With no `--ask` and no `--json`, the text that jag already wrote stays, as in rg (user answer, hitl, 2026-10-15, FEATURE-23 Q1 option 1). When the reader closes the pipe, jag exits with no message (conventions-4, DECISION-39).
aliases:
- CV-4
generated:
  by: claude-code/opus-5.5
  at: '2026-11-05T21:21:03Z'
---

# CONVENTION-04: On Ctrl-C, jag starts no new model call and exits with 130

On Ctrl-C, jag starts no new model call and exits with 130. With `--ask` or `--json`, it writes no partial result. With no `--ask` and no `--json`, the text that jag already wrote stays, as in rg (user answer, hitl, 2026-10-15, FEATURE-23 Q1 option 1). When the reader closes the pipe, jag exits with no message (conventions-4, DECISION-39).

<!-- LINKS:START (generated, do not edit) -->

## Links

- governs: [COMPONENT-01: Command line](../components/COMPONENT-01.md)
<!-- LINKS:END -->
