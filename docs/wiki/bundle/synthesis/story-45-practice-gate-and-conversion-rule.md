---
type: Synthesis
title: STORY-45 practice gate and conversion rule
description: The 2 checks of the development practice run (D-008), the conversion rule of path (a) (D-006), and the empty-term rule.
status: draft
generated:
  by: claude-code/opus-5.5
  at: '2026-11-03T21:59:06Z'
sources:
- resource: roadmap:/stories/STORY-45.md
- resource: prd:/decisions/DECISION-70.md
- resource: /concepts/evaluation-method-of-the-earlier-search-tool.md
---

# STORY-45 practice gate and conversion rule

The text of the STORY-45 spec on `main` (PR #288, rows 21 to 26 and "The conversion rule"). The spec file is `docs/superpowers/specs/2026-10-27-STORY-45-run-the-held-out-evaluation-before-release-design.md`. This page holds no private name, path, or case id.

## Why a practice run

The first development test of the labeling rule failed (see [the STORY-45 page](/synthesis/story-45-test-the-labeling-rule-on-the-development-set-first.md)). The planner chose path (a), "convert with the old method" (decision D-006, the STORY-45 path, 2026-11-03). The conversion rule below replaces the rule of the first test. It maps the method of [the earlier evaluation tool](/concepts/evaluation-method-of-the-earlier-search-tool.md) to the labeled lines of `jag-bench`.

## The practice gate (decision D-008)

Decided by human `404pilo`, through hitl, on 2026-11-03 (card "How close the practice scores must be", option "Two checks, written before the run", no condition). Veracity 90.

Before the private run, `jag-bench` runs once on the converted development case file: 74 cases with an answer and 9 cases with no answer. The score of a tool is the mean of its ndcg@10 over the cases with an answer.

1. Label check (blind grep): the blind grep score must be 0.10 or less. The old score was 0.06. Blind grep uses no model, so this check tests the labels and the metric.
2. Bar check (jag): the jag score must be 0.70 or more. This is the release bar of DECISION-70 and CRITERION-73. It uses the fixed prompt and threshold of `jag-bench`. A jag score above the old score (0.73) is not a failure.

Outcome:

- Check 1 fails: the labels are wrong. Stop. There is no private run. The planner chooses the next path.
- Check 1 passes and check 2 fails: there is no private run. A fix story comes first, and the planner writes it.
- Both pass: freeze the rule and the converter, convert the private set, and do the one private run.

The 2 checks and the conversion rule are frozen before the practice scores are seen. They do not change after the scores. Nothing is tuned on the result.

## The freeze

Before the development run, the rule hash and the SHA-256 of the converter are recorded, and only the first 12 characters of each are printed. After a pass of both checks, the private set is converted with the same converter. The hashes must be the same before the one run.

## The conversion rule (decision D-006)

For each case of the old set:

1. Term: use the term rule of the earlier tool on the question text (keep the first 12 unique terms). Join the escaped terms with `|` into 1 pattern, with `ignore_case: true` and `fixed_strings: false`.
2. Evidence lines: the gold documents are the expected documents of each subject whose form is not `absent`. The needles are the expected values of each subject (split at `;`, searched only in the documents of that subject), and the `mustContain` entries (a list gives each string, searched in all gold documents). Keep a needle only with a digit or 5 characters or more. A needle that matches more than 5 lines of a document marks no line of that document. A needle with a line break marks no line.
3. Labels: grade 2 for each line of a gold document that matches the term and is at most 3 lines from an evidence line of the same document. Grade 1 for each other matching line of a gold document. A line gets 1 label only, the higher grade. An evidence line that does not match the term gets no label.
4. No answer: a case with no label gets `answers: []`. It stays in the case file and is not in the mean.
5. Empty term: a case whose term is empty after step 1 gets `answers: []` and the fixed term `no-term`, because `jag-bench` needs a term that is not empty. `jag-bench` does not search for the term of a case with no answer.

## The count check

The converter prints only counts. At the development conversion (step 4.3 of the spec), the set must give 74 cases with an answer and 9 with no answer. If the counts differ, find the cause before the freeze. If the cause is an error in the converter, fix it and convert again. Otherwise stop and send `BLOCKED · decision` with the 2 counts. No score is seen yet, so this is not tuning. The spec gives no expected count for the private set.

## Result of the practice run and decision D-011 (2026-11-04)

Result: blind grep 0.063 (check 1 passes). jag 0.592 (check 2 fails; the bar is 0.70). By D-008, there is no private run, and a fix story comes first.

Decision D-011 was made by decision-judge on 2026-11-04 (level middle, confidence high, reversible, hidden). Veracity 70. No human made this decision.

- The fix story first measures the best possible score under the frozen `jag-bench` metric.
- A bench-only flag behind `--features bench` ranks the own windows of `jag-bench` in the best order, with no model call. It prints 3 totals only: the ceiling, the cases with more than 200 results, and the labeled lines that the pre-rank removes.
- If the ceiling is clearly 0.70 or more, the next fix is the old prompt, then the pre-rank.
- If the ceiling is near or below 0.70, the gate (rows 23 and 25 of the spec) goes back to the human.
- The rule hash and the converter hash stay. A changed prompt or threshold is frozen again before the held-out run.

## A failed gate

No private set is converted and no run occurs. The failed check is named in a `BLOCKED · decision` message with no number.

<!-- LINKS:START (generated, do not edit) -->

## Links

- sources: [roadmap:/stories/STORY-45.md](../../../product/roadmap/stories/STORY-45.md)
- sources: [prd:/decisions/DECISION-70.md](../../../product/prd/decisions/DECISION-70.md)
- sources: [Evaluation method of the earlier search tool](../concepts/evaluation-method-of-the-earlier-search-tool.md)
<!-- LINKS:END -->
