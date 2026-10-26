---
type: Decision
title: The private-pattern check is on by default for each model service
description: jag checks snippets for private patterns before each call to TypeSafe,
  an AI gateway, or a Clef server; only the user can turn the check off.
status: stable
decided_at: '2026-10-26'
alternatives:
- Off by default, and the user can turn it on
- Always on, with no setting to turn it off
- No check
about:
- /capabilities/CAPABILITY-14.md
---

# DECISION-66: The private-pattern check is on by default for each model service

The private-pattern check is on by default for each model service. jag does not turn the check off by itself, because it does not decide if a server is local. The user can turn the check off, for example for a Clef server that the user trusts.

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-14: Hold private text back from the model](../capabilities/CAPABILITY-14.md)
<!-- LINKS:END -->
