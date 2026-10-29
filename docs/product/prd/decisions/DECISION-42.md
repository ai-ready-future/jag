---
type: Decision
title: The report counts results scored and results skipped
description: jag shows the results it scored and one count for each reason that it
  did not score a result; together they are all results found.
status: stable
decided_at: '2026-10-29'
alternatives:
- Show only the count of results scored
- One count of all results not scored
about:
- /capabilities/CAPABILITY-07.md
- /decisions/DECISION-12.md
- /decisions/DECISION-13.md
---

# DECISION-42: The report counts results scored and results skipped

The report counts results. It shows the results that jag scored, and one count for each reason that jag did not score a result. The reasons are the cost limit, a private pattern, and a snippet that is too long for the server. All the counts add up to all the results that the term found.

Changed on 2026-10-29 (the first form, of 2026-10-07, had only two counts: scored and skipped by the cost limit). Source: the user, answer G5 of the simulation of CAPABILITY-13 and CAPABILITY-14.

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [CAPABILITY-07: Show the cost of a search](../capabilities/CAPABILITY-07.md)
- about: [DECISION-12: The cost limit has a default that the user can change](DECISION-12.md)
- about: [DECISION-13: Reaching the cost limit has its own exit code](DECISION-13.md)
- traced by: [FEATURE-20: Show the cost of a search](../../roadmap/features/FEATURE-20.md)
<!-- LINKS:END -->
