---
type: AcceptanceCriterion
title: jag --setup adds a comment line with the default for each missing setting
description: After jag --setup on a new or older file, each non-secret setting is
  in the file, as a value or as a comment line with its default; values and keys stay
  as they are.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-05T20:31:09Z'
item: /stories/STORY-85.md
---

# CRITERION-152: jag --setup adds a comment line with the default for each missing setting

After `jag --setup` on a new file, or on an older file that has only some of the settings, the file has 1 comment line with the default value for each non-secret setting of STORY-84 that the file did not have as a value or as a comment line: `base-url`, `model`, `timeout`, `retry-wait`, `max-scored` and `max-chars`. A value that the user wrote stays as it is. `config.api-key` and `config.gateway-api-key` are in the file only if the user entered them, and a key that was in the file stays as it is. With only the comment line of a setting in the file, jag uses the default of that setting. `jag --forget-key` keeps each comment line and each setting.

<!-- LINKS:START (generated, do not edit) -->

## Links

- item: [STORY-85: jag --setup shows each missing setting as a comment line with its default](../stories/STORY-85.md)
<!-- LINKS:END -->
