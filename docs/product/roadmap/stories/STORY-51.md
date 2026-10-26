---
type: Story
title: Publish each channel from one tag
description: As an agent builder (PERSONA-01), I want one version tag to publish jag
  on each channel with the same version and no private data, so that each channel
  gives the same jag.
status: draft
state: blocked
github:
  number: 76
  node_id: I_kwDOVKMnL88AAAABXTMxNQ
  url: https://github.com/ai-ready-future/jag/issues/76
  pushed_hash: e5b7dd
  synced:
    state: blocked
spec: docs/superpowers/specs/2026-10-14-STORY-48-install-jag-from-github-releases-crates-io-or-npm-design.md
parent: /features/FEATURE-18.md
blocked_by:
- /stories/STORY-45.md
milestone: /milestones/MILESTONE-02.md
traces:
- to: prd:/constraints/CONSTRAINT-01.md
  hash: 1970e3
---

# STORY-51: Publish each channel from one tag

As an agent builder (PERSONA-01), I want one version tag to publish jag on each channel with the same version and no private data, so that each channel gives the same jag.

Source of `blocked_by` STORY-45: the user, hitl answer of 2026-10-26 (STORY-45, the held-out evaluation, is done before the first `v*` tag; EPIC-06 can follow release 1). The first tag is user step U6 of the spec.

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [CRITERION-82: One tag publishes each channel with the same version](../criteria/CRITERION-82.md) | - | - | - | - | - |
| [CRITERION-83: The repository stores no publish token](../criteria/CRITERION-83.md) | - | - | - | - | - |
| [CRITERION-84: The published packages hold no private data](../criteria/CRITERION-84.md) | - | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [FEATURE-18: Install jag from GitHub Releases, crates.io, or npm](../features/FEATURE-18.md)
- blocked by: [STORY-45: Run the held-out evaluation before release](STORY-45.md)
- milestone: [MILESTONE-02: Make it good](../milestones/MILESTONE-02.md)
- traces: [CONSTRAINT-01: The public release holds no private data](../../prd/constraints/CONSTRAINT-01.md)
- criterion or run of: [CRITERION-82: One tag publishes each channel with the same version](../criteria/CRITERION-82.md)
- criterion or run of: [CRITERION-83: The repository stores no publish token](../criteria/CRITERION-83.md)
- criterion or run of: [CRITERION-84: The published packages hold no private data](../criteria/CRITERION-84.md)
<!-- LINKS:END -->
