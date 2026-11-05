---
type: Story
title: Set each non-secret setting in the config file
description: As an agent builder, I want each non-secret setting of jag as a key in
  the config file, so that I set it one time, and not in each shell or job.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-05T20:31:09Z'
state: backlog
parent: /features/FEATURE-29.md
milestone: /milestones/MILESTONE-02.md
traces:
- to: prd:/capabilities/CAPABILITY-01.md
  hash: 295cfa
---

# STORY-84: Set each non-secret setting in the config file

As an agent builder (PERSONA-01), I want to set each non-secret setting of jag in `~/.config/jag/config.yml`, so that I set it one time, and not in each shell or job.

Source: request REQ-config-timeout, 2026-11-05, from the requests session. The human said: "add a story to support timeout property equivalent to JAG_TIMEOUT to the config file. jag setup should create the config file with all the properties it supports and their default values (if properties are not there already)". The answer of the user through hitl, 2026-11-05: "all possible settings that are not secrets with their defaults .. we should cover all settings that we accept via parameters also as config file settings".

## Scope

A setting is a parameter that has an option and a `JAG_` variable. These are the non-secret settings in the code on `main` (`src/config/model.rs`, `src/config/limit.rs`):

| File key | Option | Variable | Default | Valid values |
|---|---|---|---|---|
| `config.base-url` | `--base-url` | `JAG_BASE_URL` | `https://api.typesafe.ai/v1` | an https address (already a file key, ADR-0015) |
| `config.model` | `--model` | `JAG_MODEL` | `jev-1.13.0` | a model name (already a file key, ADR-0015) |
| `config.timeout` | `--timeout` | `JAG_TIMEOUT` | 4 | whole seconds from 1 to 60 |
| `config.retry-wait` | `--retry-wait` | `JAG_RETRY_WAIT` | 1 | whole seconds from 1 to 60 |
| `config.max-scored` | `--max-scored` | `JAG_MAX_SCORED` | 200 | a whole number from 1 to 2,000 |
| `config.max-chars` | `--max-chars`, `--no-max-chars` | `JAG_MAX_CHARS` | 20,000 | a whole number of 1,000 or more, or no limit |

1. This story adds the 4 new file keys `config.timeout`, `config.retry-wait`, `config.max-scored` and `config.max-chars`. Each has the meaning and the valid values of its variable.
2. For each setting, the order is the order of CV-7 and ADR-0015: the option, then the variable, then the file, then the default.
3. A bad value in the file gives exit 2, and the message has the words of the message for a bad variable, but it names the file key and the file.
4. `--setup` and `--forget-key` keep each of these keys when they write the file again, and they accept a file that has them.

Not settings, so not in scope: the keys (`config.api-key`, `config.gateway-api-key`, `JAG_TYPESAFE_API_KEY`, `TYPESAFE_API_KEY`, `JAG_GATEWAY_API_KEY`), the private patterns of DECISION-68 (a list of the user, not a parameter), and the options of one search that have no `JAG_` variable (the options of `rg` such as `-i` and `--sort`, and `--json`, `--color`, `--ask`, `--check`, `--setup`, `--forget-key`). `NO_COLOR` is a standard variable of many programs, not a setting of jag.

The spec decides the file form of "no limit" for `config.max-chars`.

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [CRITERION-151: config.timeout sets the time of one model call](../criteria/CRITERION-151.md) | - | - | - | - | - |
| [CRITERION-154: The other new file keys follow the rule of config.timeout](../criteria/CRITERION-154.md) | - | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [FEATURE-29: The config file holds each non-secret setting, and setup shows each default](../features/FEATURE-29.md)
- milestone: [MILESTONE-02: Make it good](../milestones/MILESTONE-02.md)
- traces: [CAPABILITY-01: Guided key setup and health check](../../prd/capabilities/CAPABILITY-01.md)
- criterion or run of: [CRITERION-151: config.timeout sets the time of one model call](../criteria/CRITERION-151.md)
- criterion or run of: [CRITERION-154: The other new file keys follow the rule of config.timeout](../criteria/CRITERION-154.md)
- blocks: [STORY-85: jag --setup shows each missing setting as a comment line with its default](STORY-85.md)
<!-- LINKS:END -->
