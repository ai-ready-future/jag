---
type: AcceptanceCriterion
title: CI runs each parity case as an e2e test
description: CI installs rg at a fixed version and runs each case of the parity test
  set as an e2e test on each pull request. A difference fails the check.
status: draft
item: /stories/STORY-66.md
---

# CRITERION-113: CI runs each parity case as an e2e test

CI installs rg at a fixed version and runs each case of the parity test set as an automated e2e test on each pull request. A difference in stdout, stderr, or exit code fails the check, and the check output shows the difference.

Source: user request to the orchestrator of 2026-10-15: "codify it so that it can be executed again in the future as an e2e test".

<!-- LINKS:START (generated, do not edit) -->

## Links

- item: [STORY-66: Run the rg parity cases as e2e tests in CI](../stories/STORY-66.md)
<!-- LINKS:END -->
