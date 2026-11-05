---
type: Synthesis
title: 'D-015: bring back the old prompt first (STORY-79 conclusion)'
description: 'STORY-79 is done: the best possible score of the jag-bench windows is clearly above the D-008 bar. D-015 chooses the fix story STORY-81, the old prompt as a new prompt version, with a stop rule.'
status: draft
sources:
- resource: roadmap:/stories/STORY-79.md
- resource: roadmap:/stories/STORY-81.md
generated:
  by: claude-code/sonnet-5.5
  at: '2026-11-04T14:26:38Z'
---

# D-015: bring back the old prompt first (STORY-79 conclusion)

Decision D-015 of 2026-11-04, made by decision-judge (level middle, confidence high, reversible). Veracity 70. The conclusion of STORY-79 is a result of the team (the maintainer ran it one time). The page has no number of that run.

## Result of STORY-79 (done, PR #314)

- STORY-79 measured the best possible jag-bench score on the development set. The score uses a perfect order of the windows that jag-bench makes itself, after the pre-rank (D-011).
- The best possible score is clearly above the bar of D-008, not near it.
- So the window rules, the credit rules, and the pre-rank do not by themselves keep jag below the bar. The gap is in the order that jag gives to the windows.

## Decision D-015

- Take path A of D-011: the fix story STORY-81 brings back the prompt of the old tool as a new prompt version.
- Stop rule: 1 development run under the frozen check 2. There is no third prompt, and the prompt does not change between runs.
- If check 2 fails, rows 23 and 25 of the STORY-45 spec go to the human as a `revisit:`.
- A change of the pre-rank limit is not a second automatic fix. It conflicts with METRIC-05, and a new pre-rank default changes ADR-0002. So it is only an option on that `revisit:` card.

## Next

STORY-45 waits for STORY-81. Earlier decisions: [the practice gate and conversion rule](story-45-practice-gate-and-conversion-rule.md) holds D-008 and D-011.

<!-- LINKS:START (generated, do not edit) -->

## Links

- sources: [roadmap:/stories/STORY-79.md](../../../product/roadmap/stories/STORY-79.md)
- sources: [roadmap:/stories/STORY-81.md](../../../product/roadmap/stories/STORY-81.md)
- fed: [D-016 and STORY-81: the prompt jag-p2, the cut, and the decoys in one run](d-016-and-story-81-the-prompt-jag-p2-the-cut-and-the-decoys-in-one-run.md)
<!-- LINKS:END -->
