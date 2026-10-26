---
type: AcceptanceCriterion
title: A fork pull request shows no private-data result before a maintainer approves
  the run
description: A test fork pull request to ai-ready-future/jag gets no private-data
  scan and shows no result of the scan until a maintainer approves the run.
status: draft
item: /stories/STORY-77.md
---

# CRITERION-136: A fork pull request shows no private-data result before a maintainer approves the run

A test fork pull request to `ai-ready-future/jag` gets no `private-data` scan until a maintainer approves the run. Before the approval, the pull request shows that the check waits for an approval. It shows no pass or fail of the scan.

This criterion and CRITERION-137 and CRITERION-138 replace the fork test of issue #99 (DECISION-57).

<!-- LINKS:START (generated, do not edit) -->

## Links

- item: [STORY-77: Approve the private-data check of a fork pull request before it runs](../stories/STORY-77.md)
<!-- LINKS:END -->
