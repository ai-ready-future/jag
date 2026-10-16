---
type: Feature
title: Check that jag is ready before the first publish
description: Before the first publish, the user uses jag on real work and fixes the
  bugs, and audits find no open high security finding and no license or compliance
  problem.
status: draft
state: backlog
github:
  number: 106
  node_id: I_kwDOVKMnL88AAAABXfzmAQ
  url: https://github.com/ai-ready-future/jag/issues/106
  pushed_hash: e7a780
  synced:
    state: backlog
parent: /epics/EPIC-05.md
milestone: /milestones/MILESTONE-02.md
traces:
- to: prd:/scope/SCOPE-01.md
  hash: 4be1a2
- to: prd:/constraints/CONSTRAINT-01.md
  hash: 1970e3
- to: prd:/constraints/CONSTRAINT-02.md
  hash: fd39da
---

# FEATURE-22: Check that jag is ready before the first publish

Before the first publish (FEATURE-18 steps U3 to U6), three checks must pass:

- The user uses jag on real work for a period, and each found bug is fixed or tracked.
- A security audit of the code, the dependencies, and the release workflow finds no open high finding.
- A license and compliance audit finds jag in good standing.

Source: user decision of 2026-10-15, given to chores-2 while step U3 was paused. The user said: "I would like to first use the app for a while locally and iterate and fix it", "I would like to audit it for security vulnerabilities", and "I would like to check the license and audit it for compliance and make sure we are in good standing". The user also said: "all these activities may deserve stories".

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [STORY-62: Use jag on real work before the first publish](../stories/STORY-62.md) | ready | - | - | - | - |
| [STORY-63: Audit jag for security vulnerabilities before the first publish](../stories/STORY-63.md) | done | - | - | - | - |
| [STORY-64: Audit the license and the compliance before the first publish](../stories/STORY-64.md) | done | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [EPIC-05: Safe public release](../epics/EPIC-05.md)
- milestone: [MILESTONE-02: Make it good](../milestones/MILESTONE-02.md)
- traces: [SCOPE-01: Release 1: first public release](../../prd/scope/SCOPE-01.md)
- traces: [CONSTRAINT-01: The public release holds no private data](../../prd/constraints/CONSTRAINT-01.md)
- traces: [CONSTRAINT-02: jag protects the API key of the user](../../prd/constraints/CONSTRAINT-02.md)
- blocks: [FEATURE-18: Install jag from GitHub Releases, crates.io, or npm](FEATURE-18.md)
- child: [STORY-62: Use jag on real work before the first publish](../stories/STORY-62.md)
- child: [STORY-63: Audit jag for security vulnerabilities before the first publish](../stories/STORY-63.md)
- child: [STORY-64: Audit the license and the compliance before the first publish](../stories/STORY-64.md)
<!-- LINKS:END -->
