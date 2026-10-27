---
type: AcceptanceCriterion
title: The held-out score meets the target and beats grep
description: One local run of the Rust jag on the held-out split gives an ndcg@10
  of 0.70 or more, higher than blind grep. Only the numbers go public. A lower score
  means no release tag.
status: draft
item: /stories/STORY-45.md
---

# CRITERION-73: The held-out score meets the target and beats grep

The maintainer runs the Rust jag one time on the held-out split, on their own machine (DECISION-70). The run gives jag an ndcg@10 of 0.70 or more, which is higher than blind grep.

- The prompt and the 0.70 threshold are frozen before the run. The held-out split is not used to choose anything, and nothing is tuned on the result.
- The public summary shows only the numbers and no case content. It goes in the release notes and in one README line.
- The data path, the key, and the per-case results stay on the maintainer's machine.
- If the score is below 0.70, or not higher than blind grep, there is no release tag, and a fix story comes first. STORY-51, which publishes from the tag, is blocked by STORY-45.

<!-- LINKS:START (generated, do not edit) -->

## Links

- item: [STORY-45: Run the held-out evaluation before release](../stories/STORY-45.md)
<!-- LINKS:END -->
