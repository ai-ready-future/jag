---
type: AcceptanceCriterion
title: config.timeout sets the time of one model call
description: With config.timeout in the file and no --timeout and no JAG_TIMEOUT,
  one model call uses that time; a bad value gives exit 2 with the words of a bad
  JAG_TIMEOUT.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-05T19:50:55Z'
item: /stories/STORY-84.md
---

# CRITERION-151: config.timeout sets the time of one model call

With `config.timeout` in `~/.config/jag/config.yml`, and with no `--timeout` and no `JAG_TIMEOUT`, one model call uses the time in the file. The order is the option `--timeout`, then the variable `JAG_TIMEOUT`, then `config.timeout`, then the default of 4 seconds (CV-7, ADR-0015). A value that is not a whole number of seconds from 1 to 60 gives exit 2, and the message has the same words as the message for a bad `JAG_TIMEOUT`, but it names `config.timeout` and the file. For example: "config.timeout in <path> is not a whole number of seconds from 1 to 60."

<!-- LINKS:START (generated, do not edit) -->

## Links

- item: [STORY-84: Set the call timeout in the config file, and let setup write each setting with its default](../stories/STORY-84.md)
<!-- LINKS:END -->
