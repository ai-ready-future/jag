---
type: Decision
title: An agent skill is not an editor plugin
description: The installable jag skill is text for an agent, so it is outside the
  non-goal 'no GUI, web app, or editor plugin'; SCOPE-08 does not change.
status: stable
decided_at: '2026-10-19'
alternatives:
- Change SCOPE-08 so that it permits agent skills
- Keep the question open for a later Scope session
about:
- /scope/SCOPE-08.md
- /capabilities/CAPABILITY-11.md
---

# DECISION-53: An agent skill is not an editor plugin

jag ships a skill that an agent builder installs into an agent. The skill is instructions for the agent, not a program that runs in an editor. So the skill is outside SCOPE-08, and SCOPE-08 does not change.

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [SCOPE-08: jag has no GUI, web app, or editor plugin](../scope/SCOPE-08.md)
- about: [CAPABILITY-11: Agent instructions to install or copy](../capabilities/CAPABILITY-11.md)
<!-- LINKS:END -->
