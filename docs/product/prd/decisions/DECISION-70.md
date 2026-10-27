---
type: Decision
title: The maintainer runs the held-out evaluation once, on their own machine, with
  the Rust jag
description: The held-out set stays private. Before the first release tag, the maintainer
  runs the Rust jag once on the held-out split on their own machine, and only a summary
  of the numbers goes public.
status: stable
decided_at: '2026-10-27'
alternatives:
- Code here, set in a private repo
- All of it in a private repo
- All of it in this repo (withdrawn 2026-10-27)
- Accept the prototype result
- Write a new public set
about:
- /scope/SCOPE-10.md
- /metrics/METRIC-01.md
- /constraints/CONSTRAINT-01.md
---

# DECISION-70: The maintainer runs the held-out evaluation once, on their own machine, with the Rust jag

Principle: Run Rust jag once, privately.

The held-out set is private case material that is not jag's. It stays on the maintainer's machine and never goes into this repository (CONSTRAINT-01).

Before the first release tag, the maintainer runs the Rust jag one time on the held-out split, on their own machine. A session guides the maintainer through the run. No CI job runs it.

These rules apply to the run:

- The prompt and the 0.70 threshold are frozen before the run. The held-out split must not be used to choose anything: no prompt change, no setting change, and no tuning on the result.
- Only a summary of the numbers goes public: the release notes and one README line. No case text, no question, no per-case result, and no data path goes public.
- If jag gets an ndcg@10 below 0.70, or a score that is not higher than blind grep, there is no release tag. A fix story comes first.
- The run tool is part of the Rust jag. Its code and its invented test cases can be public. The data path, the key, and the per-case results stay on the maintainer's machine.

Source: the user, in the hitl card "Held-out run", 2026-10-27: "Run Rust jag once, privately" (option 1). In the same answer, the user withdrew the earlier answer "All of it in this repo" to the card "Where do the held-out set and its run tool go?". This answer changes STORY-45 and CRITERION-73, and adds STORY-78 for the run tool.

<!-- LINKS:START (generated, do not edit) -->

## Links

- about: [SCOPE-10: The held-out score confirms the gain over grep](../scope/SCOPE-10.md)
- about: [METRIC-01: Answer quality](../metrics/METRIC-01.md)
- about: [CONSTRAINT-01: The public release holds no private data](../constraints/CONSTRAINT-01.md)
<!-- LINKS:END -->
