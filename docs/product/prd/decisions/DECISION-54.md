---
type: Decision
title: The skill teaches jag as an enhanced rg, with no advice on when to use which
  tool
description: The jag skill tells the agent to use jag in place of rg and shows how
  jag and --ask work; it gives no advice on when to use which tool.
status: stable
decided_at: '2026-10-20'
alternatives:
- The skill tells the agent when to choose jag and when plain keyword search is enough
about:
- /capabilities/CAPABILITY-11.md
- /constraints/CONSTRAINT-08.md
---

# DECISION-54: The skill teaches jag as an enhanced rg, with no advice on when to use which tool

Principle: skill instructs the agents to use jag instead of rg, and it also shows how to use jag effectively and the ask functionality built into jag; skill should not mention when to use what; it should just let the agent know how jag works and how it can be used as an enhanced rg.

The jag skill tells the agent to use jag in place of rg. It shows how to use jag well, with a term and a plain-language question in `--ask`. It gives no advice on when to use jag and when to use another tool. The README installs the program first, and the skill second (`npx skills add ai-ready-future/jag`).

Source: the user, in the hitl cards on ADR-0025, 2026-10-20. Card 2 (b): "skill instructs the agents to use jag instead of rg .. and it also shows how to use jag effectively and us the ask functionality built into jag etc... skill should not mention when to use what. . should just let the agent know how jag works and how it can be used as an enhanced rg". Card 2 (a): "Program first, then skill". This answer changes CAPABILITY-11 behavior 1, CRITERION-117, and CONSTRAINT-08.

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-11: Agent instructions to install or copy](../capabilities/CAPABILITY-11.md)
- about: [CONSTRAINT-08: One README command installs the agent skill](../constraints/CONSTRAINT-08.md)
<!-- LINKS:END -->
