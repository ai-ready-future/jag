---
type: adr
title: Name the crates.io package jag-cli
description: Open this when you change the crates.io name of jag, the cargo install command, or the name that the release publishes to crates.io.
area: building-blocks
status: proposed
date: 2026-10-20
supersedes: 0011-build-jag-as-one-cargo-package-with-one-module-per-component.md  # partial: the crates.io name only
superseded_by: null
amends: null
refines: null
deviates_from: null
related:
  - 0010-ship-jag-on-github-releases-crates-io-and-npm.md
  - 0024-install-the-jag-agent-skill-from-the-repository-with-the.md
parent_spec: docs/product/system-design.md#building-blocks
model:
  status: modeled
  element: C-cli
---

# ADR-0026: Name the crates.io package jag-cli

## Context

0011-build-jag-as-one-cargo-package-with-one-module-per-component.md gives the one Cargo package of jag the crates.io name `jag-grep`. The crates.io name `jag` belongs to another crate. The user gave these answers:

- Hitl `ANSWER` of 2026-10-20 on the card that accepts 0024-install-the-jag-agent-skill-from-the-repository-with-the.md: "accept . can it be jag-cli instead of jag-grep?"
- Hitl `ANSWER` of 2026-10-20, card "The crate name: jag-grep or jag-cli?": "Rename to jag-cli".

Facts that the decision uses:

- On 2026-10-20 the crates.io API gives "not found" for `jag-cli` and for `jag-grep`. jag has not published `jag-grep`, and the first publish of an empty version 0.0.0 (`scripts/release/first-publish.sh`) has not run.
- Only the package name is `jag-grep`. The library and the command are both `jag` (`[lib]` and `[[bin]]` in `Cargo.toml`), and no Rust code names `jag_grep`.
- crates.io keeps a published crate. It lets the owner delete a crate only in some cases, for example in the first 72 hours after the publish. So after the first publish, a new name usually means a second crate, and the old crate stays.

## Decision

- The one Cargo package of jag has the crates.io name `jag-cli`. Rust users install jag with `cargo install --locked jag-cli`.
- The library and the command stay `jag`. The npm name stays `@airf/jag`.
- The first release publishes `jag-cli`. No release publishes `jag-grep`, also not as an empty version.
- The other parts of 0011-build-jag-as-one-cargo-package-with-one-module-per-component.md do not change: one package, one module for each component, the edition, the pinned toolchain, and the checks.

## Why

- These are the words of the user.
- The name is free, and jag has not published a crate yet, so the change is one pull request with no second crate to keep.
- `jag-cli` is the usual form of a crate that gives the command of a tool when the short name is taken.

## Alternatives considered

- Keep `jag-grep`: no change, and the name says that jag is a grep tool, but the user chose `jag-cli`.
- `jagrep`: free, but it looks like a different command.

## Consequences

- One code pull request changes the package name in `Cargo.toml` and `Cargo.lock`, the crate name in the release workflow and the release scripts and their tests, the cargo command in the npm launcher message and its test, and the README. It must merge before the first release.
- The trusted publishing setting on crates.io and the first empty publish use the name `jag-cli`.
- The crate file of a release is `jag-cli-<version>.crate`.
- The crates.io search for "grep" still finds jag by the keyword `grep` in `Cargo.toml`.
- A new crates.io name after the first release needs a new ADR and leaves the old crate in place.
