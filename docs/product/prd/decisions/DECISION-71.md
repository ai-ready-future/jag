---
type: Decision
title: jag calls a server with no key when the user sets none
description: On a base URL that is not TypeSafe, jag calls with no key when the user
  sets none; a server that refuses the call for a key gives the key-not-accepted failure.
status: stable
decided_at: '2026-10-29'
alternatives:
- An empty key setting means no key
- Keep the stop before the call when no key is set
about:
- /capabilities/CAPABILITY-13.md
- /decisions/DECISION-59.md
- /decisions/DECISION-05.md
---

# DECISION-71: jag calls a server with no key when the user sets none

On a base URL that is not TypeSafe, jag calls the server with no key when the user sets no key for that server. jag does not stop before the call. When the server refuses the call because a key is missing or bad, jag fails with the key-not-accepted code of DECISION-05. The message names the setting that holds the key for that server.

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-13: Score with a Clef server that the user runs](../capabilities/CAPABILITY-13.md)
- about: [DECISION-59: A Clef server needs no key](DECISION-59.md)
- about: [DECISION-05: jag keeps rg exit codes and adds one code for each model failure](DECISION-05.md)
<!-- LINKS:END -->
