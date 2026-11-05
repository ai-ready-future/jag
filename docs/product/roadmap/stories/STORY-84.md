---
type: Story
title: Set the call timeout in the config file, and let setup write each setting with
  its default
description: As an agent builder, I want config.timeout in the config file, and jag
  --setup to write each setting with its default, so that I set the timeout one time.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-05T19:51:08Z'
state: backlog
parent: /features/FEATURE-29.md
milestone: /milestones/MILESTONE-02.md
traces:
- to: prd:/capabilities/CAPABILITY-01.md
  hash: 295cfa
---

# STORY-84: Set the call timeout in the config file, and let setup write each setting with its default

As an agent builder (PERSONA-01), I want to set the time of one model call as `config.timeout` in `~/.config/jag/config.yml`, and I want `jag --setup` to write each setting that jag supports with its default value, so that I set the timeout one time and see each setting in one file.

Source: request REQ-config-timeout, 2026-11-05, from the requests session. The human said: "add a story to support timeout property equivalent to JAG_TIMEOUT to the config file. jag setup should create the config file with all the properties it supports and their default values (if properties are not there already)". The human wrote `jag setup`. The command is `jag --setup` (ADR-0006).

## Scope

1. A new file key `config.timeout`. It has the same meaning and the same valid values as `JAG_TIMEOUT`: a whole number of seconds from 1 to 60. The default is 4 seconds. The order is the order of CV-7 and ADR-0015: the option `--timeout`, then the variable `JAG_TIMEOUT`, then the file, then the default.
2. `jag --setup` adds each non-secret setting that is not in the file yet, with its default value. The request names `base-url`, `model`, `timeout` and `retry-wait`; the list and the form wait for the open points. It does not change a value that the user wrote. It writes no key (`api-key`, `gateway-api-key`) that the user did not enter in setup.
3. `jag --forget-key` keeps each of these settings, as ADR-0015 says for `config.base-url` and `config.model`.

## Open points (before the spec)

- ADR-0015 and DF-1. ADR-0015 says that jag writes no other setting to the file. DF-1 keeps the times out of the file. The architect decides amend or supersede, and the user approves the ADR change.
- The model pin of ADR-0009. Each release pins the Jev model that its threshold was tuned on. The file wins over the default (ADR-0015). If setup writes `config.model: jev-1.13.0` as a live value, a later release that pins a newer model still calls jev-1.13.0 for that user. The same is true for `config.base-url` and for a later change of a default time. The decision: write the defaults as live values, or as comment lines (for example `# timeout: 4`) that show the default but do not set it.
- `retry-wait`. The human chose to include it, so it needs a file key `config.retry-wait` with the rules of `JAG_RETRY_WAIT` (1 to 60 seconds, default 1). To confirm.
- Other settings. jag also has a cost limit and a size limit (options and variables). The request names 4 settings. To confirm if setup also writes these 2.

The criteria below do not decide the list of settings or the form of the defaults. The planner updates CRITERION-152 after the answer.

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [CRITERION-151: config.timeout sets the time of one model call](../criteria/CRITERION-151.md) | - | - | - | - | - |
| [CRITERION-152: jag --setup shows each chosen non-secret setting with its default](../criteria/CRITERION-152.md) | - | - | - | - | - |
| [CRITERION-153: A second jag --setup does not change the file](../criteria/CRITERION-153.md) | - | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [FEATURE-29: The config file holds the call timeout, and setup writes each setting](../features/FEATURE-29.md)
- milestone: [MILESTONE-02: Make it good](../milestones/MILESTONE-02.md)
- traces: [CAPABILITY-01: Guided key setup and health check](../../prd/capabilities/CAPABILITY-01.md)
- criterion or run of: [CRITERION-151: config.timeout sets the time of one model call](../criteria/CRITERION-151.md)
- criterion or run of: [CRITERION-152: jag --setup shows each chosen non-secret setting with its default](../criteria/CRITERION-152.md)
- criterion or run of: [CRITERION-153: A second jag --setup does not change the file](../criteria/CRITERION-153.md)
<!-- LINKS:END -->
