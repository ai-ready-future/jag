---
type: Decision
title: An error or an answer that jag cannot read gives the service-down failure
description: When the server sends an error or an answer that jag cannot read, jag
  fails with the service-down code; the message gives the base URL and the status.
status: stable
decided_at: '2026-10-29'
alternatives:
- A new exit code for an answer that jag cannot read
- Try again before jag fails
about:
- /capabilities/CAPABILITY-13.md
- /decisions/DECISION-05.md
- /decisions/DECISION-01.md
---

# DECISION-73: An error or an answer that jag cannot read gives the service-down failure

The server can answer with an error, or with an answer that jag cannot read, for example when the base URL is not a Clef server. jag then fails with the service-down code of DECISION-05. The message gives the base URL and the status that the server sent.

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-13: Score with a Clef server that the user runs](../capabilities/CAPABILITY-13.md)
- about: [DECISION-05: jag keeps rg exit codes and adds one code for each model failure](DECISION-05.md)
- about: [DECISION-01: jag fails clearly when the model is not available](DECISION-01.md)
<!-- LINKS:END -->
