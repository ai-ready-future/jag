---
type: Area
title: Data and ownership
description: The data and ownership area of the design.
status: draft
area: data
area_state: decided
anchor: data-and-ownership
decisions:
- key: data-1
  binds: The store of the API key
  source: human:404pilo, 2026-10-12
- key: data-2
  binds: Nothing is kept between searches
  source: human:404pilo, 2026-10-12
- adr: adr:/0029-read-each-setting-from-the-config-file-and-let-setup-write.md
  key: data-3
  binds: Each setting that is not a key has a key in the config file, and setup writes each missing default as a comment line (closes DF-1)
  source: adr
open_rows: []
generated:
  by: claude-code/opus-5.5
  at: '2026-11-05T20:35:31Z'
---

# Data and ownership

<!-- LINKS:START (generated, do not edit) -->

## Links

- decisions adr: [adr:/0029-read-each-setting-from-the-config-file-and-let-setup-write.md](../../adr/0029-read-each-setting-from-the-config-file-and-let-setup-write.md)
- area: [data: Data and ownership](data.md)
- named by: [COMPONENT-02: Key and settings](../components/COMPONENT-02.md)
- named by: [data: Data and ownership](data.md)
<!-- LINKS:END -->
