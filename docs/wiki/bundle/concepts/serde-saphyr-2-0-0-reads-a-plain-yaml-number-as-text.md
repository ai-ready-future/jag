---
type: Concept
title: serde-saphyr 2.0.0 reads a plain YAML number as text
description: With default options, serde-saphyr 2.0.0 reads a plain scalar such as 4, 04, 0x0A, 4.0 or true into an Option<String> field as its text. An empty value or ~ gives None. A !!int tag, a list, or a duplicate key gives an error.
status: draft
generated:
  by: claude-code/sonnet-5.5
  at: '2026-11-06T01:50:39Z'
---

# serde-saphyr 2.0.0 reads a plain YAML number as text

Result of a probe, learned on 2026-11-06 from the specifier (FEATURE-29 spec, PR #371, Assumption A1). Veracity 60: a probe with a source in the code of the crate.

## Fact

serde-saphyr is the YAML crate of jag (ADR-0012). With default options, version 2.0.0 reads a plain YAML scalar into an `Option<String>` field as its text. Example: the line `timeout: 4`.

| YAML value | Result |
|---|---|
| `4` | the text "4" |
| `04` | the text "04" |
| `0x0A` | the text "0x0A" |
| `4.0` | the text "4.0" |
| `true` | the text "true" |
| nothing after the key (`timeout:`) | None |
| `~` | None |
| `!!int 4` | an error |
| `[4]` | an error |
| a duplicate key | an error |

## Source

- A probe in Docker with the image `rust:1.99`, on 2026-11-05.
- The function `deserialize_string` in `src/de/deserializer.rs` of serde-saphyr 2.0.0. The option `no_schema` is false by default.
