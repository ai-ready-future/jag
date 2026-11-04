---
type: AcceptanceCriterion
title: A conflicting pull request fails with a clear message
description: When a pull request conflicts with main and has no merge ref, the private-data
  check fails before the checkout and says to merge main into the branch.
status: draft
generated:
  by: prd/6.2.0
  at: '2026-11-04T13:47:03Z'
item: /stories/STORY-80.md
---

# CRITERION-144: A conflicting pull request fails with a clear message

When a pull request conflicts with main and GitHub has no merge ref for it, the private-data check fails before the checkout with the message that the pull request conflicts with main and that main must be merged into the branch.

<!-- LINKS:START (generated, do not edit) -->

## Links

- item: [STORY-80: Name the merge conflict when private-data cannot check out a PR](../stories/STORY-80.md)
<!-- LINKS:END -->
