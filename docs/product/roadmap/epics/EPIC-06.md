---
type: Epic
title: An agent uses jag well
description: The agent builder gives jag to an agent with one install or one copy,
  and the agent then uses jag well with no instructions from the builder.
status: draft
state: ready
github:
  number: 177
  node_id: I_kwDOVKMnL88AAAABYgDHSw
  url: https://github.com/ai-ready-future/jag/issues/177
  pushed_hash: 20a3c2
  synced:
    state: ready
milestone: /milestones/MILESTONE-02.md
traces:
- to: prd:/capabilities/CAPABILITY-11.md
  hash: 6a8a68
- to: prd:/constraints/CONSTRAINT-08.md
  hash: 26d3ad
---

# EPIC-06: An agent uses jag well

The agent builder installs the jag skill with one README command, or copies the same teaching as a snippet. The agent then uses jag in place of rg, decides when to use `--ask`, writes a good term and question for it, sets the size limit, reads the results and the failures, and gives the user the cause and the fix when jag can't run. A repeatable test proves this behavior.

<!-- LINKS:START (generated, do not edit) -->

## Links

- milestone: [MILESTONE-02: Make it good](../milestones/MILESTONE-02.md)
- traces: [CAPABILITY-11: Agent instructions to install or copy](../../prd/capabilities/CAPABILITY-11.md)
- traces: [CONSTRAINT-08: One README command installs the agent skill](../../prd/constraints/CONSTRAINT-08.md)
- child: [FEATURE-24: Install the jag skill with one README command](../features/FEATURE-24.md)
- child: [FEATURE-25: Copy the same teaching as a snippet](../features/FEATURE-25.md)
- child: [FEATURE-26: Prove that an agent with the skill uses jag well](../features/FEATURE-26.md)
<!-- LINKS:END -->
