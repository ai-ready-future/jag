---
type: Capability
title: Say which text leaves the machine
description: jag tells the user which text it sends to the model service, so the user
  can decide before a search.
status: stable
kano: performance
addresses:
- /personas/PERSONA-02.md
- /personas/PERSONA-03.md
depends_on:
- /capabilities/CAPABILITY-02.md
---

# CAPABILITY-09: Say which text leaves the machine

jag tells the user which text it sends to the model service. The user can see this before a search and decide whether to run it.

<!-- LINKS:START (generated, do not edit) -->

## Links

- addresses: [PERSONA-02: Operator](../personas/PERSONA-02.md)
- addresses: [PERSONA-03: Knowledge seeker](../personas/PERSONA-03.md)
- depends on: [CAPABILITY-02: Term and question search](CAPABILITY-02.md)
- constrained by: [CONSTRAINT-10: The output never shows the text of a held-back line](../constraints/CONSTRAINT-10.md)
- subject of: [DECISION-60: jag does not check where the model server runs](../decisions/DECISION-60.md)
- subject of: [DECISION-74: The report of the text that leaves the machine names the host](../decisions/DECISION-74.md)
- subject of: [DECISION-81: The preview of the text that leaves the machine lists the held-back lines](../decisions/DECISION-81.md)
- included by: [SCOPE-01: Release 1: first public release](../scope/SCOPE-01.md) (could)
- included by: [SCOPE-02: Release 2: trust and cost](../scope/SCOPE-02.md) (must)
<!-- LINKS:END -->
