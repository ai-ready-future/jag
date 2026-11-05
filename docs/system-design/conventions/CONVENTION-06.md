---
type: Convention
title: Only C-config reads the key, and the key never shows
description: Only C-config reads the key, the key never shows, the config file must be private, and jag sends each key only on its own route.
status: draft
rule: 'Only C-config reads the key. The key lives in a type that never prints itself in a message, an error, or debug text. No option takes the key, so it never shows in the process list. `--setup` reads it from the terminal with no echo. jag refuses a config file that the group or other users can read, and exits with code 3 and the fix. This check applies to each read of the file, also a read for a setting only, when the key comes from a variable or when jag needs no key. jag sends a key only on its own route: a key from `JAG_TYPESAFE_API_KEY`, `TYPESAFE_API_KEY`, or `config.api-key` goes only to the TypeSafe API, and only a key from `JAG_GATEWAY_API_KEY` or `config.gateway-api-key` goes to a gateway. With no key for the route, the search stops before the call with exit 3 (conventions-6, QS-5, CONSTRAINT-02, 0015-read-the-base-url-and-the-model-name-from-config-base-url.md, 0017-use-a-separate-key-for-an-ai-gateway.md).'
aliases:
- CV-6
generated:
  by: claude-code/opus-5.5
  at: '2026-11-05T21:21:04Z'
---

# CONVENTION-06: Only C-config reads the key, and the key never shows

Only C-config reads the key. The key lives in a type that never prints itself in a message, an error, or debug text. No option takes the key, so it never shows in the process list. `--setup` reads it from the terminal with no echo. jag refuses a config file that the group or other users can read, and exits with code 3 and the fix. This check applies to each read of the file, also a read for a setting only, when the key comes from a variable or when jag needs no key. jag sends a key only on its own route: a key from `JAG_TYPESAFE_API_KEY`, `TYPESAFE_API_KEY`, or `config.api-key` goes only to the TypeSafe API, and only a key from `JAG_GATEWAY_API_KEY` or `config.gateway-api-key` goes to a gateway. With no key for the route, the search stops before the call with exit 3 (conventions-6, QS-5, CONSTRAINT-02, 0015-read-the-base-url-and-the-model-name-from-config-base-url.md, 0017-use-a-separate-key-for-an-ai-gateway.md).

<!-- LINKS:START (generated, do not edit) -->

## Links

- governs: [COMPONENT-02: Key and settings](../components/COMPONENT-02.md)
- governs: [COMPONENT-05: Jev scorer](../components/COMPONENT-05.md)
<!-- LINKS:END -->
