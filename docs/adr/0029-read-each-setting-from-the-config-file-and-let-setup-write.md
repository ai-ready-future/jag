---
type: adr
title: Read each setting from the config file, and let setup write each default as a comment line
description: Open this when you add a setting, change the keys of the config file, or change what --setup and --forget-key write or keep in it.
area: data
status: proposed
date: 2026-11-05
supersedes: 0015-read-the-base-url-and-the-model-name-from-config-base-url.md # partial
superseded_by: null
amends: null
refines: null
deviates_from: null
related: [0009-pin-the-jev-model-version-in-each-release.md, 0012-use-clap-serde-saphyr-and-rpassword-as-the-outside-crates.md, 0017-use-a-separate-key-for-an-ai-gateway.md]
parent_spec: null
model:
  status: modeled
  element: C-config
  reason: C-config reads the config file, gives each setting its source, and writes the file in --setup and --forget-key
---

# ADR-0029: Read each setting from the config file, and let setup write each default as a comment line

## Context

ADR-0015 puts `config.base-url` and `config.model` in `~/.config/jag/config.yml`, and says that jag writes no other setting to the file, except the gateway key of ADR-0017. DF-1 of the system design keeps the times and the limits out of the file. Its trigger is "a user asks for another default that they can save". Request REQ-config-timeout (STORY-84) asks for `config.timeout`, and asks `jag --setup` to write the defaults. The user answered the two open points (hitl `ANSWER`, 2026-11-05):

- Q1, option A: setup writes each default as a comment line, for example `# timeout: 4` and `# model: jev-1.13.0`, so that a new release can still change a default.
- Q2: "all possible settings that are not secrets with their defaults .. we should cover all settings that we accept via parameters also as config file settings".

The keys of the file are a contract with the user (ADR-0015).

## Decision

A setting is a value that jag reads from an option and from a `JAG_` variable, and that has a default. Each setting that is not a key has a key in the `config` section of the file. The name of the key is the long option name with no `--`:

| Key | Option | Variable | Default |
|---|---|---|---|
| `config.base-url` | `--base-url` | `JAG_BASE_URL` | `https://api.typesafe.ai/v1` |
| `config.model` | `--model` | `JAG_MODEL` | `jev-1.13.0` |
| `config.timeout` | `--timeout` | `JAG_TIMEOUT` | `4` |
| `config.retry-wait` | `--retry-wait` | `JAG_RETRY_WAIT` | `1` |
| `config.max-scored` | `--max-scored` | `JAG_MAX_SCORED` | `200` |
| `config.max-chars` | `--max-chars` | `JAG_MAX_CHARS` | `20000` |

The defaults in this table are the values of the code on 2026-11-05. A release can change a default.

The key of a setting works as its variable. jag reads the key in each flow that reads the variable, and with the same text rules. The order is the option, then the variable, then the file, then the default (CV-7). A bad value in the file gives the usage error of a bad variable, with exit 2. The message names the key, and, as the message for a bad variable, it never shows the value. A file that is not valid YAML, or that other users can read, still gives exit 3 (ADR-0015).

`jag --setup` writes 1 comment line for each setting that has no value in the file, in the form `# <name>: <default>`, under `config:`. A value that the user wrote stays as it is. A comment line is not a value. `--setup` and `--forget-key` keep each comment line. A second `--setup` of the same release does not change the file. The keys `api-key` and `gateway-api-key` get no comment line. jag writes a key only when the user enters it.

These items are not settings by this rule, and setup writes no comment line for them: the keys, the options of 1 search that have no `JAG_` variable (the rg options, `--ask`, `--json`, `--color`, `--no-max-chars`), and `NO_COLOR`. The private patterns of DECISION-68 are a list of the user, not a setting. This ADR does not decide their place in the file; the spec of CAPABILITY-14 decides it. A setting that a later feature adds, for example the switch of DECISION-80, gets a key by this rule when it has an option, a `JAG_` variable, and a default.

This ADR replaces the sentence "jag writes no other setting to the file, except the gateway key of ADR-0017" of ADR-0015. The other parts of ADR-0015 stay: the nested `config.` form, the keys `config.base-url` and `config.model`, and the rule that `--setup` and `--forget-key` keep each value of the user.

## Why

- The user asked for a file key for each setting that an option or a variable gives (Q2). With 1 rule for the list, a new setting gets a key with no new ADR.
- A comment line shows each setting and its default, and the user removes `# ` to set it. jag reads no value from it. So the default of a new release applies, also for the model that ADR-0009 pins in each release (Q1).
- The key works as its variable, so the user learns 1 set of value rules and 1 error for each setting.
- Setup never changes a value that the user wrote (FEATURE-01 spec decision 18), and a second setup gives the same file.

## Alternatives considered

- Keep ADR-0015 and DF-1, with options and variables only: no change to the file. But the user must set a variable in each shell and in each job, and the request asks for the file.
- Setup writes each default as a value, for example `timeout: 4`: the file is easy to read. But the file wins over the default, so a user keeps the old model and the old limits after an update, and ADR-0009 fails for that user (Q1, option B).
- Only `config.timeout`: the smallest change. But the user asked for each setting (Q2), and each later setting needs a new ADR.

## Consequences

- To remove or rename a key of the table breaks the files of users. Such a change needs a new major version of jag and a new ADR. A new setting gets its key by the rule of this ADR.
- The strict parse of `--setup` and `--forget-key` must know each key of the table. Else a user who removes `# ` from `# timeout: 4` gets the refusal for an unknown key (exit 2).
- The struct of the file must take a YAML number for `timeout`, `retry-wait`, `max-scored`, and `max-chars`, for example as text for the parse of the variable. Else `timeout: 4` gives a YAML type error (exit 3) and not the usage error of a bad value (exit 2).
- ADR-0012: `serde-saphyr` reads the file and drops each comment. jag does not write the file with `serde-saphyr`. `file_text` in `src/config/change.rs` writes the whole file again from the values, so today each comment line goes away at the next `--setup` or `--forget-key`. The write must change: it keeps each line of the file that it does not change, also the comment lines of the user, adds only the missing comment lines, and changes only the line of a key that it writes. `serde-saphyr` stays the reader. A comment line must be indented under `config:`, and a `config:` section that holds only comment lines reads as no value.
- A comment line keeps the default of the release that wrote it. After an update, the line can show an old default, and the value that jag uses is the new default. The spec decides if a later `--setup` changes such a line. A second `--setup` of the same release does not change the file.
- A flow that reads a variable of the table now also reads the file. For example, `--json` with no `--ask` reads `JAG_MAX_CHARS`, so it reads `config.max-chars` and the file checks apply to it.
- `JAG_MAX_CHARS` has no value for "no limit"; only `--no-max-chars` gives it. The spec decides the file form of "no limit" for `config.max-chars`.
- DF-1 of the system design is closed, and the Settings row names this ADR.
