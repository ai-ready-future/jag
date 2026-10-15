---
type: AcceptanceCriterion
title: rg and jag give the same bytes and exit code in a pipe
description: For each case, rg and jag with the same arguments on the same files give
  the same stdout bytes, the same stderr bytes, and the same exit code when the output
  goes to a pipe.
status: draft
item: /stories/STORY-65.md
---

# CRITERION-111: rg and jag give the same bytes and exit code in a pipe

For each case of the parity test set, `rg <args>` and `jag <args>` (no question) on the same fixture files give byte-identical stdout, byte-identical stderr, and the same exit code when the output goes to a pipe.

Source: user request to the orchestrator of 2026-10-15: "an rg command and a jag command with the same exact arguments should return exactly the same thing".

<!-- LINKS:START (generated, do not edit) -->

## Links

- item: [STORY-65: Compare rg and jag output on a parity test set](../stories/STORY-65.md)
<!-- LINKS:END -->
