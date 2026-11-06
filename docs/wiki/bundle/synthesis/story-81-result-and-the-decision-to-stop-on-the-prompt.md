---
type: Synthesis
title: 'STORY-81 result: check 2 failed, the human stops work on the prompt (2026-11-06)'
description: STORY-81 scored the old prompt as a new prompt version. Check 1 passed, check 2 failed, the decoy result passed. The human chose to stop and change no STORY-45 spec row.
status: draft
generated:
  by: knowledge/1.4.8
  at: '2026-11-06T17:51:14Z'
sources:
- id: STORY-81
  resource: roadmap:/stories/STORY-81.md
  title: Score with the old prompt as a new prompt version
  last_modified: '2026-11-04'
- id: STORY-79
  resource: roadmap:/stories/STORY-79.md
  title: Measure the best possible jag-bench score on the development set
  last_modified: '2026-11-04'
- id: d-016-and-story-81-the-prompt-jag-p2-the-cut-and-the-decoys-in-one-run
  resource: /synthesis/d-016-and-story-81-the-prompt-jag-p2-the-cut-and-the-decoys-in-one-run.md
  title: 'D-016 and STORY-81: the prompt jag-p2, the cut, and the decoys in one run'
  last_modified: '2026-11-05'
- id: story-45-practice-gate-and-conversion-rule
  resource: /synthesis/story-45-practice-gate-and-conversion-rule.md
  title: STORY-45 practice gate and conversion rule
  last_modified: '2026-11-03'
- id: d-015-bring-back-the-old-prompt-first-story-79-conclusion
  resource: /synthesis/d-015-bring-back-the-old-prompt-first-story-79-conclusion.md
  title: 'D-015: bring back the old prompt first (STORY-79 conclusion)'
  last_modified: '2026-11-04'
---

# STORY-81 result: check 2 failed, the human stops work on the prompt (2026-11-06)

This page holds words only. It has no score, no count, no run number, no model output text, and no private path.

Source: the STORY-81 spec on `main`, `docs/superpowers/specs/2026-11-04-STORY-81-score-with-the-old-prompt-as-a-new-prompt-version-design.md`. Earlier facts: [D-016 and STORY-81](d-016-and-story-81-the-prompt-jag-p2-the-cut-and-the-decoys-in-one-run.md).

## Setup (human choice, veracity 90)

- The run used the hosted default model. The human chose this through hitl (answer A).
- The earlier failed runs came from a local server with 1 process. Some calls ended because jag stopped waiting first. The server log showed no server error.

## Result (veracity 90)

- Check 1 passed.
- Check 2 failed.
- The decoy result passed.
- The threshold rule of STORY-81 spec row 7 gave a cut higher than the default cut of that row (D-016).

## Decision (human, through hitl, 2026-11-06, option C, veracity 90)

- Stop this line of work on the prompt.
- Record the result in the knowledge graph only.
- Do not change rows 23 and 25 of the STORY-45 spec now.
- The other options were A (keep the rows and close) and B (study the higher cut that the row 7 rule gives). The human did not choose them.

## Related pages

- Roadmap: STORY-81 and STORY-79.
- PRD: METRIC-01.
- The earlier rule for a failed check 2: [D-015](d-015-bring-back-the-old-prompt-first-story-79-conclusion.md). It sent rows 23 and 25 to the human. This decision answers that.
- [STORY-45 practice gate and conversion rule](story-45-practice-gate-and-conversion-rule.md).

<!-- LINKS:START (generated, do not edit) -->

## Links

- sources: [roadmap:/stories/STORY-81.md](../../../product/roadmap/stories/STORY-81.md)
- sources: [roadmap:/stories/STORY-79.md](../../../product/roadmap/stories/STORY-79.md)
- sources: [D-016 and STORY-81: the prompt jag-p2, the cut, and the decoys in one run](d-016-and-story-81-the-prompt-jag-p2-the-cut-and-the-decoys-in-one-run.md)
- sources: [STORY-45 practice gate and conversion rule](story-45-practice-gate-and-conversion-rule.md)
- sources: [D-015: bring back the old prompt first (STORY-79 conclusion)](d-015-bring-back-the-old-prompt-first-story-79-conclusion.md)
<!-- LINKS:END -->
