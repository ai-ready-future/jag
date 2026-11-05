---
type: Synthesis
title: 'D-016 and STORY-81: the prompt jag-p2, the cut, and the decoys in one run'
description: 'D-016 (2026-11-04): STORY-81 measures the cut and the decoys in the 1 development run. The user chose a generic variant of the old prompt (2026-11-05), named jag-p2, and accepts that the score can differ.'
status: draft
sources:
- resource: roadmap:/stories/STORY-81.md
- resource: /synthesis/d-015-bring-back-the-old-prompt-first-story-79-conclusion.md
generated:
  by: claude-code/sonnet-5.5
  at: '2026-11-05T15:09:04Z'
---

# D-016 and STORY-81: the prompt jag-p2, the cut, and the decoys in one run

Two facts for STORY-81. Source: the spec on `main`, `docs/superpowers/specs/2026-11-04-STORY-81-score-with-the-old-prompt-as-a-new-prompt-version-design.md` (PR #334). The page has no prompt text, no path outside the repository, and no score of the development set.

## Decision D-016 (2026-11-04)

Made by decision-judge (level middle, confidence high, hard to reverse, visible). Veracity 70.

STORY-81 measures the threshold and the decoys in the 1 development run of the new prompt. A rule that the team writes before the run decides each part:

- **The cut.** The rule tries each cut from 0.05 to 0.95 in steps of 0.05. It picks the best balanced accuracy over the cases with an answer and the decoys that got a score. The cut stays 0.4, unless the best cut is 0.06 or more higher. A tie goes to the cut nearest 0.4, then to the lower cut.
- **The decoys.** A bench-only option of `jag-bench` scores the cases with no answer in the same run. The 4 gate lines do not change (D-008).
- **The checks before the run.** Chores checks 2 counts: 9 cases with all subjects absent, and 0 decoys with an empty term.
- **The pass rule.** The decoy result passes at AUC 0.85 or more (METRIC-07) with 5 or more scored decoys.
- **A decoy fail after a pass of check 2 is a stop.** There is no held-out run and no prompt change. A `revisit:` card goes to the human.

Why: only this meets ADR-0003 and CRITERION-146 with 1 run (D-015) and no change to the gate lines (D-008).

## Decision of the user (2026-11-05)

Made by the human `404pilo` through chores. Veracity 90.

- STORY-81 uses a generic variant of the old prompt `noul-b-v1`, not the exact text. The variant has generic words in some words of the 2 criteria lines.
- The new prompt version is named `jag-p2`, so the name shows that it is a variant.
- The user accepts that the score can differ from the old score.

## Next

The implementer builds the code, then chores guides the 1 development run. See the earlier decision on [the old prompt first](d-015-bring-back-the-old-prompt-first-story-79-conclusion.md).

<!-- LINKS:START (generated, do not edit) -->

## Links

- sources: [roadmap:/stories/STORY-81.md](../../../product/roadmap/stories/STORY-81.md)
- sources: [D-015: bring back the old prompt first (STORY-79 conclusion)](d-015-bring-back-the-old-prompt-first-story-79-conclusion.md)
<!-- LINKS:END -->
