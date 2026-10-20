---
type: Capability
title: Agent instructions to install or copy
description: The agent builder installs a jag skill into their agent, or copies a
  snippet; either one teaches the agent when and how to call jag.
status: stable
kano: delighter
addresses:
- /journeys/JOURNEY-01.md
- /needs/NEED-01.md
- /needs/NEED-02.md
- /needs/NEED-13.md
depends_on:
- /capabilities/CAPABILITY-01.md
- /capabilities/CAPABILITY-02.md
- /capabilities/CAPABILITY-04.md
- /capabilities/CAPABILITY-05.md
---

# CAPABILITY-11: Agent instructions to install or copy

jag ships one set of agent instructions in two forms. The agent builder installs the skill into each agent that the install command supports, such as Claude Code. For an agent that can't install a skill, the agent builder copies the snippet into the instructions of the agent. Both forms teach the same behavior.

After the install, the agent does these things with no instructions from the agent builder:

1. The agent chooses jag when plain keyword search returns too much text, and it uses plain keyword search when that is enough.
2. The agent gives jag a broad term and a clear plain-language question.
3. The agent sets a limit on the text that jag returns.
4. The agent reads the structured output and opens the file and the line of each result. It acts correctly when jag reports no answer, text that it left out, or a service failure.

If jag is not installed, the key is not set, or the model service is down, the agent tells the user what is wrong and how to fix it. Then the agent continues with the search tool that it used before, such as grep or rg.

<!-- LINKS:START (generated, do not edit) -->

## Links

- addresses: [JOURNEY-01: Agent builder fixes a wrong answer](../journeys/JOURNEY-01.md)
- addresses: [NEED-01: Give the agent a search tool for a folder of text](../needs/NEED-01.md)
- addresses: [NEED-02: Search output fills the agent context with noise](../needs/NEED-02.md)
- addresses: [NEED-13: Search prompt tuning fixes one case and breaks another](../needs/NEED-13.md)
- depends on: [CAPABILITY-01: Guided key setup and health check](CAPABILITY-01.md)
- depends on: [CAPABILITY-02: Term and question search](CAPABILITY-02.md)
- depends on: [CAPABILITY-04: Limit on the text that jag returns](CAPABILITY-04.md)
- depends on: [CAPABILITY-05: Text output for people and structured output for agents](CAPABILITY-05.md)
- constrained by: [CONSTRAINT-08: One README command installs the agent skill](../constraints/CONSTRAINT-08.md)
- subject of: [DECISION-53: An agent skill is not an editor plugin](../decisions/DECISION-53.md)
- included by: [SCOPE-01: Release 1: first public release](../scope/SCOPE-01.md) (should)
- traced by: [EPIC-06: An agent uses jag well](../../roadmap/epics/EPIC-06.md)
- traced by: [FEATURE-24: Install the jag skill with one README command](../../roadmap/features/FEATURE-24.md)
- traced by: [FEATURE-25: Copy the same teaching as a snippet](../../roadmap/features/FEATURE-25.md)
- traced by: [FEATURE-26: Prove that an agent with the skill uses jag well](../../roadmap/features/FEATURE-26.md)
- traced by: [MILESTONE-02: Make it good](../../roadmap/milestones/MILESTONE-02.md)
<!-- LINKS:END -->
