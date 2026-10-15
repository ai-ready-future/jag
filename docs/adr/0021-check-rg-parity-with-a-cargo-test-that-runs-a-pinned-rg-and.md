---
type: adr
title: Check rg parity with a cargo test that runs a pinned rg and jag
description: Open this when you add or change an rg parity case, a fixture, a deliberate difference, the rg pin, or how the parity tests run in CI.
area: quality
status: proposed
date: 2026-10-15
supersedes: null
superseded_by: null
amends: null
refines: null
deviates_from: null
related:
  - 0001-find-matches-with-the-ripgrep-libraries.md
  - 0012-use-clap-serde-saphyr-and-rpassword-as-the-outside-crates.md
  - 0020-name-the-release-archives-and-the-npm-platform-packages-by.md
parent_spec: docs/product/roadmap/features/FEATURE-23.md
model:
  status: out-of-scope
  reason: the parity tests are not an element of the system design
---

# ADR-0021: Check rg parity with a cargo test that runs a pinned rg and jag

## Context

FEATURE-23: an rg command and a jag command with the same arguments and no question must give the same stdout, the same stderr, the same color, and the same exit code. Before release 1, the user chose rg as the reference for jag. The parity cases must run again as e2e tests in CI on each pull request, with rg at a fixed version and with no rg config, in a pipe and with a pseudo terminal.

Each later parity case, fixture, and recorded difference uses the form that this ADR sets. A change of the form later changes each case. So the form is expensive to change.

Facts that the decision uses:

- `Cargo.toml` sets `unsafe_code = "forbid"`. This lint also applies to the test targets.
- `rustix` is already a dependency of jag. Its `pty` and `termios` features open and set a pseudo terminal with a safe API, on Linux and on macOS.
- rg 15.2.0 prints the files in a different order in each run when it searches more than one file with more than one thread (5 runs on 40 files gave 5 different outputs). With `--sort path`, the order is fixed.
- jag and rg use the same test for stdin: they read stdin only if it is not a terminal and it is a file, a pipe, or a socket (`src/search/input.rs`). So `/dev/null` and a terminal give the same result for both.
- The ripgrep releases give one archive and one `.sha256` file for each of the four targets of 0020-name-the-release-archives-and-the-npm-platform-packages-by.md.

## Decision

The rg parity tests are one cargo integration test, `tests/rg_parity.rs`. It reads the cases from a data file, copies a fixture tree into a new temporary folder, and runs the pinned rg and the jag of the build there with the same arguments and the same fixed environment. It runs each case in three modes: in a pipe, in a pipe with `--color=always`, and with stdout on a `rustix` pseudo terminal. It compares stdout, stderr, and the exit code byte for byte, with no change to the order or the text. CI gets rg from the ripgrep GitHub release at the version and the SHA-256 in a pin file of the repository. A difference that stays is a deliberate difference only if a user `ANSWER` records it, and the test then checks the stored jag bytes.

## Why

- A cargo integration test runs in `cargo test --locked`, in the `rust` job of CI on Linux and macOS, as the other e2e tests in `tests/` do. It builds the jag of the pull request and needs no second test runner.
- The cases and the fixtures are data. A tester adds a case with no Rust code. `serde-saphyr` is already a dependency (0012-use-clap-serde-saphyr-and-rpassword-as-the-outside-crates.md), so it reads the case file.
- A copy in a new temporary folder outside the repository is necessary. Git cannot store a path with a `.git` part, and a `.gitignore` in the repository changes what git stores and what rg finds. In the copy, the ignore files of the jag repository have no effect.
- `rustix` gives the pseudo terminal with safe code, so the `unsafe_code` lint stays `forbid`. It adds features to a crate that jag already has, not a new crate.
- A pinned archive with a SHA-256 in the repository gives the same rg on each runner and on each machine of a developer. The release `.sha256` file comes from the same place as the archive, so the pin in the repository is the check, not that file.
- A byte comparison with no normalization finds each difference, also in color codes and in error text. A rule that hides a difference is a user decision, not a test decision.
- A record with a user `ANSWER` makes each difference that stays known and checked (STORY-67).

## Alternatives considered

- The `script` command for the pseudo terminal: the Linux form (util-linux) and the macOS form (BSD) take different arguments, and both put stderr and stdout into one stream.
- A Python helper with the `pty` module: it adds a second language and a second runtime to the e2e tests.
- `libc::openpty` in the test: it needs `unsafe` code, and the lint forbids it.
- A new crate for the pseudo terminal, such as `portable-pty` or `expectrl`: a new outside crate for what `rustix` already does.
- Cases in Rust code: no data file, but each case needs Rust code, and a reviewer cannot read a recorded difference as data.
- The rg of the package manager of the runner: its version changes with the runner image, so a test result changes with no change in jag.
- Sort the output of rg before the comparison: the test then does not check the order, and the user asked for the same output.
- Skip a case with a deliberate difference: the case then checks nothing, and a later change of jag goes unseen.

