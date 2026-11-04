---
type: Story
title: Measure the best possible jag-bench score on the development set
description: As the maintainer, I want jag-bench to print the best score that a perfect
  order of its own windows can get, so that I know if a fix to jag can reach the bar.
status: draft
generated:
  by: process:github
  at: '2026-11-04T12:13:07Z'
state: done
actual_end_date: '2026-11-04'
github:
  number: 298
  node_id: I_kwDOVKMnL88AAAABcaqQHA
  url: https://github.com/ai-ready-future/jag/issues/298
  pushed_hash: 99503a
  synced:
    state: done
    owner: human:404pilo
spec: docs/superpowers/specs/2026-11-04-STORY-79-measure-the-best-possible-jag-bench-score-on-the-development-set-design.md
parent: /epics/EPIC-02.md
milestone: /milestones/MILESTONE-02.md
owner: human:404pilo
discovered_from: /stories/STORY-45.md
traces:
- to: prd:/metrics/METRIC-01.md
  hash: bc4856
---

# STORY-79: Measure the best possible jag-bench score on the development set

As the maintainer, I want jag-bench to print the best ndcg@10 that a perfect order of its own result windows can get on the development set, so that I know if a change to jag can reach the release bar of 0.70 before I change the prompt or the pre-rank.

The development test of STORY-45 failed only the jag check: blind grep passed, and jag got less than 0.70. The conversion rule, the converter and the 2 checks stay frozen (D-001, D-005, D-008). The cause of the gap is not known. Two of the known differences from the old run are parts of the jag-bench score (the line credit, and the ideal score over all labeled lines), not parts of the jag ranking. So the best possible score can be less than 0.70. Decision D-011 (decision-judge, 2026-11-04) chose to measure that score first.

The rules of D-011:

- The perfect order uses the result windows that jag-bench makes itself: the matcher, the scan, the window plan, and the pre-rank only when a case has more than 200 results. The pipeline stops before the model call. Each window gets the credit of the STORY-78 spec, row 11: the highest grade of its labeled match lines that no earlier window got. Context lines get no credit. The frozen score formula (STORY-78 spec, row 10) gives the score.
- The mode is a new flag of jag-bench, only in the `bench` feature. The default output and the default score path do not change, so the parity test (STORY-78 spec, row 7) stays true.
- The run uses the development case file that is already converted. There is no new conversion. The mode makes no model call, so it costs nothing.
- The mode prints only 3 totals: the mean best ndcg@10 over the cases with an answer, the number of cases with more than 200 results, and the number of labeled lines that the pre-rank removes. It prints no question, no case text and no per-case result (STORY-45 spec, row 5).
- This run is not a development run of the gate of STORY-45.

What comes next depends on the 3 totals. The researcher runs the mode with the maintainer and reads the totals:

- If the best score is clearly 0.70 or more, a fix story changes jag. A story that brings back the old prompt comes before a story that scores more than 200 results.
- If the best score is near 0.70 or less, the gate rows 23 and 25 of the STORY-45 spec go back to the human as a `revisit:`.
- The third total tells if the pre-rank limit or the window and ideal score rule is the limit.

The rule hash and the converter hash of STORY-45 stay the same. A changed prompt or threshold is frozen again before the held-out run, and the held-out run uses a new run commit after the fix (STORY-45 spec, row 7).

## Result

On 2026-11-04 the researcher and the maintainer ran `jag-bench --best-order` one time on the converted development case file (STORY-79 spec, row 16). The run made no model call, and the file was not converted again. The 3 totals stay on the maintainer's machine. Only this conclusion goes into the repository:

- The best possible score of the jag-bench windows is above the release bar of D-008 (jag 0.70 or more).
- The best order comes after the pre-rank (D-011, condition a). So the cost of the pre-rank is already in the best score, and the best score is still above the bar. The window and credit rules (STORY-78 spec, rows 10 and 11) and the pre-rank do not by themselves keep jag below the bar. The gap is in the order that jag gives to the windows.
- The best possible score is clearly above the bar, not near it (D-015), so the fix story changes jag: the old prompt first.

The next path, from D-011 (condition d) and D-015: a fix story for the order of jag that brings back the prompt of the old tool (STORY-81). STORY-45 waits for STORY-81. A change of the pre-rank limit is not an automatic second fix (D-015): it can only be an option on a `revisit:` card of STORY-45 spec rows 23 and 25.

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [CRITERION-142: jag-bench prints the best score of its own windows and no case text](../criteria/CRITERION-142.md) | - | - | - | - | - |
| [CRITERION-143: The default jag-bench output does not change](../criteria/CRITERION-143.md) | - | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [EPIC-02: Ranked search with file and line](../epics/EPIC-02.md)
- milestone: [MILESTONE-02: Make it good](../milestones/MILESTONE-02.md)
- discovered from: [STORY-45: Run the held-out evaluation before release](STORY-45.md)
- traces: [METRIC-01: Answer quality](../../prd/metrics/METRIC-01.md)
- criterion or run of: [CRITERION-142: jag-bench prints the best score of its own windows and no case text](../criteria/CRITERION-142.md)
- criterion or run of: [CRITERION-143: The default jag-bench output does not change](../criteria/CRITERION-143.md)
- blocks: [STORY-45: Run the held-out evaluation before release](STORY-45.md)
- discovered: [STORY-81: Score with the old prompt as a new prompt version](STORY-81.md)
<!-- LINKS:END -->
