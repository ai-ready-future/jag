---
type: AcceptanceCriterion
title: jag --setup writes each non-secret setting with its default
description: After jag --setup on a new or older file, each non-secret setting is
  in the file with its default; values and keys that the user wrote stay as they are.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-05T19:50:55Z'
item: /stories/STORY-84.md
---

# CRITERION-152: jag --setup writes each non-secret setting with its default

After `jag --setup` on a new file, or on an older file that has only some of the settings, the file has each non-secret setting with its default value: `config.base-url`, `config.model`, `config.timeout` and `config.retry-wait`. A value that the user wrote stays as it is. `config.api-key` and `config.gateway-api-key` are in the file only if the user entered them, and a key that was in the file stays as it is. `jag --forget-key` keeps each of the settings.

<!-- LINKS:START (generated, do not edit) -->

## Links

- item: [STORY-84: Set the call timeout in the config file, and let setup write each setting with its default](../stories/STORY-84.md)
<!-- LINKS:END -->
