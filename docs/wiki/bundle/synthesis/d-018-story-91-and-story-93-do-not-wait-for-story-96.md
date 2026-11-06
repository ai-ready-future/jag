---
type: Synthesis
title: 'D-018: STORY-91 and STORY-93 do not wait for STORY-96'
description: 'D-018 (2026-11-06, decision-judge): the report in DECISION-79 and DECISION-80 is the existing report of DECISION-42. STORY-96 is another report, so STORY-91 and STORY-93 need no blocked_by link to it.'
status: draft
sources:
- resource: prd:/decisions/DECISION-42.md
- resource: prd:/decisions/DECISION-74.md
- resource: prd:/decisions/DECISION-79.md
- resource: prd:/decisions/DECISION-80.md
- resource: roadmap:/stories/STORY-91.md
- resource: roadmap:/stories/STORY-93.md
generated:
  by: claude-code/sonnet-5.5
  at: '2026-11-06T07:09:10Z'
---

# D-018: STORY-91 and STORY-93 do not wait for STORY-96

Decision D-018 of 2026-11-06, made by decision-judge (level detail, confidence medium, reversible, hidden, band normal). Veracity 70.

## Question

Do STORY-91 and STORY-93 need `blocked_by` STORY-96, because they add to "the report"?

## Answer

No.

- "The report of each search" in DECISION-79 and DECISION-80 is the existing report of DECISION-42. It holds one count for each reason that jag did not score a result. FEATURE-20 built the report. STORY-91 adds the count for a private pattern to it.
- STORY-96 is another report: the one of DECISION-74, "the report of the text that leaves the machine". Only STORY-89 extends it.
- Links follow only if the STORY-96 spec makes the 2 reports one.

## Pages used

- Product decisions: DECISION-42, DECISION-74, DECISION-79, DECISION-80.
- Roadmap: STORY-91, STORY-93, and STORY-96. STORY-96 is in the open PR #375. Cite the page on `main` after the merge.

<!-- LINKS:START (generated, do not edit) -->

## Links

- sources: [prd:/decisions/DECISION-42.md](../../../product/prd/decisions/DECISION-42.md)
- sources: [prd:/decisions/DECISION-74.md](../../../product/prd/decisions/DECISION-74.md)
- sources: [prd:/decisions/DECISION-79.md](../../../product/prd/decisions/DECISION-79.md)
- sources: [prd:/decisions/DECISION-80.md](../../../product/prd/decisions/DECISION-80.md)
- sources: [roadmap:/stories/STORY-91.md](../../../product/roadmap/stories/STORY-91.md)
- sources: [roadmap:/stories/STORY-93.md](../../../product/roadmap/stories/STORY-93.md)
<!-- LINKS:END -->
