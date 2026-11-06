---
type: Story
title: The report of each search tells which text left the machine
description: As a knowledge seeker, I want each search that calls the model service
  to report which text left the machine, so that I know what jag sent.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-06T06:43:51Z'
state: backlog
parent: /features/FEATURE-32.md
milestone: /milestones/MILESTONE-04.md
traces:
- to: prd:/capabilities/CAPABILITY-09.md
  hash: 1e8d02
---

# STORY-96: The report of each search tells which text left the machine

As a knowledge seeker (PERSONA-03), I want each search that calls the model service to report which text left the machine, so that I know what jag sent (CAPABILITY-09).

Source: the answer of the user through hitl, 2026-11-05: "A: now, in Local model option".

## Scope

1. After each search that calls the model service, jag reports the number of results and the number of lines that it sent.
2. A search with no question makes no model call (DECISION-32), so it gives no report.
3. The spec decides if the report shows by default or only on request, where it goes, and its field in the structured output (CAPABILITY-05).

STORY-89 adds the host that gets the text to this report (DECISION-74). STORY-91 adds the count of held-back lines (DECISION-79), and STORY-93 adds the line that tells that the check is off (DECISION-80).

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [CRITERION-172: The report counts the text that left the machine](../criteria/CRITERION-172.md) | - | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [FEATURE-32: Say which text leaves the machine](../features/FEATURE-32.md)
- milestone: [MILESTONE-04: Local model option](../milestones/MILESTONE-04.md)
- traces: [CAPABILITY-09: Say which text leaves the machine](../../prd/capabilities/CAPABILITY-09.md)
- criterion or run of: [CRITERION-172: The report counts the text that left the machine](../criteria/CRITERION-172.md)
- blocks: [STORY-89: The report of the text that leaves the machine names the host](STORY-89.md)
<!-- LINKS:END -->
