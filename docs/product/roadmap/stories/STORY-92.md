---
type: Story
title: Add private patterns of the user in the config file
description: As an operator, I want to add my own private patterns in the config file,
  for example for patient IDs, so that jag also holds back the private text of my
  team.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-05T22:52:34Z'
state: backlog
github:
  number: 367
  node_id: I_kwDOVKMnL88AAAABc5GD6Q
  url: https://github.com/ai-ready-future/jag/issues/367
  pushed_hash: 6f21bb
  synced:
    state: backlog
parent: /features/FEATURE-31.md
blocked_by:
- /stories/STORY-91.md
milestone: /milestones/MILESTONE-04.md
traces:
- to: prd:/capabilities/CAPABILITY-14.md
  hash: fdaf5e
---

# STORY-92: Add private patterns of the user in the config file

As an operator (PERSONA-02), I want to add my own private patterns in the config file, for example a pattern for patient IDs, so that jag also holds back the private text of my team.

Source: DECISION-68 and DECISION-78 (stable). The planner plan of SCOPE-04, 2026-11-05, after the answer of the user through hitl ("A: now, as backlog").

## Scope

1. The user adds patterns in `~/.config/jag/config.yml`. jag checks the text against the built-in list and the list of the user (DECISION-68).
2. When a pattern of the user is not valid, jag stops before any model call with exit 2, and the message names the pattern. jag never runs a search with a check that is weaker than the user asked for (DECISION-78).
3. `jag --setup` and `jag --forget-key` keep the list of the user.

This story adds a list to the config file. ADR-0029 names the private patterns of DECISION-68 as a list, not a setting, and says that the CAPABILITY-14 spec places them. So the spec of this story decides the list key and its form. If the spec needs a change of ADR-0029 or ADR-0015, it sends a `revisit:` to the architect.

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [CRITERION-166: A pattern of the user holds back a match](../criteria/CRITERION-166.md) | - | - | - | - | - |
| [CRITERION-167: A pattern of the user that is not valid stops jag with exit 2](../criteria/CRITERION-167.md) | - | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [FEATURE-31: Hold private text back from the model](../features/FEATURE-31.md)
- blocked by: [STORY-91: Hold back each snippet that holds a key, a token or a password](STORY-91.md)
- milestone: [MILESTONE-04: Local model option](../milestones/MILESTONE-04.md)
- traces: [CAPABILITY-14: Hold private text back from the model](../../prd/capabilities/CAPABILITY-14.md)
- criterion or run of: [CRITERION-166: A pattern of the user holds back a match](../criteria/CRITERION-166.md)
- criterion or run of: [CRITERION-167: A pattern of the user that is not valid stops jag with exit 2](../criteria/CRITERION-167.md)
<!-- LINKS:END -->
