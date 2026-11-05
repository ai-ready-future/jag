---
type: AcceptanceCriterion
title: The other new file keys follow the rule of config.timeout
description: config.retry-wait, config.max-scored and config.max-chars follow the
  order and the error rule of config.timeout, and setup and forget-key keep them.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-05T20:31:09Z'
item: /stories/STORY-84.md
---

# CRITERION-154: The other new file keys follow the rule of config.timeout

The rule of CRITERION-151 is also true for `config.retry-wait`, `config.max-scored` and `config.max-chars`. With the key in the file, and with no option and no variable for it, jag uses the value in the file. The order is the option, then the variable, then the file, then the default (CV-7, ADR-0015). A bad value gives exit 2, and the message has the words of the message for a bad `JAG_RETRY_WAIT`, `JAG_MAX_SCORED` or `JAG_MAX_CHARS`, but it names the file key and the file. `jag --setup` and `jag --forget-key` accept a file with these keys, and keep them.

<!-- LINKS:START (generated, do not edit) -->

## Links

- item: [STORY-84: Set each non-secret setting in the config file](../stories/STORY-84.md)
<!-- LINKS:END -->
