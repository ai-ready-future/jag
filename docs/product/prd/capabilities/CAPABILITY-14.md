---
type: Capability
title: Hold private text back from the model
description: Before each model call, jag checks each snippet for keys, tokens, passwords,
  and the patterns of the user, and it does not send a snippet that matches.
status: draft
kano: must-be
addresses:
- /personas/PERSONA-02.md
- /personas/PERSONA-03.md
depends_on:
- /capabilities/CAPABILITY-02.md
---

# CAPABILITY-14: Hold private text back from the model

Before each model call, jag checks each snippet against a list of private patterns. The list holds built-in patterns for keys, tokens, and passwords, and the patterns that the user adds, for example patient IDs.

jag does not send a snippet that matches, and it does not score that snippet. The result lists each such line as not scored, with the reason.

The check is on for each model service: TypeSafe, an AI gateway, and a Clef server. The user can turn the check off, for example for a Clef server that the user trusts.

<!-- LINKS:START (generated, do not edit) -->

## Links

- addresses: [PERSONA-02: Operator](../personas/PERSONA-02.md)
- addresses: [PERSONA-03: Knowledge seeker](../personas/PERSONA-03.md)
- depends on: [CAPABILITY-02: Term and question search](CAPABILITY-02.md)
- constrained by: [CONSTRAINT-10: The output never shows the text of a held-back line](../constraints/CONSTRAINT-10.md)
- subject of: [DECISION-66: The private-pattern check is on by default for each model service](../decisions/DECISION-66.md)
- subject of: [DECISION-67: jag skips a snippet that matches a private pattern](../decisions/DECISION-67.md)
- subject of: [DECISION-68: The private patterns are a built-in list plus a list of the user](../decisions/DECISION-68.md)
- subject of: [DECISION-77: The private-pattern check examines all text that jag sends](../decisions/DECISION-77.md)
- subject of: [DECISION-78: A pattern of the user that is not valid stops jag with exit 2](../decisions/DECISION-78.md)
- subject of: [DECISION-79: Held-back lines add no exit code](../decisions/DECISION-79.md)
- subject of: [DECISION-80: One setting turns off the built-in list and the list of the user](../decisions/DECISION-80.md)
- subject of: [DECISION-81: The preview of the text that leaves the machine lists the held-back lines](../decisions/DECISION-81.md)
- subject of: [QUESTION-06: Which exit code applies when the private-pattern check skips lines?](../questions/QUESTION-06.md)
- subject of: [QUESTION-07: Does turning the private-pattern check off also turn off the built-in list?](../questions/QUESTION-07.md)
- included by: [SCOPE-04: After release 1: local model option](../scope/SCOPE-04.md) (must)
<!-- LINKS:END -->
