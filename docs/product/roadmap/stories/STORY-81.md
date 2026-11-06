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
state: cancelled
github:
  number: 316
  node_id: I_kwDOVKMnL88AAAABcdSeRA
  url: https://github.com/ai-ready-future/jag/issues/316
  pushed_hash: d2e60a
  synced:
    state: cancelled
    owner: human:404pilo
spec: docs/superpowers/specs/2026-11-04-STORY-81-score-with-the-old-prompt-as-a-new-prompt-version-design.md
parent: /epics/EPIC-02.md
milestone: /milestones/MILESTONE-02.md
owner: human:404pilo
discovered_from: /stories/STORY-79.md
traces:
- to: prd:/metrics/METRIC-01.md
  hash: bc4856
---

# STORY-81: Score with the old prompt as a new prompt version

As the maintainer, I want jag to score with the prompt of the old tool as a new prompt version, tuned and tested on the development set, so that jag can reach the release bar before the one held-out run.

STORY-79 showed that the best possible score of the jag-bench windows is above the release bar of D-008, also after the pre-rank. So the gap is in the order that jag gives to the windows. The prompt is the largest known difference that jag controls: the development run of STORY-45 used `jag-p1`, and the old tool used the prompt `noul-b-v1`. D-011 (condition d) and D-015 choose this story: the best possible score is clearly above the bar, not near it.

The scope:

- Bring the prompt of the old tool into jag as a new `PROMPT_VERSION`. Each release holds one built-in prompt with a version id (ADR-0004).
- Tune the threshold with the new prompt on the development set, and keep the decoy AUC at the bar of METRIC-07 (ADR-0003). The model stays the pinned model (ADR-0009).
- Run the development test of STORY-45 one more time, with the frozen 2 checks (D-008), the same conversion rule and the same converter. The rule hash and the converter hash do not change (D-011).
- Freeze the new prompt and the new threshold before the held-out run. The held-out run uses a new run commit after this fix (STORY-45 spec, row 7).

A step before the work: the text of the old prompt is not in this repository and not on this machine. It is in the old prototype folder of the "Extra context files" row of `docs/adlc.md`. The user gets the text out with `git show` or `git archive`, as that row says, and does not edit files in that folder. Chores guides this step.

The stop rule (D-015). The specifier writes it into the STORY-81 spec before the development run:

- The old prompt, as a new prompt version, gets 1 development run under the frozen check 2. There is no third prompt, and the prompt does not change between runs.
- If check 2 fails, STORY-45 spec rows 23 and 25 go back to the human as a `revisit:`.
- A change of the pre-rank limit (more than 200 results) is not an automatic second fix. It conflicts with METRIC-05 and needs a change of ADR-0002 and DECISION-12, so it can only be an option on that `revisit:` card.
- STORY-45 spec row 23 says that the practice test runs one time, with `jag-p1`. The STORY-81 spec changes that row for this second development run.

## Result

STORY-81 is cancelled on 2026-11-06. The second practice test ran with `jag-p2` (code PR #338). Check 1 passed, check 2 failed, and the decoy result passed. The threshold rule of STORY-81 spec row 7 gave a cut higher than the default cut of that row (D-016).

By the stop rule (spec row 9), rows 23 and 25 went back to the human as a `revisit:` card. The human chose option C through hitl on 2026-11-06: "stop, write result in graph". This line of work on the prompt stops, and STORY-45 spec rows 23 and 25 do not change. The scribe records the result in the knowledge graph. Options not chosen: A (keep the rows and close STORY-81) and B (study the higher cut that the row 7 rule gives).

STORY-81 is not `done`, because spec row 21 says that it is `done` only when check 2 also passes.

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