## Consequences

- Files:
  - `tests/rg_parity.rs`: the test.
  - `tests/parity/cases.yaml`: the cases. Each case has an id, the arguments, the fixture tree, the modes (pipe, color, and terminal; default all three), and an optional stdin file and optional extra environment variables.
  - `tests/parity/fixtures/<tree>/`: the fixture trees. A name that starts with `.` is stored with the prefix `dot-` in place of the `.`, and the copy gets the real name. So a fixture cannot have a real name that starts with `dot-`. The private-data check also reads the fixtures, so a fixture must not hold a private name.
  - `tests/parity/expected/`: the stored jag bytes of the deliberate differences.
  - `tests/parity/rg.sha256`: the pin. It has one line for each of the four targets of 0020-name-the-release-archives-and-the-npm-platform-packages-by.md, in the form of `sha256sum -c`. The archive name gives the version. The first pin is ripgrep 15.2.0.
  - `scripts/parity/install-rg.sh`: it downloads the archive of the target of the machine from the ripgrep GitHub release, checks it against the pin with `sha256sum -c` or `shasum -a 256 -c`, and extracts `rg`.
  - `.gitattributes` (a new file; the repository has none today): `tests/parity/fixtures/** -text` and `tests/parity/expected/** -text`, so that git does not change the bytes.
- Run:
  - Both programs get an empty environment, then the same fixed variables: `HOME` (an empty temporary folder), `TERM=xterm-256color`, and the extra variables of the case. So rg reads no config file (`RIPGREP_CONFIG_PATH` is not set), and jag reads no key and no settings.
  - Stdin is `/dev/null`, or the stdin file of the case. Stdin is never the pseudo terminal, so a program cannot wait for input. Each run has a time limit.
  - In the terminal mode, stdout is the other end of a new pseudo terminal with a fixed size and the default settings, and stderr is a pipe. The test closes its own copy of the terminal side after the program starts. It reads stdout and stderr while the program runs, so that a full buffer does not stop the program. It reads the pseudo terminal until the program closes it: on Linux, that read gives `EIO`, not end of file, and the test takes `EIO` as the end. Both programs get the same settings, so the `\r\n` of the terminal is the same for both.
  - In the color mode, stdout is a pipe, and the test puts `--color=always` before the arguments of the case, for both programs (CRITERION-112).
  - The test finds rg at the path in `JAG_PARITY_RG`. If that variable is not set, the test fails when `CI` is set, and else it prints one skip line and passes. `cargo test` shows that line only with `--nocapture`. The test fails if `rg --version` does not start with `ripgrep` and the version of the pin.
  - The test runs all cases, then fails once with the list of each case and mode that differs, and the first different byte of each stream.
- CI: the `rust` job of `ci.yml` (on `ubuntu-latest` and `macos-latest`) and the `build` job of `release.yml` (before the step "Test on the target") run `scripts/parity/install-rg.sh` and set `JAG_PARITY_RG` before `cargo test`. GitHub sets `CI` in each job, so each job that runs `cargo test` with no rg fails. This changes workflow files, so the push needs the `workflow` scope. Only `private-data` is a required check on `main` today, so a parity failure does not stop a merge by itself. A parity failure in `release.yml` stops the release.
- Deliberate differences: a case gets `deliberate:` with `answer:`, the user `ANSWER` that records the difference. The field is necessary. For that case and mode, the test checks that jag gives the stored bytes and the exit code, and that rg does not give them. When rg and jag agree again, the test fails, and the record must go.
- Order (open, user card C): the test does not sort output. A case that can match in more than one file must fix the order of rg with its arguments (for example `--sort path`), or search one file. DECISION-34 makes each option that is not in its list a usage error, so jag accepts `--sort path` only after a change of DECISION-34.
- Program name (open, user card A): rg starts each error line with `rg:`, and jag with `jag:`. With no rule, each error case is a difference. A rule that takes the two names as equal is a user decision; until an `ANSWER` gives it, each error case needs a deliberate record or a fix.
- A new rg version is a pull request that changes `tests/parity/rg.sha256`, and the records that it makes old.
- The test target adds `rustix` with the features `pty` and `termios` to `[dev-dependencies]`. The build of the jag binary does not get these features.
