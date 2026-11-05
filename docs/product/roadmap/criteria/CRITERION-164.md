---
type: AcceptanceCriterion
title: A result that matches a built-in pattern is not sent
description: A result that matches a built-in pattern is not sent and not scored;
  its line is listed as not scored with the reason, and the exit code does not change.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-05T22:52:34Z'
item: /stories/STORY-91.md
---

# CRITERION-164: A result that matches a built-in pattern is not sent

A result whose text or lines around the match hold a key, a token or a password of the built-in list is not sent to the model and not scored. The result lists the line as not scored, with the reason. The report counts the held-back lines, and the exit code is the code that the search gets without them (DECISION-67, DECISION-77, DECISION-79). This is true for TypeSafe, an AI gateway and a Clef server (DECISION-66).

<!-- LINKS:START (generated, do not edit) -->

## Links

- item: [STORY-91: Hold back each snippet that holds a key, a token or a password](../stories/STORY-91.md)
<!-- LINKS:END -->
