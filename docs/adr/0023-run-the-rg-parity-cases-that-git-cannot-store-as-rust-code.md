---
type: adr
title: Run the rg parity cases that git cannot store as Rust code in a second test
description: Open this when you add or change an rg parity case that needs files git cannot store, or files outside the copied fixture tree.
area: quality
status: proposed
date: 2026-10-15
supersedes: null
superseded_by: null
amends: null
refines: null
deviates_from:
  - 0021-check-rg-parity-with-a-cargo-test-that-runs-a-pinned-rg-and.md
related:
  - 0022-use-rg-as-the-reference-for-the-output-of-jag-with-no.md
parent_spec: docs/product/roadmap/features/FEATURE-23.md
model:
  status: out-of-scope
  reason: the parity tests are not an element of the system design
---

# ADR-0023: Run the rg parity cases that git cannot store as Rust code in a second test

## Context

0021-check-rg-parity-with-a-cargo-test-that-runs-a-pinned-rg-and.md sets one cargo test, `tests/rg_parity.rs`, that reads all rg parity cases from `tests/parity/cases.yaml`. Each case copies a fixture tree from the repository into a new temporary folder and runs rg and jag in that folder.

Some FEATURE-23 cases need files that a fixture tree cannot hold:

- Git cannot store a file or a folder with mode `000`.
- Git stores a symbolic link as a link only on systems that permit it. On other systems, the checkout is a text file.
- Git cannot store a path with a `.git` part.
- On macOS, a file name that is not UTF-8 is not permitted.
- An `.ignore` or `.gitignore` file in a parent folder of the folder where rg runs is outside the copied tree.

The user chose a deviation from 0021-check-rg-parity-with-a-cargo-test-that-runs-a-pinned-rg-and.md for these cases (hitl `ANSWER` of 2026-10-15, card "rg parity cases that git cannot store", option 1).

## Decision

A case whose files git cannot store on each system, or that needs files outside the copied tree, can be Rust code in a second cargo test, `tests/rg_parity_fs.rs`. Each case makes its files in a new temporary folder, can run in a subfolder of it, and then follows all other rules of 0021-check-rg-parity-with-a-cargo-test-that-runs-a-pinned-rg-and.md: the pinned rg, the same fixed environment, the three modes, the byte comparison, and the name rule. The two tests share their run and compare code in `tests/parity/harness.rs`. A case in `tests/rg_parity_fs.rs` has no deliberate record: a difference must be fixed in jag, or the case moves to `tests/parity/cases.yaml` with a record. All other cases stay in `tests/parity/cases.yaml`, and that file gets no set-up fields.

## Why

- The files of these cases exist only after a program makes them, so Rust code is the direct form. Set-up fields in `tests/parity/cases.yaml` would make the data file a small language, and each new set-up step would need new Rust code in the test.
- One shared harness gives one form of the run and the comparison. A rule of 0021-check-rg-parity-with-a-cargo-test-that-runs-a-pinned-rg-and.md that changes applies to the two tests.
- With no deliberate record in Rust code, a reviewer reads each recorded difference as data in one file, as 0021-check-rg-parity-with-a-cargo-test-that-runs-a-pinned-rg-and.md requires.
- The limit to cases that git cannot store, or that need files outside the copied tree, keeps the data file the default for each new case.

## Alternatives considered

- Set-up fields in `tests/parity/cases.yaml`, for example a file mode, a link, or a run folder: all cases stay data, but the data file gets a set-up language, and each new kind of set-up needs a change of the test and of the file format.
- Change 0021-check-rg-parity-with-a-cargo-test-that-runs-a-pinned-rg-and.md with a new ADR that amends it: the main rule "cases are data" then has a second form in the same decision. A deviation keeps the main rule and records the exception.
- No tests for these cases: FEATURE-23 then does not check unreadable files, links, git folders, parent ignore files, or names that are not UTF-8.

## Consequences

- Files:
  - `tests/rg_parity_fs.rs`: the cases that git cannot store, as Rust code. Each case has an id, a set-up function, a run folder relative to the temporary folder, and the arguments.
  - `tests/parity/harness.rs`: the run and compare code of the two tests. Each test includes it with `#[path]`.
- A case that is only for one system has a `cfg` for that system, for example the file name that is not UTF-8 is only for Linux.
- A case that sets a mode of `000` gives back the permissions after the run, so that the temporary folder can go.
- A case that a fixture tree can hold goes in `tests/parity/cases.yaml`, not in `tests/rg_parity_fs.rs`.
- When a case in `tests/rg_parity_fs.rs` differs from rg and the difference stays, the case moves to `tests/parity/cases.yaml` with a `deliberate:` record. If no fixture tree can hold the case, the move needs a user `ANSWER` and a change of this ADR.
- The test finds rg at the path in `JAG_PARITY_RG`, and it fails or skips as `tests/rg_parity.rs` does. The CI jobs of 0021-check-rg-parity-with-a-cargo-test-that-runs-a-pinned-rg-and.md run it with no change.
