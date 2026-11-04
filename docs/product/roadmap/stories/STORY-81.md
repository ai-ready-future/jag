---
type: Story
title: Score with the old prompt as a new prompt version
description: As the maintainer, I want jag to score with the prompt of the old tool
  as a new prompt version, so that jag can reach the release bar before the held-out
  run.
status: draft
generated:
  by: prd/6.2.0
  at: '2026-11-04T14:10:26Z'
state: backlog
parent: /epics/EPIC-02.md
milestone: /milestones/MILESTONE-02.md
discovered_from: /stories/STORY-79.md
traces:
- to: prd:/metrics/METRIC-01.md
  hash: bc4856
---

# STORY-81: Score with the old prompt as a new prompt version

As the maintainer, I want jag to score with the prompt of the old tool as a new prompt version, tuned and tested on the development set, so that jag can reach the release bar before the one held-out run.

STORY-79 showed that the best possible score of the jag-bench windows is above the release bar of D-008, also after the pre-rank. So the gap is in the order that jag gives to the windows. The prompt is the largest known difference that jag controls: the development run of STORY-45 used `jag-p1`, and the old tool used the prompt `noul-b-v1`. D-011 (condition d) puts this story first. A story that scores more than 200 results comes only after it, because METRIC-05 says that jag must not raise answer quality by scoring many more snippets.

The scope:

- Bring the prompt of the old tool into jag as a new `PROMPT_VERSION`. Each release holds one built-in prompt with a version id (ADR-0004).
- Tune the threshold with the new prompt on the development set, and keep the decoy AUC at the bar of METRIC-07 (ADR-0003). The model stays the pinned model (ADR-0009).
- Run the development test of STORY-45 one more time, with the frozen 2 checks (D-008), the same conversion rule and the same converter. The rule hash and the converter hash do not change (D-011).
- Freeze the new prompt and the new threshold before the held-out run. The held-out run uses a new run commit after this fix (STORY-45 spec, row 7).

A step before the work: the text of the old prompt is not in this repository and not on this machine. It is in the old prototype folder of the "Extra context files" row of `docs/adlc.md`. The user gets the text out with `git show` or `git archive`, as that row says, and does not edit files in that folder. Chores guides this step.

If the jag check of the development test fails again, the next path is the story that scores more than 200 results (with the limits of METRIC-05), or a `revisit:` of STORY-45 spec rows 23 and 25 through a card.

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [CRITERION-146: jag uses the new prompt version and a tuned threshold](../criteria/CRITERION-146.md) | - | - | - | - | - |
| [CRITERION-147: The development test runs again with the frozen checks](../criteria/CRITERION-147.md) | - | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [EPIC-02: Ranked search with file and line](../epics/EPIC-02.md)
- milestone: [MILESTONE-02: Make it good](../milestones/MILESTONE-02.md)
- discovered from: [STORY-79: Measure the best possible jag-bench score on the development set](STORY-79.md)
- traces: [METRIC-01: Answer quality](../../prd/metrics/METRIC-01.md)
- criterion or run of: [CRITERION-146: jag uses the new prompt version and a tuned threshold](../criteria/CRITERION-146.md)
- criterion or run of: [CRITERION-147: The development test runs again with the frozen checks](../criteria/CRITERION-147.md)
- blocks: [STORY-45: Run the held-out evaluation before release](STORY-45.md)
<!-- LINKS:END -->
