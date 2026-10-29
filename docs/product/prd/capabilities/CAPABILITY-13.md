---
type: Capability
title: Score with a Clef server that the user runs
description: The user sets the base URL to a Clef or Clef-Flash server, and jag scores
  each search there, with no TypeSafe key, so that no text goes to TypeSafe.
status: draft
kano: must-be
addresses:
- /personas/PERSONA-02.md
- /personas/PERSONA-03.md
depends_on:
- /capabilities/CAPABILITY-01.md
- /capabilities/CAPABILITY-02.md
---

# CAPABILITY-13: Score with a Clef server that the user runs

The user sets the base URL to the address of a Clef or Clef-Flash server that the user runs. jag then sends each scoring call of a search to that server, and to no other service.

Text that stays on a server that the user controls cannot leak credentials to an outside service. It also lets a team keep the data inside the limits that apply, such as HIPAA. The user must make sure that the server keeps the data inside these limits ([DECISION-60](../decisions/DECISION-60.md)).

The server needs no key. jag sends a key only when the user sets one.

jag does not check where the server runs, and it does not check which Clef model the server runs.

When one call is too long for the server, the search still works. When the server does not answer, jag fails clearly, as with the TypeSafe service.

<!-- LINKS:START (generated, do not edit) -->

## Links

- addresses: [PERSONA-02: Operator](../personas/PERSONA-02.md)
- addresses: [PERSONA-03: Knowledge seeker](../personas/PERSONA-03.md)
- depends on: [CAPABILITY-01: Guided key setup and health check](CAPABILITY-01.md)
- depends on: [CAPABILITY-02: Term and question search](CAPABILITY-02.md)
- subject of: [DECISION-58: The user chooses a Clef server with the base URL setting](../decisions/DECISION-58.md)
- subject of: [DECISION-59: A Clef server needs no key](../decisions/DECISION-59.md)
- subject of: [DECISION-60: jag does not check where the model server runs](../decisions/DECISION-60.md)
- subject of: [DECISION-61: jag does not care which Clef model the server runs](../decisions/DECISION-61.md)
- subject of: [DECISION-62: jag makes no quality claim for a Clef server](../decisions/DECISION-62.md)
- subject of: [DECISION-63: jag has no speed target with a Clef server](../decisions/DECISION-63.md)
- subject of: [DECISION-64: The cost of a Clef search is not available](../decisions/DECISION-64.md)
- subject of: [DECISION-65: A search still works when one call is too long for the server](../decisions/DECISION-65.md)
- subject of: [DECISION-69: The release after release 1 serves users whose text must stay on the machine](../decisions/DECISION-69.md)
- subject of: [DECISION-71: jag calls a server with no key when the user sets none](../decisions/DECISION-71.md)
- subject of: [DECISION-72: jag sends the model name that the user set, or its default](../decisions/DECISION-72.md)
- subject of: [DECISION-73: An error or an answer that jag cannot read gives the service-down failure](../decisions/DECISION-73.md)
- subject of: [DECISION-74: The report of the text that leaves the machine names the host](../decisions/DECISION-74.md)
- subject of: [DECISION-75: The readiness check of a server with no key reports the server items](../decisions/DECISION-75.md)
- subject of: [DECISION-76: jag skips a snippet that is too long for the server by itself](../decisions/DECISION-76.md)
- subject of: [QUESTION-04: What does the readiness check report for a server with no key?](../questions/QUESTION-04.md)
- subject of: [QUESTION-05: What happens when one snippet alone is too long for the server?](../questions/QUESTION-05.md)
- included by: [SCOPE-04: After release 1: local model option](../scope/SCOPE-04.md) (must)
<!-- LINKS:END -->
