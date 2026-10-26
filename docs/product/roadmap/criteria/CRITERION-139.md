---
type: AcceptanceCriterion
title: Pull requests of this repository and pushes to main keep the automatic scan
description: A pull request from a branch of ai-ready-future/jag and a push to main
  run the private-data scan with no approval, and the job id stays private-data.
status: draft
item: /stories/STORY-77.md
---

# CRITERION-139: Pull requests of this repository and pushes to main keep the automatic scan

A pull request from a branch of `ai-ready-future/jag` runs the `private-data` scan with no approval, as before. A push to `main` also runs the scan with no approval. The job id stays `private-data`, so the ruleset of `main` does not change.

<!-- LINKS:START (generated, do not edit) -->

## Links

- item: [STORY-77: Approve the private-data check of a fork pull request before it runs](../stories/STORY-77.md)
<!-- LINKS:END -->
