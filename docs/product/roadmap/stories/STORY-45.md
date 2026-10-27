---
type: Story
title: Run the held-out evaluation before release
description: As an agent builder (PERSONA-01), I want the maintainer to run the Rust
  jag once on the private held-out split before release 1, so that I can trust that
  jag answers better than grep.
status: draft
state: backlog
github:
  number: 70
  node_id: I_kwDOVKMnL88AAAABXTMnlA
  url: https://github.com/ai-ready-future/jag/issues/70
  synced:
    state: backlog
parent: /epics/EPIC-02.md
blocked_by:
- /stories/STORY-78.md
milestone: /milestones/MILESTONE-02.md
traces:
- to: prd:/scope/SCOPE-10.md
  hash: 1a7d4e
- to: prd:/metrics/METRIC-01.md
  hash: bc4856
---

# STORY-45: Run the held-out evaluation before release

As an agent builder (PERSONA-01), I want the maintainer to run the Rust jag once on the private held-out split before release 1, so that I can trust that jag answers better than grep.

The run follows DECISION-70:

- The held-out set stays on the maintainer's machine. The maintainer runs the Rust jag one time on the held-out split, with a session to guide the run. No CI job runs it.
- The prompt and the 0.70 threshold are frozen before the run. The held-out split is not used to choose anything, and nothing is tuned on the result.
- Only a summary of the numbers goes public: the release notes and one README line. The data path, the key, and the per-case results stay on the maintainer's machine.
- If the score is below 0.70, or not higher than blind grep, there is no release tag. A fix story comes first.
- The run uses the run tool of STORY-78.

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [CRITERION-73: The held-out score meets the target and beats grep](../criteria/CRITERION-73.md) | - | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [EPIC-02: Ranked search with file and line](../epics/EPIC-02.md)
- blocked by: [STORY-78: Score jag and grep on a labeled set from a local path](STORY-78.md)
- milestone: [MILESTONE-02: Make it good](../milestones/MILESTONE-02.md)
- traces: [SCOPE-10: The held-out score confirms the gain over grep](../../prd/scope/SCOPE-10.md)
- traces: [METRIC-01: Answer quality](../../prd/metrics/METRIC-01.md)
- criterion or run of: [CRITERION-73: The held-out score meets the target and beats grep](../criteria/CRITERION-73.md)
- blocks: [STORY-51: Publish each channel from one tag](STORY-51.md)
<!-- LINKS:END -->
