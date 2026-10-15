---
type: adr
title: Use rg as the reference for the output of jag with no question
description: Open this when you change what jag writes with no --ask (stdout, stderr, color, exit code, match rules), or when you change the rg version that jag follows.
area: contracts
status: proposed
date: 2026-10-15
supersedes: null
superseded_by: null
amends: null
refines: null
deviates_from: null
related:
  - 0001-find-matches-with-the-ripgrep-libraries.md
  - 0018-take-the-question-only-from-the-ask-option.md
  - 0007-number-the-exit-codes-above-2-from-3-to-8.md
  - 0008-write-structured-output-as-one-json-document.md
parent_spec: docs/product/roadmap/features/FEATURE-23.md
model:
  status: modeled
  element: C-cli
---

# ADR-0022: Use rg as the reference for the output of jag with no question

## Context

With no `--ask`, jag makes no model call and works as a plain search (0018-take-the-question-only-from-the-ask-option.md). Scripts and agents read what it writes (IF-3). After release 1, a change of that output breaks them, so the output form is expensive to change.

The accepted ADRs hold other parts of the contract: the command form (0018-take-the-question-only-from-the-ask-option.md), the exit codes above 2 (0007-number-the-exit-codes-above-2-from-3-to-8.md), and the JSON document (0008-write-structured-output-as-one-json-document.md). No ADR holds the text output with no `--ask`.

Until 2026-10-15, grep was the reference (DECISION-18, DECISION-32, DECISION-34, DECISION-38). The user answered on 2026-10-15 (hitl, FEATURE-23, Q1 option 1): "rg is the reference for jag with no question". The second answer (Q2 option 1): the parity comes before release 1.

## Decision

With no `--ask`, rg is the reference for jag. For the same arguments, jag writes the same stdout, the same stderr, and the same color codes, gives the same exit code, and matches the same lines as rg. The reference is the rg version that the rg parity tests pin. jag differs from rg only where a user `ANSWER` records a deliberate difference.

## Why

- The user chose rg (`ANSWER` of 2026-10-15). A user can then put jag in the place of rg in a command or a script.
- jag finds matches with the ripgrep libraries (0001-find-matches-with-the-ripgrep-libraries.md). So the pattern rules and the skip rules of jag are already the rules of rg. With grep as the reference, jag must translate them.
- A reference with a fixed version can be tested byte for byte in CI. With no version, "the same as rg" changes when rg changes.
- A recorded difference is known and tested. A difference with no record is a bug.

## Alternatives considered

- grep as the reference: the earlier PRD decisions. The user changed it on 2026-10-15. The pattern rules of the ripgrep libraries are not the rules of grep (RK-1).
- An output form of jag with no reference: no work to follow another tool, but no test can say what is right, and a user of rg or grep cannot put jag in its place.
- The newest rg as the reference, with no pin: jag follows rg at once, but a new rg release can make a CI run fail with no change in jag.

## Consequences

- Before release 1, jag changes each grep form to the rg form, or a user `ANSWER` records the difference (STORY-67). Known differences: the color codes, the file heading with line numbers in a terminal, `<stdin>` in place of `(standard input)`, and the program name at the start of an error line.
- The options that jag accepts are not this decision. DECISION-34 lists them, and the FEATURE-23 spec decides what jag does with `--include` and `--exclude`, which rg does not have. For each option that jag accepts, rg is the reference for its output.
- With `--json`, jag writes its own JSON document (0008-write-structured-output-as-one-json-document.md), not the JSON lines of `rg --json`. The FEATURE-23 spec decides if this is a deliberate difference.
- Where jag refuses an option that rg accepts (an option that DECISION-34 does not keep), the output of jag differs from rg. Each such case needs a recorded difference.
- A new rg pin that changes the output of rg also changes the reference. Before release 1, jag follows it. After release 1, a change of the output of jag needs a new major version of jag, or a recorded difference.
- After release 1, a change of the reference (for example back to grep) needs a new major version of jag and a new ADR.
- DECISION-33 ("a regular expression, as in grep") and the system design (IF-3, RK-1, and other lines that name grep) must say rg. The planner changes them.
