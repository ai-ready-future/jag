# Release readiness

This file is the record of the checks that must pass before the first
publish of jag (FEATURE-22). The spec is
`docs/superpowers/specs/2026-10-15-STORY-62-check-that-jag-is-ready-before-the-first-publish-design.md`.

Each section gives the commit, the date, the result, and the evidence (run
URLs and issue numbers). This file holds no key, no address, and no steps of
an attack.

## Use period

Criterion: CRITERION-105 (STORY-62).

Not done.

## Dependency audit

Criterion: CRITERION-106 (STORY-63).

Done for the first part: the audit tools run on `main`. The criterion is
checked again on the commit of the first tag (see "Before the first tag").

- Commit: `16b5cc7` (`main`, PR #146).
- Date: 2026-10-16 (UTC).
- Result: the run of `deps.yml` is green. It runs `scripts/deps/check.sh`:
  `cargo-deny` (advisories, licenses, bans, and sources), `npm audit`, and
  `release.py check-npm-deps`. No open finding of high or critical severity.
- Evidence: https://github.com/ai-ready-future/jag/actions/runs/38837824825
- `ignore` entries in `deny.toml`: none, so no issue with the label
  `security` is necessary.
- The fix of the 4 advisories of `rustls` 0.23.45 (RUSTSEC-2026-0346 to
  RUSTSEC-2026-0349): `Cargo.lock` has `rustls` 0.23.46 (PR #146).

## Security review

Criterion: CRITERION-107 (STORY-63).

Not done.

## Licenses

Criterion: CRITERION-108 (STORY-64).

Done for the first part: the license audit runs on `main`. The criterion is
checked again on the commit of the first tag (see "Before the first tag").

- Commit: `6d77602` (`main`, PR #147).
- Date: 2026-10-16 (UTC).
- Result: `cargo-deny check licenses` and `release.py check-npm-deps` pass.
  Both run in `scripts/deps/check.sh`.
- Evidence: the run of `deps.yml` on the commit is green:
  https://github.com/ai-ready-future/jag/actions/runs/38838946165
- The license of jag is `MIT` (`LICENSE`). `gh api repos/ai-ready-future/jag --jq .license.spdx_id`
  gives `MIT`.
- The npm packages have no third-party dependency (`check-npm-deps`).
- Each of the 6 allowed licenses of `deny.toml` is permissive. The license
  gives the right to use, copy, and give the code again when the notice of
  the code goes with each copy. So each one is compatible with MIT:
  - `MIT`: permissive. Needs the copyright notice and the license text.
  - `Apache-2.0`: permissive. Needs the license text and the notices.
  - `ISC`: permissive. Needs the copyright notice and the license text.
  - `BSD-3-Clause`: permissive. Needs the notice and the license text. It
    forbids the use of the names of the authors for promotion.
  - `Unicode-3.0`: permissive. Needs the copyright notice and the license
    text.
  - `CDLA-Permissive-2.0`: permissive. It applies to data files. It needs
    the license text with each copy.
- No copyleft license is in the list. A new license needs a pull request that
  changes `deny.toml` and `about.toml`.
- `unused-allowed-license = "deny"` passes, so each of the 6 licenses is used.
- A search for files named `NOTICE*` in the sources of the 109 crates that
  `cargo fetch --locked` gets for the four release targets (in `rust:1.99`,
  2026-10-16) found none. So the template needs no `NOTICE` text.

## Notices and metadata

Criterion: CRITERION-109 (STORY-64).

Done for the first part: the dry run on `main` makes and checks the files.
The criterion is checked again on the commit of the first tag (see "Before
the first tag").

- Commit: `6d77602` (`main`, PR #147).
- Date: 2026-10-16 (UTC).
- Result: the release dry run on the commit is green. All jobs passed
  (`prepare`, `npm-main`, `notices`, `build` for the 4 targets, `crate`,
  `deps`, and `scan`). The 5 publish jobs were skipped, so nothing was
  published.
- Evidence: https://github.com/ai-ready-future/jag/actions/runs/38838966142
  (`workflow_dispatch` on `main`).
- `LICENSE` is at the root of the repository, and the same file is in the
  archive of each platform and in each of the 5 npm packages (the tests of
  `test_release.py` compare the bytes).
- Each archive holds `jag`, `README.md`, `LICENSE`, and
  `THIRD-PARTY-LICENSES`. Each platform npm package holds `bin/jag`,
  `README.md`, `LICENSE`, `THIRD-PARTY-LICENSES`, and `package.json`. The job
  `build` runs `check-npm-tarball`, which fails if one of these files is not
  there.
- `THIRD-PARTY-LICENSES` is made by the job `notices` with cargo-about 0.9.2.
  In this run it has 118,698 bytes and 64 license sections, and
  `check-notices` passes. The file is not stored in the repository. The size
  is not the size of the first test (118,661 bytes on `0dad67b`): `Cargo.lock`
  changed with `rustls` 0.23.46.
- The metadata of the crate and of the 5 npm packages is checked in the same
  run. `check-crate-metadata` (job `crate`) and `check-npm-tarball` (job
  `build` and job `npm-main`) check `license` (`MIT`), `repository`, and
  `description`. `check-crate-list` checks the files of the crate.
- The scan of the packages (job `scan`) passed with the notice file in the
  archives and the platform packages.

## Before the first tag

The list of the spec, section "Before the first tag". Chores does it on the
commit of the tag.

Not done.
