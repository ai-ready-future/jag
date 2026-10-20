---
type: Feature
title: Prove that an agent with the skill uses jag well
description: A repeatable test runs an agent with the jag skill and shows that it
  uses jag in place of rg, sets the limit, reads the results, and gives the user the
  cause and the fix when jag fails.
status: draft
state: ready
github:
  number: 180
  node_id: I_kwDOVKMnL88AAAABYgDWGw
  url: https://github.com/ai-ready-future/jag/issues/180
  pushed_hash: 6afd36
  synced:
    state: ready
parent: /epics/EPIC-06.md
blocked_by:
- /features/FEATURE-24.md
milestone: /milestones/MILESTONE-02.md
traces:
- to: prd:/capabilities/CAPABILITY-11.md
  hash: 6a8a68
---

# FEATURE-26: Prove that an agent with the skill uses jag well

The maintainer runs one repeatable test. The test gives an agent with the jag skill a set of tasks. For each task, it records whether the agent ran jag in place of rg, whether it set a limit on the text, whether it used the file and line of a result, and whether it gave the user the cause and the fix that jag writes when jag could not run.

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [STORY-74: Run one repeatable agent test](../stories/STORY-74.md) | ready | - | - | - | - |
| [STORY-75: The test records what the agent did](../stories/STORY-75.md) | ready | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [EPIC-06: An agent uses jag well](../epics/EPIC-06.md)
- blocked by: [FEATURE-24: Install the jag skill with one README command](FEATURE-24.md)
- milestone: [MILESTONE-02: Make it good](../milestones/MILESTONE-02.md)
- traces: [CAPABILITY-11: Agent instructions to install or copy](../../prd/capabilities/CAPABILITY-11.md)
- child: [STORY-74: Run one repeatable agent test](../stories/STORY-74.md)
- child: [STORY-75: The test records what the agent did](../stories/STORY-75.md)
<!-- LINKS:END -->
