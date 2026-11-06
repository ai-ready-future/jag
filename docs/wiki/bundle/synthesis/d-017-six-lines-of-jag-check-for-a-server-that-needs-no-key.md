---
type: Synthesis
title: 'D-017: six lines of jag --check for a server that needs no key'
description: 'D-017 (2026-11-05, decision-judge): on the route of a server that needs no key, jag --check shows 6 lines. Both key lines say not needed, and a new line shows that the server answers a scoring call.'
status: draft
sources:
- resource: prd:/decisions/DECISION-75.md
- resource: prd:/decisions/DECISION-04.md
- resource: prd:/decisions/DECISION-48.md
- resource: prd:/questions/QUESTION-04.md
generated:
  by: claude-code/sonnet-5.5
  at: '2026-11-06T06:55:31Z'
---

# D-017: six lines of jag --check for a server that needs no key

Decision D-017 of 2026-11-05, made by decision-judge (level detail, confidence medium, reversible, visible, band relaxed). Veracity 70.

## Question

How do the lines of `jag --check` show a server that needs no key? DECISION-75 does not map the 5 items of DECISION-04 to this route.

## Answer

On this route `jag --check` shows 6 lines:

- Both key lines say "not needed".
- The credit line says "not available".
- A new line, "the server answers a scoring call", comes after the line "the service is reachable".
- The key-file line does not change.
- A refused call with no key fails the new line. The exit code is 4.

The other routes keep 5 lines.

## Why

- DECISION-75 says "the key items" in the plural, so it covers more than one line.
- The line form is not a contract (STORY-06 spec, decision 16).

## Pages used

- Product decisions: DECISION-75, DECISION-04, DECISION-48. Question: QUESTION-04.
- STORY-06 spec, decisions 12 and 16.
- STORY-86 spec, row 9 (PR #374). The PR is not merged yet. Cite the spec on `main` after the merge.

<!-- LINKS:START (generated, do not edit) -->

## Links

- sources: [prd:/decisions/DECISION-75.md](../../../product/prd/decisions/DECISION-75.md)
- sources: [prd:/decisions/DECISION-04.md](../../../product/prd/decisions/DECISION-04.md)
- sources: [prd:/decisions/DECISION-48.md](../../../product/prd/decisions/DECISION-48.md)
- sources: [prd:/questions/QUESTION-04.md](../../../product/prd/questions/QUESTION-04.md)
<!-- LINKS:END -->
