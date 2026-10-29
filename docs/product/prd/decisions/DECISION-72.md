---
type: Decision
title: jag sends the model name that the user set, or its default
description: jag sends the model name that the user set to a Clef server, or its default
  model name when the user sets none; the server uses its own model.
status: stable
decided_at: '2026-10-29'
alternatives:
- Send no model name to a Clef server
- The user must set a model name for a Clef server
about:
- /capabilities/CAPABILITY-13.md
- /decisions/DECISION-61.md
---

# DECISION-72: jag sends the model name that the user set, or its default

jag sends the model name that the user set. When the user sets no model name, jag sends its default model name. A Clef server uses its own model, whatever the name ([DECISION-61](DECISION-61.md)).

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-13: Score with a Clef server that the user runs](../capabilities/CAPABILITY-13.md)
- about: [DECISION-61: jag does not care which Clef model the server runs](DECISION-61.md)
<!-- LINKS:END -->
