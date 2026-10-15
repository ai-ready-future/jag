---
type: Feature
title: Give the same output as rg for the same arguments
description: An rg command and a jag command with the same arguments and no question
  give the same bytes, the same color, and the same exit code, and e2e tests check
  this in CI.
status: draft
state: backlog
github:
  number: 113
  node_id: I_kwDOVKMnL88AAAABXgIsWw
  url: https://github.com/ai-ready-future/jag/issues/113
  pushed_hash: 922c26
  synced:
    state: backlog
parent: /epics/EPIC-04.md
milestone: /milestones/MILESTONE-02.md
traces:
- to: prd:/capabilities/CAPABILITY-06.md
  hash: 79d5d3
- to: prd:/personas/PERSONA-04.md
  hash: 02412c
---

# FEATURE-23: Give the same output as rg for the same arguments

An rg command and a jag command with the same arguments and no question give the same stdout, the same stderr, the same color, and the same exit code. The team uses this parity as a test strategy: the parity cases run again as e2e tests in CI. Each difference is fixed in jag, or a user ANSWER records it as a deliberate difference.

Source: user request to the orchestrator of 2026-10-15. The user said, word for word: "lets also add a story or feature to ensure and verify parity with rg .. the idea being that an rg command and a jag command with the same exact arguments should return exactly the same thing . including the coloring of the output lets use this as a testing strategy .. then assign the work to the new rolesin the team that can absorb this work, run it and codify it so that it can be executed again in the future as an e2e test ... plan this work in the plan as a story or add to a story we may already have about feature parity and behvaior parity then lets have the team work on it when possible".

User answer (hitl, 2026-10-15): Q1 option 1, rg is the reference for jag with no question. Q2 option 1, the parity comes before release 1, so FEATURE-18 is blocked_by FEATURE-23. DECISION-18, DECISION-32, DECISION-34, and DECISION-38 now name rg in place of grep. STORY-67 changes jag to match rg for each difference: the color codes, the file heading with line numbers in a terminal, `<stdin>` in place of `(standard input)`, and the program name at the start of each error message. The spec decides the options that rg does not have (`--include` and `--exclude`, DECISION-34).

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [STORY-65: Compare rg and jag output on a parity test set](../stories/STORY-65.md) | backlog | - | - | - | - |
| [STORY-66: Run the rg parity cases as e2e tests in CI](../stories/STORY-66.md) | backlog | - | - | - | - |
| [STORY-67: Fix each rg parity difference or record it as deliberate](../stories/STORY-67.md) | backlog | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [EPIC-04: A shell user keeps their habits](../epics/EPIC-04.md)
- milestone: [MILESTONE-02: Make it good](../milestones/MILESTONE-02.md)
- traces: [CAPABILITY-06: Feels like grep](../../prd/capabilities/CAPABILITY-06.md)
- traces: [PERSONA-04: Shell power user](../../prd/personas/PERSONA-04.md)
- blocks: [FEATURE-18: Install jag from GitHub Releases, crates.io, or npm](FEATURE-18.md)
- child: [STORY-65: Compare rg and jag output on a parity test set](../stories/STORY-65.md)
- child: [STORY-66: Run the rg parity cases as e2e tests in CI](../stories/STORY-66.md)
- child: [STORY-67: Fix each rg parity difference or record it as deliberate](../stories/STORY-67.md)
<!-- LINKS:END -->
