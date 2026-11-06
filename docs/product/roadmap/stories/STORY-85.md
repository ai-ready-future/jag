---
type: Story
title: jag --setup shows each missing setting as a comment line with its default
description: As an agent builder, I want jag --setup to show each non-secret setting
  in the config file as a comment line with its default, so that I see each setting.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-05T20:31:09Z'
state: ready
github:
  number: 350
  node_id: I_kwDOVKMnL88AAAABc2mtIQ
  url: https://github.com/ai-ready-future/jag/issues/350
  pushed_hash: 4a6a4d
  synced:
    state: blocked
spec: docs/superpowers/specs/2026-11-05-STORY-84-each-setting-in-the-config-file-design.md
parent: /features/FEATURE-29.md
blocked_by:
- /stories/STORY-84.md
milestone: /milestones/MILESTONE-02.md
traces:
- to: prd:/capabilities/CAPABILITY-01.md
  hash: 295cfa
---

# STORY-85: jag --setup shows each missing setting as a comment line with its default

As an agent builder (PERSONA-01), I want `jag --setup` to show each non-secret setting in the config file with its default value, so that I see in one file each setting that I can change.

Source: request REQ-config-timeout, 2026-11-05, and the answer of the user through hitl, 2026-11-05: the defaults are comment lines (option A). The reason: each release pins its Jev model (ADR-0009), and a value in the file wins over the default (ADR-0015). A comment line shows the default, but does not set it, so a later release can still change the default.

## Scope

1. `jag --setup` adds 1 comment line for each non-secret setting of STORY-84 that is not in the file yet, with its default value, for example `# timeout: 4` in the `config` section.
2. A setting is "in the file" when the file has it as a value or as a comment line. Setup does not add a second line for it.
3. Setup does not change a value that the user wrote. It writes a key (`api-key`, `gateway-api-key`) only when the user enters it in setup. A key that was in the file stays as it is.
4. A comment line is not a value: with only the comment line, jag uses the default.
5. `jag --setup` and `jag --forget-key` keep each comment line when they write the file again.

This story needs STORY-84, because the comment lines show the file keys of STORY-84.

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [CRITERION-152: jag --setup adds a comment line with the default for each missing setting](../criteria/CRITERION-152.md) | - | - | - | - | - |
| [CRITERION-153: A second jag --setup does not change the file](../criteria/CRITERION-153.md) | - | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [FEATURE-29: The config file holds each non-secret setting, and setup shows each default](../features/FEATURE-29.md)
- blocked by: [STORY-84: Set each non-secret setting in the config file](STORY-84.md)
- milestone: [MILESTONE-02: Make it good](../milestones/MILESTONE-02.md)
- traces: [CAPABILITY-01: Guided key setup and health check](../../prd/capabilities/CAPABILITY-01.md)
- criterion or run of: [CRITERION-152: jag --setup adds a comment line with the default for each missing setting](../criteria/CRITERION-152.md)
- criterion or run of: [CRITERION-153: A second jag --setup does not change the file](../criteria/CRITERION-153.md)
- blocks: [STORY-93: Turn the private-pattern check off with one setting](STORY-93.md)
<!-- LINKS:END -->
