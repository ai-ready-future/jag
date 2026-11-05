---
type: Story
title: Install jag from a local clone with cargo, guided by chores
description: As the maintainer, I want the chores session to guide me through the
  install of jag with cargo from a local clone, so that I can run the jag of the current
  code on my machine.
status: draft
generated:
  by: prd/6.2.1
  at: '2026-11-05T14:44:11Z'
state: done
actual_end_date: '2026-11-05'
github:
  number: 343
  node_id: I_kwDOVKMnL88AAAABczRgCA
  url: https://github.com/ai-ready-future/jag/issues/343
  pushed_hash: 197cc6
  synced:
    state: done
    owner: human:404pilo
parent: /epics/EPIC-05.md
milestone: /milestones/MILESTONE-02.md
owner: human:404pilo
traces:
- to: prd:/scope/SCOPE-01.md
  hash: 4be1a2
---

# STORY-83: Install jag from a local clone with cargo, guided by chores

As the maintainer, I want the chores session to guide me step by step through the install of the jag CLI with cargo from a local clone of this repository, so that I can run the jag of the current code on my machine.

This story needs no build and no spec. The deliverable is the chore guide below. The chores session reads this page and runs the guide with the human, 1 step at a time. It gives each command with the `$ ` prefix and checks the result before the next step. No new role is necessary.

Source: request REQ-install-from-source, 2026-11-05, from the requests session. The human said: "add a new chores station that will help guide me to install jag cli locally with cargo from code". The human chose 1 chore guide, not a new role and not only a docs page.

Scope: the install from source only. The README and ADR-0026 give the install from crates.io (`cargo install --locked jag-cli`). This guide does not change them.

## Chore guide: install jag from a local clone with cargo

Before step 1, ask the human for the path of the clone. In the commands below, `<clone>` is that path. If the human has no clone, use step 0.

0. Get a clone (only if the human has no clone).

   ```
   $ git clone https://github.com/ai-ready-future/jag.git <clone>
   ```

   Result: the folder `<clone>` has the file `rust-toolchain.toml`.

1. Go to the clone and get the latest code.

   ```
   $ cd <clone>
   $ git pull --ff-only
   ```

   Result: git shows no error. If git shows local changes or a different branch, ask the human which code to install. Do not reset or discard their changes.

2. Check for rustup. Check rustup, not only cargo: a cargo from Homebrew or from the system package does not read `rust-toolchain.toml`, and the build needs Rust 1.99 (`rust-version` in `Cargo.toml`).

   ```
   $ rustup --version
   ```

   - If rustup shows a version, go to step 4.
   - If the shell shows `command not found`, but `cargo --version` works, the human has a cargo without rustup. Tell the human. Install rustup in step 3. The human can remove the other Rust later.

3. Install rustup (only if step 2 found no rustup). This is the command of rustup.rs for macOS and Linux:

   ```
   $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

   The human accepts the default install. Then load the cargo folder into this shell:

   ```
   $ . "$HOME/.cargo/env"
   $ rustup --version
   ```

   Result: rustup shows a version.

4. Install the toolchain of the clone. Run this in the clone, because `rust-toolchain.toml` selects Rust 1.99.0 there. Some rustup versions do not install the active toolchain automatically, so this command installs it if it is missing:

   ```
   $ rustup show active-toolchain || rustup toolchain install
   $ rustc --version
   ```

   Result: `rustc --version` shows `1.99.0`.

5. Check the linker. On macOS, the build needs the Command Line Tools:

   ```
   $ xcode-select -p
   ```

   If this shows an error, run `xcode-select --install`, and wait until the human completes the install window. On Linux, the build needs a C compiler and linker, for example the package `build-essential` on Debian or Ubuntu.

6. Build and install jag from the clone.

   ```
   $ cargo install --locked --path .
   ```

   Result: the last line of cargo starts with `Installed package` and names the executable `jag`. The build takes some minutes. Cargo does not build `jag-bench`, because it needs the feature `bench`. If cargo shows that `jag` is already installed from another source, run the command again with `--force`. Before that, tell the human that `--force` replaces the jag that cargo installed from crates.io (`cargo install jag-cli`) with the jag of the clone.

7. Make sure that `~/.cargo/bin` is in `PATH`.

   ```
   $ hash -r
   $ command -v jag
   ```

   `hash -r` clears the command paths that the shell keeps, so that the shell finds the new jag.

   - If this shows `<home>/.cargo/bin/jag`, go to step 8.
   - If it shows nothing, `~/.cargo/bin` is not in `PATH`. Load it into this shell:

     ```
     $ . "$HOME/.cargo/env"
     ```

     Then add it to the start file of the shell of the human, so that each new shell has it. Check the file first, and add the line only if the file does not have it:

     ```
     $ grep -F '.cargo/env' ~/.zshrc || echo '. "$HOME/.cargo/env"' >> ~/.zshrc     # zsh, the default shell of macOS
     $ grep -F '.cargo/env' ~/.bashrc || echo '. "$HOME/.cargo/env"' >> ~/.bashrc   # bash
     ```

     The human runs only the line of their shell. Then run `hash -r` and `command -v jag` again.
   - If it shows a different path (for example an npm, Homebrew or `~/.local/bin` jag), an older jag comes first in `PATH`. Show all of them with `which -a jag`. Ask the human to remove the older jag, or to put `~/.cargo/bin` first in `PATH`.

8. Check the result (the done check).

   ```
   $ hash -r
   $ command -v jag
   $ jag --version
   $ grep '^version' <clone>/Cargo.toml
   ```

   Result: `command -v jag` shows `<home>/.cargo/bin/jag`, and the version of `jag --version` is the version in `Cargo.toml` of the clone. Both are necessary: the version alone does not show which binary runs, because the crates.io and npm packages can have the same version.

9. Tell the human how to update later: in the clone, run `git pull --ff-only`, then step 6 again.

## Result

On 2026-11-05 chores ran the guide with the human on a Mac, in the main clone at `main` `463bde2`. CRITERION-150 passes, and no step failed:

- Step 0: not necessary. The clone was on `main` and had no local changes.
- Steps 1, 2, 4 and 5: no problem. rustup 1.27.1 was installed. `rust-toolchain.toml` gave `rustc` 1.99.0. The Command Line Tools were installed.
- Step 6: `cargo install --locked --path .` installed `jag`. `--force` was not necessary.
- Step 7: `~/.cargo/bin` was already in `PATH`.
- Step 8: `command -v jag` shows `~/.cargo/bin/jag`. `jag --version` shows 0.1.0, the version in `Cargo.toml`. `cargo install --list` shows the path of the clone.

Not tested on this machine, because they were not necessary: step 3 (the install of rustup) and the change of the shell start file in step 7.

<!-- STATUS:START (generated, do not edit) -->

## Status

| Item | State | Progress | Owner | Blocked by | Drift |
| --- | --- | --- | --- | --- | --- |
| [CRITERION-150: jag from the clone runs on the machine of the human](../criteria/CRITERION-150.md) | - | - | - | - | - |
<!-- STATUS:END -->

<!-- LINKS:START (generated, do not edit) -->

## Links

- parent: [EPIC-05: Safe public release](../epics/EPIC-05.md)
- milestone: [MILESTONE-02: Make it good](../milestones/MILESTONE-02.md)
- traces: [SCOPE-01: Release 1: first public release](../../prd/scope/SCOPE-01.md)
- criterion or run of: [CRITERION-150: jag from the clone runs on the machine of the human](../criteria/CRITERION-150.md)
<!-- LINKS:END -->
