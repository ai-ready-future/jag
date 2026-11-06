---
type: Synthesis
title: 'D-021: keep exit 3 for a readable config file on a search with --ask'
description: 'D-021 (2026-11-06, decision-judge): after STORY-84, a search with --ask reads the config file when no variable sets a time or size. A file that other users can read gives exit 3, also when the key comes from a variable. The team keeps this.'
status: draft
sources:
- resource: roadmap:/stories/STORY-84.md
- resource: roadmap:/stories/STORY-57.md
- resource: prd:/decisions/DECISION-04.md
- resource: adr:/0029-read-each-setting-from-the-config-file-and-let-setup-write.md
generated:
  by: claude-code/sonnet-5.5
  at: '2026-11-06T15:19:20Z'
---

# D-021: keep exit 3 for a readable config file on a search with --ask

Decision D-021 of 2026-11-06, made by decision-judge (level detail, confidence high, reversible, visible, band relaxed), before release 1. Veracity 70.

## Question

After STORY-84, a search with `--ask` reads the config file when no variable sets a time or size. So a config file that other users can read gives exit 3, also when the key comes from a variable. Keep this, or change the code?

## Answer

Keep it. Do not change the code.

- ADR-0029 applies the file checks to each flow that reads the file. FEATURE-21 made the same trade.
- The exit 3 message names the cause and the fix: `chmod 600`.
- The user can also avoid the file. Set the 4 variables: `JAG_TIMEOUT`, `JAG_RETRY_WAIT`, `JAG_MAX_SCORED` and `JAG_MAX_CHARS`.
- The STORY-84 spec Risks list and D-020 name the search too.

## Pages used

- ADR-0029 (adr).
- STORY-84 spec, rows 7 to 9: the flows that read a key, when jag reads the file, and `--json` with no question.
- STORY-57 spec, Risk 1.
- Product decision DECISION-04, item 5 (the key-file item).
- The code is on `main` (STORY-84, commit 7efc286): `src/config.rs` (the file checks and the `chmod 600` fix text) and `src/config/limit.rs` (the 4 variables).
- STORY-84 spec on `main`: `docs/superpowers/specs/2026-11-05-STORY-84-each-setting-in-the-config-file-design.md`.

<!-- LINKS:START (generated, do not edit) -->

## Links

- sources: [roadmap:/stories/STORY-84.md](../../../product/roadmap/stories/STORY-84.md)
- sources: [roadmap:/stories/STORY-57.md](../../../product/roadmap/stories/STORY-57.md)
- sources: [prd:/decisions/DECISION-04.md](../../../product/prd/decisions/DECISION-04.md)
- sources: [adr:/0029-read-each-setting-from-the-config-file-and-let-setup-write.md](../../../adr/0029-read-each-setting-from-the-config-file-and-let-setup-write.md)
<!-- LINKS:END -->
