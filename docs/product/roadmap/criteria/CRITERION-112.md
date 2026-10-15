---
type: AcceptanceCriterion
title: rg and jag give the same bytes and color in a terminal
description: For each case, rg and jag give the same stdout bytes with the color codes,
  the same stderr bytes, and the same exit code with --color=always and in a terminal.
status: draft
item: /stories/STORY-65.md
---

# CRITERION-112: rg and jag give the same bytes and color in a terminal

For each case of the parity test set, `rg <args>` and `jag <args>` (no question) give byte-identical stdout with the color codes, byte-identical stderr, and the same exit code in two modes: with `--color=always` in a pipe, and in a terminal (a pseudo terminal).

Source: user request to the orchestrator of 2026-10-15: "including the coloring of the output".

<!-- LINKS:START (generated, do not edit) -->

## Links

- item: [STORY-65: Compare rg and jag output on a parity test set](../stories/STORY-65.md)
<!-- LINKS:END -->
