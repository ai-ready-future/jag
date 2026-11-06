---
type: Synthesis
title: 'STORY-81 exit 6: the scoring deadline, the timeout settings and the size of a call'
description: 'Facts of the troubleshooter on code 8ea270e: the scoring deadline covers all calls of 1 search; only --timeout and JAG_TIMEOUT set the times until STORY-84; jag-p2 makes each call bigger; jag-bench stops on the first call that fails 2 times.'
status: draft
sources:
- resource: roadmap:/stories/STORY-81.md
- resource: roadmap:/stories/STORY-84.md
- resource: roadmap:/stories/STORY-13.md
generated:
  by: claude-code/sonnet-5.5
  at: '2026-11-06T11:46:58Z'
---

# STORY-81 exit 6: the scoring deadline, the timeout settings and the size of a call

4 facts about exit 6 in STORY-81. The troubleshooter confirmed them on the code at commit 8ea270e of `main`. Veracity 60 (result of a troubleshooter, read from the code). The cause on the gateway side is not confirmed, so this page does not give it. The page holds no case text, no score and no run number.

## Facts

1. **The deadline covers 1 search.** The scoring deadline covers all calls of 1 search (1 jag-bench case), not 1 call. It is 2 x timeout + retry_wait (`src/scorer.rs`, `ScoreSettings::new`). It is 9 s by default and 121 s at `JAG_TIMEOUT=60`.
2. **STORY-84 reads the config file.** `config.timeout` and `config.retry-wait` are not read until STORY-84. Only `--timeout` and `JAG_TIMEOUT` set the times, and the same is true for the retry wait (`src/config/model.rs`, `time()`). jag-bench has no `--timeout` option, so only the variable works there. CONVENTION-05 (draft) says otherwise, so it is wrong until STORY-84.
3. **jag-p2 makes each call bigger.** It adds about 12 KB to each call of up to 32 results, because each question holds the 2 criteria (430 B for each question, was 69 B). The batch split counts the criteria 1 time, as the STORY-13 spec row 22 says, so the criteria add only about 120 tokens to the estimate of a call. The cap of 32 results sets the number of calls, except for a call near the estimate of 24,000, which can close 1 result sooner.
4. **jag-bench stops on the first failed call.** It stops the whole run on the first call that fails. A call that can retry fails after 1 retry after retry_wait. A refusal (401, 402 or 403), or a retry that the deadline does not allow, fails after 1 attempt. It keeps no result of the cases that ended.

## Pages used

- Roadmap: STORY-81, STORY-84, STORY-13 (spec row 22).
- Code: `src/scorer.rs` and `src/config/model.rs` at commit 8ea270e. The scribe checked fact 1 in the code. The reviewer of PR #382 checked all 4 facts in the code at that commit.

<!-- LINKS:START (generated, do not edit) -->

## Links

- sources: [roadmap:/stories/STORY-81.md](../../../product/roadmap/stories/STORY-81.md)
- sources: [roadmap:/stories/STORY-84.md](../../../product/roadmap/stories/STORY-84.md)
- sources: [roadmap:/stories/STORY-13.md](../../../product/roadmap/stories/STORY-13.md)
<!-- LINKS:END -->
