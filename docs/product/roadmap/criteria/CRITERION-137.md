---
type: AcceptanceCriterion
title: After the approval, the scan runs and the required check stops the merge until
  it passes
description: After a maintainer approves the run of a test fork pull request, the
  private-data scan runs, and the pull request cannot merge to main until an approved
  run passes.
status: draft
item: /stories/STORY-77.md
---

# CRITERION-137: After the approval, the scan runs and the required check stops the merge until it passes

After a maintainer approves the run of a test fork pull request, the `private-data` scan runs on it. The ruleset of `main` still requires the check `private-data`. The pull request cannot merge to `main` before the approval, and it cannot merge until an approved run passes.

<!-- LINKS:START (generated, do not edit) -->

## Links

- item: [STORY-77: Approve the private-data check of a fork pull request before it runs](../stories/STORY-77.md)
<!-- LINKS:END -->
