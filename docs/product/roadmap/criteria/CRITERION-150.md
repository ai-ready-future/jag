---
type: AcceptanceCriterion
title: jag from the clone runs on the machine of the human
description: After the chore guide, command -v jag shows ~/.cargo/bin/jag, and jag
  --version shows the version in Cargo.toml of the clone.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-05T14:44:16Z'
item: /stories/STORY-83.md
---

# CRITERION-150: jag from the clone runs on the machine of the human

After chores runs the guide of STORY-83 with the human, `command -v jag` shows `~/.cargo/bin/jag`, and `jag --version` shows the version in `Cargo.toml` of the clone. The guide covers a machine with no rustup or no Rust 1.99.0 toolchain (`rust-toolchain.toml`), and a `PATH` without `~/.cargo/bin`.

<!-- LINKS:START (generated, do not edit) -->

## Links

- item: [STORY-83: Install jag from a local clone with cargo, guided by chores](../stories/STORY-83.md)
<!-- LINKS:END -->
