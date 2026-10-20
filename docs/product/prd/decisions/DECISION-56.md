---
type: Decision
title: The skill tells the agent that --ask exists; the agent decides when to use
  it
description: The skill shows jag TERM PATH (as rg, no cost) and --ask (a paid model
  call) and says that --ask exists; it does not tell the agent to always ask a question.
status: stable
decided_at: '2026-10-20'
alternatives:
- The skill tells the agent to always give a question in --ask
about:
- /capabilities/CAPABILITY-11.md
---

# DECISION-56: The skill tells the agent that --ask exists; the agent decides when to use it

Principle: just tells the agent of the existence of that option .. it is up to the agent to use it or not.

The skill shows both forms of jag: `jag TERM PATH`, which works as rg with no model call and no cost, and `--ask` with a plain-language question, which makes a paid model call. The skill tells the agent that `--ask` exists. It does not say "always", and it gives no advice on when to use `--ask` (DECISION-54). The agent decides.

Source: the user, in the hitl card "Does the skill tell the agent to always ask a question?", 2026-10-20: "just tells the agent of the existence of that option .. it is up to the agent to use it or not" (option 1). This answer changes CAPABILITY-11 behaviors 1 and 2 and CRITERION-118.

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-11: Agent instructions to install or copy](../capabilities/CAPABILITY-11.md)
<!-- LINKS:END -->
