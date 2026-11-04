---
type: AcceptanceCriterion
title: jag-bench prints the best score of its own windows and no case text
description: The new bench-only flag orders the windows of each case perfectly, calls
  no model, and prints only 3 totals and no case text.
status: draft
generated:
  by: prd/6.2.0
  at: '2026-11-04T11:06:42Z'
item: /stories/STORY-79.md
---

# CRITERION-142: jag-bench prints the best score of its own windows and no case text

Given the development case file, the new bench-only flag of jag-bench orders the result windows of each case perfectly, makes no model call, and prints only the mean best ndcg@10 over the cases with an answer, the number of cases with more than 200 results, and the number of labeled lines that the pre-rank removes. It prints no question, no case text and no per-case result.

<!-- LINKS:START (generated, do not edit) -->

## Links

- item: [STORY-79: Measure the best possible jag-bench score on the development set](../stories/STORY-79.md)
<!-- LINKS:END -->
