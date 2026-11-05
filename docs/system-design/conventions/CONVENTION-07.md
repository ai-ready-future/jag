---
type: Convention
title: A setting takes its value from the option, the variable, the file, or the default
description: A setting takes its value from the option, then the variable, then the config file, then the default.
status: draft
rule: 'A setting takes its value from the first of these sources: the command option, the environment variable, the config file, and the built-in default. The key has no option. DECISION-03 gives the order of the TypeSafe key, and 0017-use-a-separate-key-for-an-ai-gateway.md gives the order of the gateway key (conventions-7).'
governed_by:
- adr:/0017-use-a-separate-key-for-an-ai-gateway.md
aliases:
- CV-7
generated:
  by: claude-code/opus-5.5
  at: '2026-11-05T21:22:10Z'
---

# CONVENTION-07: A setting takes its value from the option, the variable, the file, or the default

A setting takes its value from the first of these sources: the command option, the environment variable, the config file, and the built-in default. The key has no option. DECISION-03 gives the order of the TypeSafe key, and 0017-use-a-separate-key-for-an-ai-gateway.md gives the order of the gateway key (conventions-7).

<!-- LINKS:START (generated, do not edit) -->

## Links

- governed by: [adr:/0017-use-a-separate-key-for-an-ai-gateway.md](../../adr/0017-use-a-separate-key-for-an-ai-gateway.md)
- governs: [COMPONENT-01: Command line](../components/COMPONENT-01.md)
- governs: [COMPONENT-02: Key and settings](../components/COMPONENT-02.md)
<!-- LINKS:END -->
