---
type: Feature
title: Install the jag skill with one README command
description: The agent builder runs one command from the README, and the jag skill
  teaches the agent the four behaviors and what to tell the user when jag fails.
status: draft
state: ready
github:
  number: 178
  node_id: I_kwDOVKMnL88AAAABYgDL6A
  url: https://github.com/ai-ready-future/jag/issues/178
  pushed_hash: 6cc628
  synced:
    state: ready
parent: /epics/EPIC-06.md
milestone: /milestones/MILESTONE-02.md
traces:
- to: prd:/capabilities/CAPABILITY-11.md
  hash: 6c96fe
- to: prd:/constraints/CONSTRAINT-08.md
  hash: 26d3ad
---

# FEATURE-24: Install the jag skill with one README command

The README gives one command that installs the jag skill into each agent that the install command supports, such as Claude Code. The README installs the program first and the skill second. After the install, the agent uses jag in place of rg, writes a broad term and a clear question in `--ask`, sets a limit on the text that jag returns, and reads the structured output and the failures. If jag is not installed, the key is not set, or the model service is down, the agent gives the user the cause and the fix that jag writes. The skill says nothing about other tools.

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [STORY-68: Install the skill with one command](../stories/STORY-68.md) | ready | - | - | - | - |
| [STORY-69: The agent uses jag in place of rg and sets a limit](../stories/STORY-69.md) | ready | - | - | - | - |
| [STORY-70: The agent reads the structured output](../stories/STORY-70.md) | ready | - | - | - | - |
| [STORY-71: The agent tells the user the cause and the fix](../stories/STORY-71.md) | ready | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [EPIC-06: An agent uses jag well](../epics/EPIC-06.md)
- milestone: [MILESTONE-02: Make it good](../milestones/MILESTONE-02.md)
- traces: [CAPABILITY-11: Agent instructions to install or copy](../../prd/capabilities/CAPABILITY-11.md)
- traces: [CONSTRAINT-08: One README command installs the agent skill](../../prd/constraints/CONSTRAINT-08.md)
- blocks: [FEATURE-25: Copy the same teaching as a snippet](FEATURE-25.md)
- blocks: [FEATURE-26: Prove that an agent with the skill uses jag well](FEATURE-26.md)
- child: [STORY-68: Install the skill with one command](../stories/STORY-68.md)
- child: [STORY-69: The agent uses jag in place of rg and sets a limit](../stories/STORY-69.md)
- child: [STORY-70: The agent reads the structured output](../stories/STORY-70.md)
- child: [STORY-71: The agent tells the user the cause and the fix](../stories/STORY-71.md)
<!-- LINKS:END -->
