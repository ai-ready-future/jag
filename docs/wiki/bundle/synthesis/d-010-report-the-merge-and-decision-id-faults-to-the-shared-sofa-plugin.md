---
type: Synthesis
title: 'D-010: report the merge and decision id faults to the shared sofa plugin'
description: 'Decision D-010 of 2026-11-03: send the decision log overwrite fault and the general merge author parts to the sofa plugin as 1 issue; jag-only rules stay in docs/adlc.md.'
status: draft
generated:
  by: claude-code/opus-5.5
  at: '2026-11-03T22:29:45Z'
sources:
- resource: prd:/constraints/CONSTRAINT-01.md
- resource: /synthesis/merge-rule-only-gh-pr-merge-squash-keep-commit-ccf4676.md
---

# D-010: report the merge and decision id faults to the shared sofa plugin

Decision D-010, made by decision-judge on 2026-11-03 (level middle, confidence medium, reversible, visible, band relaxed). Veracity 70. No human made this decision.

## Question

Must the new merge rule, the new decision rule, and the fault of the decision log (an overwrite) go into the shared sofa plugin?

## Answer

Yes, as 1 issue on the repository of the sofa plugin.

- The issue holds the overwrite fault: `sofa-state decision log` writes over a row that has the same id.
- The issue holds the work-around, the repository rule "Decision ids" of `docs/adlc.md`.
- The issue holds only the general parts of the merge author rule. The rules that are only for jag stay in `docs/adlc.md`.
- The issue holds no private data. The user reads the full text through hitl before the issue opens.

## New fact

The repository of the sofa plugin is private (checked with `gh repo view` on 2026-11-03). So the issue is not public, but its text must still follow `prd:/constraints/CONSTRAINT-01.md`.

## Where the rules are

`docs/adlc.md`, section 1 (the repository rules "Merge command" and "Decision ids") and section 3 (the roles).

## Open

The link to the issue is not known yet. A later change adds it after the issue opens.

<!-- LINKS:START (generated, do not edit) -->

## Links

- sources: [prd:/constraints/CONSTRAINT-01.md](../../../product/prd/constraints/CONSTRAINT-01.md)
- sources: [Merge rule: only gh pr merge --squash, keep commit ccf4676](merge-rule-only-gh-pr-merge-squash-keep-commit-ccf4676.md)
<!-- LINKS:END -->
