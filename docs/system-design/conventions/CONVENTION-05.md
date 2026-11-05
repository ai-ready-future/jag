---
type: Convention
title: Each Jev call has a timeout and 1 retry, and 1 deadline covers the search
description: Each Jev call times out after 4 seconds and gets 1 retry, and 1 deadline covers all calls of a search.
status: draft
rule: 'Each Jev call times out after 4 seconds. After a timeout, a failed connection, HTTP 429, or HTTP 5xx, jag waits 1 second and tries one more time. Then it fails with code 6. The `--timeout` option, the `JAG_TIMEOUT` variable, and the file key `config.timeout` change the time of one call. The `--retry-wait` option, the `JAG_RETRY_WAIT` variable, and the file key `config.retry-wait` change the wait before the retry, in the order of CV-7 (0029-read-each-setting-from-the-config-file-and-let-setup-write.md). Both take whole seconds from 1 to 60. One deadline covers all calls of a search: 2 × the timeout + the retry wait from the first call. No call and no retry starts after it. The deadline is 9 seconds with the defaults, and for example 180 seconds for 60 and 60 (conventions-5, QS-4, CONSTRAINT-04).'
governed_by:
- adr:/0029-read-each-setting-from-the-config-file-and-let-setup-write.md
aliases:
- CV-5
generated:
  by: claude-code/opus-5.5
  at: '2026-11-05T21:22:09Z'
---

# CONVENTION-05: Each Jev call has a timeout and 1 retry, and 1 deadline covers the search

Each Jev call times out after 4 seconds. After a timeout, a failed connection, HTTP 429, or HTTP 5xx, jag waits 1 second and tries one more time. Then it fails with code 6. The `--timeout` option, the `JAG_TIMEOUT` variable, and the file key `config.timeout` change the time of one call. The `--retry-wait` option, the `JAG_RETRY_WAIT` variable, and the file key `config.retry-wait` change the wait before the retry, in the order of CV-7 (0029-read-each-setting-from-the-config-file-and-let-setup-write.md). Both take whole seconds from 1 to 60. One deadline covers all calls of a search: 2 × the timeout + the retry wait from the first call. No call and no retry starts after it. The deadline is 9 seconds with the defaults, and for example 180 seconds for 60 and 60 (conventions-5, QS-4, CONSTRAINT-04).

<!-- LINKS:START (generated, do not edit) -->

## Links

- governed by: [adr:/0029-read-each-setting-from-the-config-file-and-let-setup-write.md](../../adr/0029-read-each-setting-from-the-config-file-and-let-setup-write.md)
- governs: [COMPONENT-05: Jev scorer](../components/COMPONENT-05.md)
- governs: [COMPONENT-07: Readiness check](../components/COMPONENT-07.md)
<!-- LINKS:END -->
