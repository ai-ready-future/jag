---
type: Decision
title: After jag fails, the skill tells the agent to tell the user, with no fallback
  rule
description: When jag cannot run, the skill tells the agent to give the user the cause
  and the fix that jag writes; it says nothing about other tools.
status: stable
decided_at: '2026-10-20'
alternatives:
- Tell the user, then continue with rg or grep
about:
- /capabilities/CAPABILITY-11.md
---

# DECISION-55: After jag fails, the skill tells the agent to tell the user, with no fallback rule

Principle: Tell the user, no fallback rule: the skill tells the agent to give the user the cause and the fix that jag writes. It says nothing about other tools, and the agent then does what it does by default.

The requirement that the agent continues with grep or rg after jag fails goes away. It was a rule on when to use which tool, and DECISION-54 says that the skill gives no such rule.

Source: the user, in the hitl card "What the agent does after jag fails" (planner, Ask the Rule), 2026-10-20: option 1. This answer changes the fallback paragraph of CAPABILITY-11 and CRITERION-123.

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-11: Agent instructions to install or copy](../capabilities/CAPABILITY-11.md)
<!-- LINKS:END -->
