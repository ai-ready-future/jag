---
type: Decision
title: jag does not check where the model server runs
description: jag sends text to the base URL that the user sets and does not decide
  if the server is local; the user keeps the data inside HIPAA limits, and the README
  says so.
status: stable
decided_at: '2026-10-26'
alternatives:
- jag accepts only a server on this machine as local
- jag accepts a server on this machine or on a private network as local
about:
- /capabilities/CAPABILITY-13.md
- /capabilities/CAPABILITY-09.md
---

# DECISION-60: jag does not check where the model server runs

jag sends the text of a search to the base URL that the user sets. It does not decide if that server is on this machine, on a private network, or in the cloud. The user must make sure that the server keeps the data inside the limits that apply, such as HIPAA. The README states this.

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-13: Score with a Clef server that the user runs](../capabilities/CAPABILITY-13.md)
- about: [CAPABILITY-09: Say which text leaves the machine](../capabilities/CAPABILITY-09.md)
<!-- LINKS:END -->
