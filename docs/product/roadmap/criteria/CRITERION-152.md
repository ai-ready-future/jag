---
type: AcceptanceCriterion
title: jag --setup shows each chosen non-secret setting with its default
description: After jag --setup on a new or older file, the file shows each non-secret
  setting that the answer names, with its default, in the chosen form; values and
  keys that the user wrote stay as they are.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-05T20:26:06Z'
item: /stories/STORY-84.md
---

# CRITERION-152: jag --setup shows each chosen non-secret setting with its default

After `jag --setup` on a new file, or on an older file that has only some of the settings, the file shows each non-secret setting that the answer to the open points of STORY-84 names, with its default value, in the form that the answer chooses (a live value or a comment line). A value that the user wrote stays as it is. `config.api-key` and `config.gateway-api-key` are in the file only if the user entered them, and a key that was in the file stays as it is. `jag --forget-key` keeps each of the settings.

<!-- LINKS:START (generated, do not edit) -->

## Links

- item: [STORY-84: Set the call timeout in the config file, and let setup write each setting with its default](../stories/STORY-84.md)
<!-- LINKS:END -->
