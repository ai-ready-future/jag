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

Done for the first part: the review of the commit below. The criterion is
checked again on the commit of the first tag (see "Before the first tag").

- Commit: `367219e` (`main`, after PR #146, #150, #147, and #153).
- Date: 2026-10-16 (UTC).
- Reviewer: a reviewer session that wrote none of the code under review
  (decision 18).
- Result: no open critical or high finding. 1 medium finding and 5 low
  findings. Each one has an open issue with the label `security`. Items 1,
  2, 3, 6, 7, 8, 9, 10, 12, and 15 pass. The level of each finding comes
  from its CVSS v3.1 vector (decision 11).
- Evidence:
  - The runs on the commit are green: `deps.yml`
    https://github.com/ai-ready-future/jag/actions/runs/38840646323,
    `CI` https://github.com/ai-ready-future/jag/actions/runs/38840646127,
    and `private-data`
    https://github.com/ai-ready-future/jag/actions/runs/38840646188.
  - `cargo test --locked` in Docker `rust:1.99` on the commit: 752 passed,
    0 failed, 1 ignored.
  - The repository settings were read with `gh api` (REST) on 2026-10-16.
  - Issues: #157, #158, #159, #160, #161, and #162.

### Findings

| Item | Place | CVSS v3.1 vector | Score | Level | Issue |
|---|---|---|---|---|---|
| 14 | Repository settings: secret scanning and push protection are off | `CVSS:3.1/AV:N/AC:H/PR:N/UI:N/S:U/C:L/I:L/A:N` | 4.8 | medium | #157 |
| 13 | `.github/workflows/adr-gate.yml`: `actions/checkout@v4` with no full SHA, no `permissions:` block, no `persist-credentials: false`, and `${{ github.base_ref }}` in `run:` | `CVSS:3.1/AV:N/AC:H/PR:N/UI:N/S:U/C:N/I:L/A:N` | 3.7 | low | #158 |
| 14 | The ruleset of `main` (25127941) has no rule `non_fast_forward` and no rule `deletion` | `CVSS:3.1/AV:N/AC:L/PR:H/UI:N/S:U/C:N/I:L/A:L` | 3.8 | low | #159 |
| 4 | `src/scorer/client.rs`: an `http` loopback base URL can use a proxy of the environment | `CVSS:3.1/AV:N/AC:H/PR:N/UI:R/S:U/C:L/I:N/A:N` | 3.1 | low | #160 |
| 11 | `.github/workflows/release.yml`, job `scan`: a dry run gets `PRIVATE_PATTERNS` | `CVSS:3.1/AV:N/AC:L/PR:H/UI:N/S:U/C:L/I:N/A:N` | 2.7 | low | #161 |
| 5 | `src/config/setup.rs` and `src/config/change.rs`: `--forget-key` does not remove a stale temporary copy of the key | `CVSS:3.1/AV:L/AC:H/PR:H/UI:R/S:U/C:L/I:N/A:N` | 1.8 | low | #162 |

### Items

1. Storage: pass. The folder is made with mode 0700 and the file with mode
   0600 (`src/config/fs.rs`). A new file is a temporary file in the same
   folder, made with `create_new` and mode 0600 before jag writes a byte,
   then renamed over `config.yml` (`src/config/change.rs`). A link or a
   special file at `config.yml` is refused. Note: a folder that exists
   keeps its mode, and jag does not check that the path of the folder is a
   folder. Only the same user can change it, and the key file is 0600 at
   all times. This is the STORY-01 design.
2. Read: pass. The mode comes from the open file (`fstat`), and jag reads
   from the same handle (`src/config.rs`, `src/config/fs.rs`). The reads for
   the base URL and the model name use the same checked read. The key order
   is the order of CV-7, DECISION-03, and ADR-0017:
   `JAG_TYPESAFE_API_KEY`, `TYPESAFE_API_KEY`, then the file; on the gateway
   route, `JAG_GATEWAY_API_KEY`, then the file. Notes: the type check of the
   file uses the path before the open, and `--setup` and `--forget-key` read
   the file before or without the mode check. The STORY-01 spec allows this,
   no text of that read goes to the output, and only the same user can
   change the folder.
3. Hold: pass. `ApiKey` has no `Clone`, `Serialize`, or `Deref`, and its
   `Debug` and `Display` give `***` (`src/config.rs`). The errors, the
   `--json` output, and the readiness check (`src/check.rs`) hold no key.
   The QS-5 tests check each error path for the key, for example
   `tests/secrecy.rs`, `tests/failure.rs`, `tests/output.rs`,
   `tests/check.rs`, `tests/gateway.rs`, and the unit tests of
   `src/config.rs`, `src/config/setup.rs`, and `src/check.rs`. All pass in
   the Docker run.
4. Send: finding #160 (low). The rest passes: only `https` with the host
   `api.typesafe.ai` and the path `/v1` is the direct route, and only the
   direct route reads a TypeSafe key (ADR-0017). Each call uses TLS with
   `RootCerts::PlatformVerifier` (ADR-0014) and `max_redirects(0)`. A base
   URL with `http` is allowed only for `127.0.0.1`, `localhost`, and
   `[::1]`.
5. Removal: finding #162 (low). `--forget-key` removes `config.yml`, or
   writes it again with no key, but a temporary file from a stopped write
   can stay.
6. Process list and terminal: pass. No option takes the key
   (`src/cli.rs`), and `--setup` reads it with `rpassword` and no echo. It
   refuses to run with no terminal.
7. Unsafe code and build scripts: pass. `Cargo.toml` has
   `unsafe_code = "forbid"`, and jag has no `build.rs`. All dependencies
   come from crates.io (`deny.toml` refuses other sources). The only
   dependency that compiles C is `ring`, from the crates in `Cargo.lock`.
8. Untrusted input: pass. The model reply is parsed into numbers only, so
   no text of the reply goes to the terminal. A reply that is not valid, or
   a score that is not from 0 to 1, gives a failure, not a panic. The body
   is limited by the default limit of `ureq` (10 MB) and by the global
   timeout. Notes: the limit is not set in the code of jag, and a small
   explicit limit is a hardening. The content and the names of the user's
   files go to the terminal as the files hold them, as `rg` does
   (ADR-0022). The `--json` output escapes control characters.
9. No telemetry and no log file: pass. The only network call is the call
   to the model service. The only file writes are the config file writes.
   No logger is in the dependency tree.
10. npm launcher: pass. `npm/jag/bin/jag.js` starts the binary of a fixed
    platform package with `spawn` and no shell. No package has an install
    script, and `release.py` and `launcher.test.js` check this.
11. `release.yml`: finding #161 (low). The rest passes: the default is
    `contents: read`; only `crates-io` and `npm` have `id-token: write`, and
    only in the environment `release`; the repository and the environment
    store no publish token (the only secret is `PRIVATE_PATTERNS`); each
    action has a full commit SHA; no job uses a cache; no `run:` text holds a
    `${{ }}` value; the jobs `github-draft` and `npm` publish the scanned
    files. Note: the job `crates-io` packs the crate again from the same
    commit with `--locked`, as the FEATURE-18 spec designs.
12. `private-data.yml`: pass. The scripts come from the folder `base` only.
    The folder `pr` is read with `git -C` as data only, with no text
    conversion and no external diff. gitleaks gets its config with `-c`,
    and an ignore file is refused. The secret goes through `env:` and is
    never printed.
13. The other workflows: finding #158 (low) for `adr-gate.yml`. `ci.yml`,
    `go-public.yml`, `private-data-tests.yml`, and `deps.yml` pass:
    `contents: read` (`go-public.yml` also `issues: read`), each action has a
    full commit SHA, and no secret on `pull_request`. Note: `go-public.yml`
    was removed on 2026-10-26, because the move to the public repository is
    done.
14. Repository settings: findings #157 (medium) and #159 (low). The rest
    passes: the tag ruleset (25184816) has "update" and "deletion" for
    `refs/tags/v*` with no bypass; the environment `release` allows only the
    tags `v*`; immutable releases are on; the Actions policy 7541 is active
    and allows only `pull_request`, `pull_request_target`, `push`, and
    `workflow_dispatch`; the fork approval setting is
    `all_external_contributors`; the default workflow permission is `read`.
    Note: the environment `release` has `can_admins_bypass: true` and no
    required reviewer. FEATURE-18 step U2 does not ask for them.
15. Dependencies: pass. The run of `deps.yml` on the commit is green (the
    URL above). `deny.toml` has no `ignore` entry.

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
  is not the size of the first test (118,661 bytes on `0dad67b`):
  PR #135 added the crates `grep-printer` 0.3.1 and `termcolor` 1.4.1 to
  `Cargo.lock`, and the "Used by" lines name them (37 bytes). The number of
  sections is the same.
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
