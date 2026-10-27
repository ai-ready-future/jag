---
type: Story
title: Score jag and grep on a labeled set from a local path
description: As an agent builder (PERSONA-01), I want the Rust jag to score jag and
  grep on a local labeled set, so that the held-out run needs no private data in the
  repository.
status: draft
state: done
actual_end_date: '2026-10-27'
github:
  number: 258
  node_id: I_kwDOVKMnL88AAAABaV0L_g
  url: https://github.com/ai-ready-future/jag/issues/258
  pushed_hash: 04bbcc
  synced:
    state: ready
spec: docs/superpowers/specs/2026-10-27-STORY-78-score-jag-and-grep-on-a-local-labeled-set-design.md
parent: /epics/EPIC-02.md
milestone: /milestones/MILESTONE-02.md
traces:
- to: prd:/metrics/METRIC-01.md
  hash: bc4856
---

# STORY-78: Score jag and grep on a labeled set from a local path

As an agent builder (PERSONA-01), I want the Rust jag to score jag and blind grep on a labeled set that stays on the maintainer's machine, so that the held-out evaluation can run with no private data in the repository.

The Rust jag has no run tool for the held-out evaluation. This story adds one, as part of the Rust jag (DECISION-70). The code of the run and its invented test cases can be public. The data path, the key, and the per-case results stay on the maintainer's machine. STORY-45 uses this run tool.

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [CRITERION-140: The run reports ndcg@10 for jag and grep and no case text](../criteria/CRITERION-140.md) | - | - | - | - | - |
| [CRITERION-141: The repository holds only invented test cases for the run](../criteria/CRITERION-141.md) | - | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [EPIC-02: Ranked search with file and line](../epics/EPIC-02.md)
- milestone: [MILESTONE-02: Make it good](../milestones/MILESTONE-02.md)
- traces: [METRIC-01: Answer quality](../../prd/metrics/METRIC-01.md)
- criterion or run of: [CRITERION-140: The run reports ndcg@10 for jag and grep and no case text](../criteria/CRITERION-140.md)
- criterion or run of: [CRITERION-141: The repository holds only invented test cases for the run](../criteria/CRITERION-141.md)
- blocks: [STORY-45: Run the held-out evaluation before release](STORY-45.md)
<!-- LINKS:END -->
