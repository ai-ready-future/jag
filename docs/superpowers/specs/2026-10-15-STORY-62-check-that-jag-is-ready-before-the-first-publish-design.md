---
title: "Check that jag is ready before the first publish: the use period, the security audit, and the license audit"
feature: FEATURE-22
stories: [STORY-62, STORY-63, STORY-64]
kind: build
gh_issue: 107
lifecycle_state: draft
inherits: [QS-5, CV-1, CV-6, CV-7, CV-8]
session: 4acb9f7a-4603-41fb-8e74-395e5117e288
---

# Check that jag is ready before the first publish: the use period, the security audit, and the license audit

## Problem

Before the first publish, three checks must pass (FEATURE-22). The user said:
"I would like to first use the app for a while locally and iterate and fix
it", "I would like to audit it for security vulnerabilities", and "I would
like to check the license and audit it for compliance and make sure we are in
good standing" (user decision of 2026-10-15). FEATURE-22 blocks FEATURE-18,
so the user steps U3 to U6 of the FEATURE-18 spec wait for it.

- STORY-62: a shell power user (PERSONA-04) wants the maintainer to use jag
  on real work for a period before the first publish, so that the first
  release has no known bug that this use found.
  - CRITERION-105: the user uses jag on real work for a period that the user
    selects. Before the first publish, each bug that this use found is fixed
    on `main` or has an open issue, and the user says that the period is
    complete.
- STORY-63: an agent builder (PERSONA-01) wants a security audit of the code,
  the dependencies, and the release workflow before the first publish, so
  that the first release has no known high vulnerability.
  - CRITERION-106: on the commit that the first publish uses, the Rust
    dependency advisory audit and npm audit of the npm packages show no open
    finding of high or critical severity.
  - CRITERION-107: a security review of the jag code and of the release
    workflow finds no open finding of high or critical severity before the
    first publish. The review includes how jag stores, reads, and sends the
    API key of the user (CONSTRAINT-02). Each finding of lower severity is
    fixed or has an open issue.
- STORY-64: an agent builder (PERSONA-01) wants a license and compliance
  audit before the first publish, so that they can use jag and its packages
  with no license problem.
  - CRITERION-108: before the first publish, a license check of all Rust and
    npm dependencies passes, and each dependency license is compatible with
    the jag license.
  - CRITERION-109: before the first publish, the repository and each release
    archive hold the jag `LICENSE` file and each notice that a dependency
    license requires. The crate and npm package metadata give the correct
    license, repository, and description.

The traced PRD pages: SCOPE-01 (release 1 is the first public release),
CONSTRAINT-01 (no private data in the public repository, its history, or a
package), and CONSTRAINT-02 (jag never prints, logs, or sends the key to a
place other than the model service, and refuses a key file that other users
can read).

A first run of the audit tools on `main` (`0dad67b`, 2026-10-15) found four
advisories of `rustls` 0.23.45 and no license problem. See decision 24.

## Decisions

| # | Decision | Choice | Source |
|---|---|---|---|
| 1 | Specs | One spec for STORY-62, STORY-63, and STORY-64, in one pull request | Orchestrator TASK for FEATURE-22 ("1 feature = 1 spec = 1 PR") |
| 2 | The first publish | The first publish of a jag program is the first version tag (FEATURE-18 step U6). "The commit that the first publish uses" is the commit of that tag. Step U3 publishes an empty version 0.0.0 with no program. U3 also waits, because FEATURE-22 blocks FEATURE-18 | Reviewer note on PR #104 (comment 6199353571): "FEATURE-18 steps U3 to U6" are the user steps of the FEATURE-18 spec, and U3 waits. FEATURE-18 spec, "The user steps before the first tag". FEATURE-22 links (`blocks: FEATURE-18`) |
| 3 | Length and form of the use period | The user selects the length. There is no minimum and no maximum. The period starts when chores opens the tracking issue (decision 5). It ends when the user says, in their own words, that it is complete. The user uses jag on their own real work, on their own machine, with their own TypeSafe key | CRITERION-105 ("a period that the user selects"); the words of the user ("use the app for a while locally and iterate and fix it"); orchestrator TASK ("the use period is the user's") |
| 4 | The binary of the use period | The archive of the platform of the user, from the artifact `build-<target>` of a release dry run on `main` (`workflow_dispatch`). Chores starts the run and gives the user the commands. After a fix merges, chores starts a new run, and the user installs the new binary. This is the binary that a release makes. Not `cargo install`: the machine of the team has no Rust toolchain | Inferred: FEATURE-18 spec decision 5 (the dry run) and `release.yml` (the artifact `build-<target>`, kept 14 days); `docs/adlc.md`, the build row (no `cargo` on this machine) |
| 5 | Record of the use period | The new label `use-period`. One tracking issue, "Use period before the first publish", with only the label `use-period`. It gets no `type:` label, because the `type:` labels are for the issues of roadmap pages. Each bug that the use finds is one issue with the labels `bug` and `use-period`. A wish that is not a bug gets `enhancement` and `use-period`, and CRITERION-105 does not count it. Each issue names the commit of the binary and the steps that show the bug. The issue holds no key and no private text. The end: the orchestrator writes a comment on the tracking issue with the date and the words of the user, then closes the issue | Reviewer note on PR #104 (name the record, for example an issue label); CRITERION-105; CONSTRAINT-01 |
| 6 | "Fixed on `main` or has an open issue" | Each issue with the labels `bug` and `use-period` is open, or it is closed as `completed` by a merged pull request or a commit on `main`. An issue that is closed as `not_planned` must also have the label `duplicate` or `invalid`. Else CRITERION-105 fails | CRITERION-105 |
| 7 | Owner of each step | See "Owners". The user: the use period, each bug report, and the end of the period. Agents: all other steps | Orchestrator TASK ("who does each step (agent vs user): the use period is the user's"); `docs/adlc.md`, the roles |
| 8 | Rust advisory tool | cargo-deny 0.20.2 (released 2026-07-09), not cargo-audit. cargo-deny reads the same RustSec database. One tool and one file (`deny.toml`) check the advisories (CRITERION-106) and the licenses (CRITERION-108) | CRITERION-106 and CRITERION-108 (both name cargo deny); crates.io (cargo-deny 0.20.2, cargo-audit 0.22.2, read 2026-10-15); context7 `/embarkstudios/cargo-deny` |
| 9 | Install of the tools | Each workflow downloads the prebuilt release archive of the tool and checks it with `sha256sum -c` against a fixed SHA-256, as `private-data.yml` does for gitleaks. cargo-deny 0.20.2, `cargo-deny-0.20.2-x86_64-unknown-linux-musl.tar.gz`, SHA-256 `9f12ed4c49936e09b48bf862b595cde2fe64fcbd9d74dfacac6131ca824c8d5f`. cargo-about 0.9.2, `cargo-about-0.9.2-x86_64-unknown-linux-musl.tar.gz`, SHA-256 `9099a59e820c38a68b9d65f300662a567d56562f9a10f6aa4c7e86c17c2566af`. Not `EmbarkStudios/cargo-deny-action`: it runs a container image that the pin does not cover | `private-data.yml` (the gitleaks download); FEATURE-18 spec decision 28 (pinned actions); the `.sha256` files of both releases, checked against the downloaded archives on 2026-10-15 |
| 10 | `deny.toml` | The file of "The dependency policy". It checks the four release targets only, and no dev-dependency. Each advisory fails the check: a vulnerability, an unmaintained crate, an unsound crate, and a yanked version. Tested on `0dad67b` in `rust:1.99`: `bans ok, licenses ok, sources ok`, and `advisories FAILED` (decision 24) | context7 `/embarkstudios/cargo-deny` (cargo-deny 0.20: no `severity-threshold`; an advisory is an error unless `ignore` names it); the test run of 2026-10-15 |
| 11 | The severity rule (one rule for CRITERION-106 and CRITERION-107) | The level comes from the CVSS v3.1 base score: critical 9.0 to 10.0, high 7.0 to 8.9, medium 4.0 to 6.9, low 0.1 to 3.9. npm gives its own level, and npm "moderate" is medium. A finding with no score (an unmaintained crate, an unsound crate, a review finding) gets a CVSS v3.1 vector from the person who records it, and then the same bands. Critical or high: the finding blocks the publish until a fix is on `main`. No issue and no `ignore` entry can replace the fix. Medium or low: a fix, or an open issue with the label `security`. For a RustSec advisory, the open issue also needs an `ignore` entry in `deny.toml`, with the issue number in `reason` | CRITERION-106 and CRITERION-107 ("high or critical"; "fixed or has an open issue"); reviewer note on PR #104 (one rule for both); RustSec gives a CVSS v3.1 vector for each advisory |
| 12 | Where the dependency checks run | A new workflow `.github/workflows/deps.yml`, job `deps`, on `pull_request`, on `push` to `main`, and on `workflow_dispatch`. It is not a required check, because a new advisory can fail it with no change in the pull request. `release.yml` gets a job `deps` with the same script, and the job `github-draft` needs it. So no release publishes with a finding that the rule does not allow. No `schedule` event: the Actions policy 7541 allows only `pull_request`, `pull_request_target`, `push`, and `workflow_dispatch` | CRITERION-106 ("on the commit that the first publish uses"); FEATURE-19 spec decision 22 (policy 7541); `ci.yml` (the job `release-tools` is not a required check) |
| 13 | npm audit | In a new temporary copy of `npm/jag/package.json`: `npm install --package-lock-only --ignore-scripts --no-audit --no-fund`, then `npm audit` with no `--audit-level`, so each finding fails. The npm packages have no third-party dependency: the main package has only the four platform packages as optional dependencies, and a platform package has none. Tested on 2026-10-15 (npm 12.2.0): "found 0 vulnerabilities" | Reviewer note on PR #104 (no `package-lock.json`, so `npm audit` gives ENOLOCK); context7 `/npm/cli` (`npm audit` needs a lock file; the exit code follows `audit-level`); `npm/jag/package.json` |
| 14 | Allowed licenses | `MIT`, `Apache-2.0`, `ISC`, `BSD-3-Clause`, `Unicode-3.0`, and `CDLA-Permissive-2.0`. These six are the licenses that the dependency tree of the four release targets needs, and each one is used. Each one is permissive: an MIT program can hold the code when the notice of the code goes with each copy. No copyleft license. A crate with an `OR` expression passes when one of its licenses is in the list. A new license needs a pull request that changes `deny.toml` and `about.toml`, and its reviewer checks the license | `cargo metadata` of `Cargo.lock` at `0dad67b`, filtered to the four targets by cargo-deny (`unused-allowed-license = "deny"` passes, so each license is used); jag `LICENSE` (MIT, FEATURE-18 spec decision 31) |
| 15 | Third-party notices | A new job `notices` of `release.yml` makes the file `THIRD-PARTY-LICENSES` with cargo-about 0.9.2, `scripts/release/about.toml`, and the template `scripts/release/notices.hbs`. `release.py` puts the file into each archive and into each platform npm package. The crate and `@airf/jag` do not get it: they hold no third-party code. The repository does not get it: it holds no dependency code, only `Cargo.lock`, and a stored copy must change with each change of `Cargo.lock`. Tested on `0dad67b` in `rust:1.99`: 118,661 bytes, 64 license sections, and the same bytes in two runs | CRITERION-109; reviewer note on PR #104 ("each archive of the release dry run on the commit that the first publish uses"); the licenses of decision 14 ask for the notice in each copy, also in binary form; the test run of 2026-10-15 |
| 16 | Metadata check | `release.py` checks the `license`, `repository`, and `description` of the crate and of the five npm packages in each dry run and each release. The values: `license` is `MIT`; `repository` is `https://github.com/ai-ready-future/jag` for the crate and `git+https://github.com/ai-ready-future/jag.git` for npm; `description` is the text of `Cargo.toml` for the crate and `@airf/jag`, and the platform text of `release.py` for each platform package | CRITERION-109; FEATURE-18 spec decisions 11 and 13; `Cargo.toml`; `npm/jag/package.json` |
| 17 | Scope of the security review | The list of "The security review". It covers the key (storage, read, hold, send, removal), the other inputs and outputs, the npm launcher, each workflow, the repository settings, and the dependency checks | CRITERION-107; CONSTRAINT-02; QS-5; CV-6; CV-8; `docs/adr/0014-call-the-model-service-with-ureq-and-serde-json-on-std.md`; `docs/adr/0017-use-a-separate-key-for-an-ai-gateway.md`; FEATURE-18 spec decision 28 |
| 18 | Who does the review | A reviewer session (the model and effort of the role reviewer in `docs/adlc.md`) that wrote none of the code under review. It writes its result into `docs/release-readiness.md` in a docs pull request. A second reviewer reviews that pull request. The reviewer opens one issue with the label `security` for each open medium or low finding. The issue names the place and the fix, and gives no steps of an attack | CRITERION-107; reviewer note on PR #104 (say where the result is recorded); `docs/adlc.md`, the roles |
| 19 | The record | One file, `docs/release-readiness.md`, with one section for each criterion: the commit, the date, the result, and the evidence (run URLs and issue numbers). The STORY-63 build pull request adds the file with empty sections | Reviewer notes on PR #104 (CRITERION-105 and CRITERION-107 need a record) |
| 20 | The check before the first tag | Before step U6, chores does the list of "Before the first tag" on the commit of the tag, and adds the result to the record. `scripts/release/README.md` gets the list | CRITERION-106 ("on the commit that the first publish uses"); FEATURE-18 spec, step U6 |
| 21 | When each story is done | STORY-62: after the tracking issue is closed with the words of the user, and the check of decision 6 passes and is in the record. A code merge alone does not finish it. STORY-63: after steps 1, 2, and 4 of "Build order": the build merges, the run of `deps.yml` on `main` is green, and the merged review record shows no open critical or high finding and an issue or a fix for each other finding. STORY-64: after step 3: the build merges, a release dry run on `main` is green with the new checks, and the sections "Licenses" and "Notices and metadata" of the record are complete | CRITERION-105 to CRITERION-109; FEATURE-19 spec decision 25 and FEATURE-18 spec decision 29 use the same form |
| 22 | Story state | STORY-62, STORY-63, and STORY-64 are `ready`. The spec is in this pull request, FEATURE-22 has no `blocked_by`, and no `ASK` of this spec is open: the spec sends none | `docs/adlc.md` rule 11 |
| 23 | ADR | No new ADR. `deny.toml`, the two workflows, and the notice file are cheap to change before and after the first publish. One accepted ADR has a line that this spec changes: the Consequences of `docs/adr/0020-name-the-release-archives-and-the-npm-platform-packages-by.md` say that the archive holds `jag`, `README.md`, and `LICENSE`. The decision of that ADR (the names) does not change. Before the STORY-64 build pull request merges, the architect updates the line by `docs/adr/CLAUDE.md` (for example the "Rewrite" form) or by `docs/adlc.md` (an `ASK`) | `docs/adr/CLAUDE.md` ("If it is cheap to change your mind, do not write an ADR"); ADR-0020 |
| 24 | The `rustls` advisories of 2026-10-15 | The first build step updates `rustls` in `Cargo.lock` to 0.23.46 or later (released 2026-10-14). This fixes RUSTSEC-2026-0346 (CVSS 4.0, medium), RUSTSEC-2026-0347 (2.3, low), RUSTSEC-2026-0348 (3.7, low), and RUSTSEC-2026-0349 (3.7, low). None is high, but a fix is one command, so no issue and no `ignore` entry is necessary | cargo-deny 0.20.2 on `0dad67b`, 2026-10-15 (`Solution: Upgrade to >=0.23.46`); crates.io (`rustls` 0.23.46, 2026-10-14) |
| 25 | Labels | Chores makes the labels `use-period` ("Found in the use period before the first publish (FEATURE-22)") and `security` ("A security finding; see the FEATURE-22 severity rule") with `gh label create`, which uses REST | FEATURE-19 spec decision 21 (chores makes the labels); the repository has neither label on 2026-10-15 |
| 26 | Tests | Python `unittest` cases in `scripts/release/test_release.py` for each new check of `release.py`. The job `deps` tests `deny.toml` and the npm audit. The dry run of the pull request tests the job `notices` and the new members | FEATURE-18 spec decision 18 |
| 27 | Changes to the FEATURE-18 spec | This spec changes these parts of the FEATURE-18 spec: decision 10 and "The archives" (a fourth member `THIRD-PARTY-LICENSES`), the file list of a platform package (decisions 12 and 13), the job order of decision 15 (the jobs `notices` and `deps`), and the paths of the dry run of decision 5 (`deny.toml` and `scripts/deps/`). Where the two specs differ, this spec applies | FEATURE-18 spec |

## Design

### Owners

| Step | Owner |
|---|---|
| The labels, the tracking issue, the dry runs for the binary of the user, and the install commands | chores |
| Use jag on real work, report each bug, say when the period is complete | the user |
| Turn a report of the user into an issue | chores, or the orchestrator |
| A fix of a bug | an implementer, through the orchestrator (`docs/adlc.md`) |
| The build of STORY-63 and STORY-64 | an implementer |
| The security review and its record | a reviewer session (decision 18) |
| The review of the record pull request | a second reviewer |
| The update of the line of ADR-0020 | the architect |
| The checks of "Before the first tag" | chores |
| The story states | the planner, by decision 21 |

### Files

| File | Change | Story |
|---|---|---|
| `Cargo.lock` | `rustls` 0.23.46 or later | STORY-63 |
| `deny.toml` | New: the dependency policy | STORY-63, STORY-64 |
| `scripts/deps/check.sh` | New: downloads cargo-deny, runs it, and runs npm audit | STORY-63, STORY-64 |
| `.github/workflows/deps.yml` | New: the job `deps` | STORY-63 |
| `.github/workflows/release.yml` | The jobs `deps` and `notices`; `build` uses the notice file; the dry-run paths | STORY-63, STORY-64 |
| `scripts/release/about.toml` | New: the cargo-about config | STORY-64 |
| `scripts/release/notices.hbs` | New: the template of `THIRD-PARTY-LICENSES` | STORY-64 |
| `scripts/release/release.py` | The notice file, the metadata checks, and `check-npm-deps` | STORY-64 |
| `scripts/release/test_release.py` | The cases of "Tests" | STORY-64 |
| `scripts/release/README.md` | "Before the first tag" | STORY-63, STORY-64 |
| `docs/release-readiness.md` | New: the record | STORY-62, STORY-63, STORY-64 |

### The use period (STORY-62)

1. Chores makes the labels of decision 25. Then chores opens the tracking
   issue "Use period before the first publish" with the label
   `use-period`. The body gives the start date, the rules of decisions 5
   and 6, and how to report a bug.
2. Chores starts a release dry run on `main`:

   ```
   $ gh workflow run release.yml --ref main
   ```

   When the run is green, chores gives the user the commands for the
   platform of the user, for example Linux x86_64:

   ```
   $ gh run download <run id> -R ai-ready-future/jag -n build-x86_64-unknown-linux-musl -D <new empty folder>
   $ tar -xzf <new empty folder>/jag-<v>-x86_64-unknown-linux-musl.tar.gz -C <new empty folder>
   $ install -m 0755 <new empty folder>/jag-<v>-x86_64-unknown-linux-musl/jag ~/.local/bin/jag
   $ jag --version
   ```

   Chores writes the run id and its commit as a comment on the tracking
   issue. The artifact stays for 14 days. After that, a new run is
   necessary.
3. The user runs `jag --setup` one time, then uses jag on their real work.
4. When the user finds a bug, the user tells the orchestrator or chores in
   their own words, or opens an issue. The agent opens the issue with the
   labels `bug` and `use-period`. It gives the words of the user, the
   commit of the binary, the command, what jag did, and what the user
   expected. It holds no key, no private file content, and no address.
5. The orchestrator routes each bug issue to a fix. A fix pull request uses
   `Fixes #<n>`, so the merge closes the issue as `completed`. After a fix
   merges, chores does step 2 again when the user asks for the new binary.
6. When the user says that the period is complete, the orchestrator writes
   a comment on the tracking issue: the date and the words of the user.
   Then it closes the tracking issue.
7. Chores runs the check of decision 6 with REST, then writes the result in
   the record:

   ```
   $ gh api -X GET repos/ai-ready-future/jag/issues -f labels=bug,use-period -f state=all --paginate --jq '.[] | [.number, .state, .state_reason, ([.labels[].name] | join(","))] | @tsv'
   ```

   For each closed issue with `state_reason` `completed`, chores checks the
   `closed` event of the issue timeline
   (`gh api repos/ai-ready-future/jag/issues/<n>/timeline`): it must name a
   commit on `main`. The check passes when each issue is open, closed by a
   commit on `main`, or closed as `not_planned` with `duplicate` or
   `invalid`.

### The dependency policy (STORY-63 and STORY-64)

`deny.toml` at the root of the repository. The test of 2026-10-15 used this
text:

```toml
# The dependency policy of jag (FEATURE-22 spec).

[graph]
# The four release targets (FEATURE-18 spec decision 2).
targets = [
    "x86_64-unknown-linux-musl",
    "aarch64-unknown-linux-musl",
    "aarch64-apple-darwin",
    "x86_64-apple-darwin",
]
exclude-dev = true

[advisories]
yanked = "deny"
unmaintained = "all"
unsound = "all"
# Each entry: { id = "RUSTSEC-YYYY-NNNN", reason = "#<issue>: <level>, <why>" }.
# Only a medium or low finding (FEATURE-22 spec decision 11).
ignore = []

[licenses]
allow = [
    "Apache-2.0",
    "BSD-3-Clause",
    "CDLA-Permissive-2.0",
    "ISC",
    "MIT",
    "Unicode-3.0",
]
confidence-threshold = 0.93
unused-allowed-license = "deny"

[bans]
multiple-versions = "warn"
wildcards = "deny"

[sources]
unknown-registry = "deny"
unknown-git = "deny"
```

With `unused-allowed-license = "deny"`, a license that the tree no longer
needs fails the check, so the list stays the list of decision 14.

### The script `scripts/deps/check.sh`

Bash, `set -euo pipefail`. It runs from the root of the repository on an
`ubuntu-24.04` runner. It needs `cargo`, `npm`, `curl`, `tar`, and
`sha256sum`.

1. Make a new temporary folder. Download the cargo-deny archive of
   decision 9 into it, check it with `sha256sum -c`, and take only the file
   `cargo-deny-0.20.2-x86_64-unknown-linux-musl/cargo-deny`.
2. Run `cargo-deny --locked check advisories licenses bans sources`. It
   reads `deny.toml`.
3. Copy `npm/jag/package.json` into a new empty folder. In that folder, run
   `npm install --package-lock-only --ignore-scripts --no-audit --no-fund`,
   then `npm audit`.
4. Run `python3 -I scripts/release/release.py check-npm-deps npm/jag/package.json`.

Each step that fails stops the script with its exit code. The script prints
no secret: it reads none.

### The workflow `deps.yml`

```yaml
name: deps

on:
  pull_request:
  push:
    branches: [main]
  workflow_dispatch:

permissions:
  contents: read

jobs:
  deps:
    name: deps
    runs-on: ubuntu-24.04
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      - name: Install the toolchain
        run: |
          rustup show active-toolchain || rustup toolchain install
          cargo --version
      - uses: actions/setup-node@949feb2413d6458794dcd2491c4babbbce0c15c1 # v7.1.0
        with:
          node-version: '24'
          package-manager-cache: false
      - run: scripts/deps/check.sh
```

No cache, no secret, and no write permission.

### The jobs of `release.yml`

- `deps`: the same steps as `deps.yml`, with `needs: prepare`. The job
  `github-draft` gets `deps` in its `needs`. So a release with a finding
  stops before the draft and before each publish. In a dry run, the job
  runs and shows the result.
- `notices`: `runs-on: ubuntu-24.04`, `needs: prepare`. It installs the
  toolchain, downloads the cargo-about archive of decision 9 with
  `sha256sum -c`, and runs:

  ```
  cargo-about generate --locked --fail -c scripts/release/about.toml -o "$RUNNER_TEMP/notices/THIRD-PARTY-LICENSES" scripts/release/notices.hbs
  ```

  Then `release.py check-notices` checks the file (see "`release.py`").
  The job uploads the artifact `notices` with `actions/upload-artifact` at
  the pin of `release.yml`, kept 14 days.
- `build`: `needs: [prepare, npm-main, notices]`. It downloads the artifact
  `notices` with `actions/download-artifact` at the pin of `release.yml`,
  and gives `--notices <path>` to `release.py archive` and to
  `release.py npm-platform`.
- The `pull_request` paths of the dry run get `deny.toml` and
  `scripts/deps/**`. `scripts/release/**` already holds `about.toml` and
  `notices.hbs`.

`--fail` stops the job when cargo-about cannot find or clarify the license
of a crate. The fix is a `clarify` entry in `about.toml`, in a pull request.

### The notice file

`scripts/release/about.toml`, as tested on 2026-10-15:

```toml
# The third-party notices of the release binary (FEATURE-22 spec).
# The order is the order of preference for a crate with more than one license.
accepted = [
    "MIT",
    "Apache-2.0",
    "ISC",
    "BSD-3-Clause",
    "Unicode-3.0",
    "CDLA-Permissive-2.0",
]
targets = [
    "x86_64-unknown-linux-musl",
    "aarch64-unknown-linux-musl",
    "aarch64-apple-darwin",
    "x86_64-apple-darwin",
]
ignore-dev-dependencies = true
ignore-build-dependencies = true
```

`scripts/release/notices.hbs`, as tested on 2026-10-15. The output is plain
text, so no license text can break a Markdown block:

```handlebars
THIRD-PARTY LICENSES

The jag binary holds code from the Rust crates below. Each license
applies to the crates that it names.
{{#each licenses}}

==============================================================================
{{{name}}} ({{{id}}})
Used by:{{#each used_by}} {{{crate.name}}} {{{crate.version}}};{{/each}}
------------------------------------------------------------------------------

{{{text}}}
{{/each}}
```

The triple braces stop the HTML escape of the text. The file names the
crates of the four targets, with no dev-dependency and no build-dependency.
One file serves all four targets, so each archive holds the same file.

No crate of the dependency tree had a `NOTICE` file on 2026-10-15 (a search
of the crate sources in `rust:1.99`). The license audit checks this again
(see "The license audit").

### `release.py`

- `make_archive` and the subcommand `archive`: the new required option
  `--notices`. The archive gets `jag-<v>-<target>/THIRD-PARTY-LICENSES`,
  mode 0644, after `LICENSE`.
- `npm_platform_package`: `"files": ["bin/jag", "THIRD-PARTY-LICENSES"]`.
  npm adds `LICENSE`, `README.md`, and `package.json` by itself.
- The subcommand `npm-platform`: the new required option `--notices`. It
  copies the file to `THIRD-PARTY-LICENSES` in the package folder.
- `check-npm-tarball`: a platform package must have exactly
  `package/package.json`, `package/README.md`, `package/LICENSE`,
  `package/THIRD-PARTY-LICENSES`, and `package/bin/jag`. The main package
  keeps its list. It also checks `license`, `repository.url`, and
  `description` by decision 16.
- The new subcommand `check-crate-metadata <.crate file>`: reads
  `Cargo.toml` of the packed crate with `tomllib`, and checks `license`,
  `repository`, and `description` by decision 16. The job `crate` runs it
  after `cargo package`.
- The new subcommand `check-notices <file>`: the file is UTF-8, is not
  empty, and its first line is `THIRD-PARTY LICENSES`.
- The new subcommand `check-npm-deps <package.json>`: the file has no
  `dependencies`, `peerDependencies`, `bundleDependencies`, or
  `bundledDependencies`. Each name in `optionalDependencies` is one of the
  four platform packages.
- Each archive and each npm package keeps the `LICENSE` file of the
  repository root, byte for byte. A test compares them.

### The security review (STORY-63)

The reviewer does the review on one commit of `main`, after steps 1 to 3
of "Build order" merge. For each item, the record gives "pass", or a
finding with its CVSS v3.1 vector, its level (decision 11), and its fix
pull request or its issue.

The key (CONSTRAINT-02, QS-5, CV-6):

1. Storage (`src/config/fs.rs`, `src/config/setup.rs`,
   `src/config/change.rs`): the folder is made with mode 0700 and the file
   with mode 0600. No other user can read a temporary file or an old copy
   of the file at any time of a write. A symbolic link or a special file is
   refused.
2. Read (`src/config/fs.rs`, `src/config.rs`): each read of the file checks
   the mode of the open file (`fstat`), also a read for the base URL or the
   model name only. No race between the check and the read lets jag read
   another file. The keys from `JAG_TYPESAFE_API_KEY`, `TYPESAFE_API_KEY`,
   and `JAG_GATEWAY_API_KEY` follow the order of CV-7.
3. Hold: the key type never prints itself, in `Display`, in `Debug`, in an
   error, in a panic message, in `--json` output, or in the output of the
   readiness check (`src/check.rs`). The test of each error path (QS-5)
   exists and passes.
4. Send (`src/scorer/client.rs`): a TypeSafe key goes only to the TypeSafe
   API, and only a gateway key goes to a gateway (ADR-0017). Each call uses
   TLS with the platform verifier (ADR-0014). No redirect is followed
   (`max_redirects(0)`). A base URL with the scheme `http` cannot send a
   key with no TLS (ADR-0015, ADR-0016).
5. Removal (ADR-0006): the removal of the key leaves no copy in the config
   folder.
6. The process list and the terminal: no option takes the key, and
   `--setup` reads it with no echo (`rpassword`).

The other code:

7. `unsafe_code = "forbid"` is set, and no build script downloads or runs
   other code.
8. Untrusted input: file content, file names, and the reply of the model
   service. A reply that is not valid, or is very large, gives a failure of
   CV-1, not a panic or an unbounded allocation. Text from the model reply
   cannot send terminal control sequences to the terminal of the user.
9. jag sends no telemetry and writes no log file (CV-8).
10. The npm launcher (`npm/jag/bin/jag.js`): it starts only the binary of
    the platform package, with no shell. No package has an install script
    (FEATURE-18 spec decision 13).

The workflows and the repository:

11. `release.yml`: each job has only the permissions that it needs
    (FEATURE-18 spec decision 28). Only the publish jobs have
    `id-token: write`, and only in the environment `release`. The
    repository stores no publish token. Each action has a full commit SHA.
    No job uses a cache. No `run:` text holds a `${{ }}` value that a
    person outside the team can set. The published files are the scanned
    files. A dry run has no secret, no `id-token`, and no environment.
12. `private-data.yml` (`pull_request_target`): no step runs, sources, or
    reads as config a file of the folder `pr`. The secret is never printed.
13. `ci.yml`, `adr-gate.yml`, `go-public.yml`, `private-data-tests.yml`,
    and `deps.yml`: `contents: read`, pinned actions, and no secret on
    `pull_request`.
14. The repository settings: the ruleset of `main`, the tag ruleset and the
    environment `release` (FEATURE-18 step U2, when they exist), the
    Actions policy 7541, the approval setting for fork workflows
    (FEATURE-19 spec decision 23), and secret scanning with push
    protection.
15. The dependencies: the last run of `deps.yml` on the reviewed commit is
    green. Each `ignore` entry of `deny.toml` follows decision 11.

### The license audit (STORY-64)

The implementer of step 3 writes the license section of the record, on the
commit of the green dry run:

- The run of `cargo-deny check licenses` and of `check-npm-deps` passes
  (the run URL of `deps.yml`).
- The six licenses of decision 14, one line each: permissive, and the
  notice goes with each copy. So each one is compatible with MIT.
- A search of the crate sources of the four targets finds no `NOTICE` file,
  or the record names the crate, and the template adds the text of its
  `NOTICE` file.
- The dry run of `release.yml` on that commit is green: the archives, the
  crate, the npm packages, the metadata checks, and the scan. The record
  gives the run URL.
- `gh api repos/ai-ready-future/jag --jq .license.spdx_id` gives `MIT`.

### Before the first tag

Chores does this list on the commit of the tag, before the push of the tag
(FEATURE-18 step U6), and writes the result into the record:

1. The run of `deps.yml` on the commit is green.
2. The release dry run on the commit is green (U6 already asks for it).
3. The security review: `git diff --stat <reviewed commit> <commit>` for
   `src/`, `npm/jag/bin/`, `.github/workflows/`, `scripts/release/`,
   `scripts/deps/`, `deny.toml`, `Cargo.toml`, and `Cargo.lock`. When a file
   changed, the reviewer reviews the change by the list of "The security
   review" and adds a dated line to the record.
4. The check of decision 6 passes.

A record pull request changes only `docs/release-readiness.md`. When it
merges after the list, `git diff --name-only <commit> <new commit>` must
show only that file. Then the tag goes on the new commit.

The job `deps` of the release run checks the advisories again on the
commit of the tag.

### Acceptance criteria and their tests

| Criterion | Test or check | Where the result is |
|---|---|---|
| CRITERION-105 | The tracking issue is closed with a comment that gives the date and the words of the user. The check of decision 6 passes | `docs/release-readiness.md`, section "Use period" |
| CRITERION-106 | `scripts/deps/check.sh` passes in `deps.yml` on the commit, and in the job `deps` of the release run of the tag. Each `ignore` entry has an open `security` issue and a level below high | The run URLs, in the section "Dependency audit" |
| CRITERION-107 | Each item of "The security review" is "pass", or a finding that decision 11 allows: no open critical or high finding, and each medium or low finding has a merged fix or an open `security` issue | The section "Security review" |
| CRITERION-108 | `cargo-deny check licenses` and `check-npm-deps` pass. The record shows why each license is compatible | The section "Licenses" |
| CRITERION-109 | The dry run on the commit is green: `check-npm-tarball` (members and metadata), `check-crate-metadata`, `check-crate-list`, and `check-notices`. `release.py archive` writes the four members (the cases of `test_release.py`). `LICENSE` is at the root of the repository | The section "Notices and metadata" |

### Tests

New cases in `scripts/release/test_release.py`:

- `archive` with `--notices`: the members are `jag`, `README.md`,
  `LICENSE`, and `THIRD-PARTY-LICENSES`, and the new member has mode 0644.
  The same input gives the same archive.
- `npm-platform` with `--notices`: `files` names `THIRD-PARTY-LICENSES`,
  and the file is in the folder.
- `check-npm-tarball`: a platform tarball with no `THIRD-PARTY-LICENSES`
  fails. A tarball with a wrong `license`, `repository.url`, or
  `description` fails.
- `check-crate-metadata`: a crate with a wrong `license` fails, and the
  crate of the repository passes.
- `check-notices`: an empty file fails, and a file with a wrong first line
  fails.
- `check-npm-deps`: a `dependencies` entry fails, an optional dependency
  that is not a platform package fails, and `npm/jag/package.json` passes.
- The `LICENSE` file of each package is the `LICENSE` file of the root.

### The record

`docs/release-readiness.md` holds these sections: "Use period",
"Dependency audit", "Security review", "Licenses", "Notices and metadata",
and "Before the first tag". Each section starts with "Not done". It holds
no key, no address, and no attack steps.

## Build order

0. Chores: the labels (decision 25), the tracking issue, and the first
   binary for the user (STORY-62, steps 1 to 3 of "The use period"). The use
   period then runs at the same time as steps 1 to 5.
1. STORY-63, one implementer pull request: `rustls` 0.23.46 or later in
   `Cargo.lock` (decision 24), `deny.toml`, `scripts/deps/check.sh`,
   `deps.yml`, the job `deps` of `release.yml`, `check-npm-deps`, and
   `docs/release-readiness.md` with empty sections.
2. The implementer of step 1, after the merge: the section "Dependency
   audit" of the record, with the URL of the green run of `deps.yml` on
   `main`.
3. STORY-64, one implementer pull request, after the architect updates the
   line of ADR-0020 (decision 23): `about.toml`, `notices.hbs`, the job
   `notices`, the changes of `release.py`, the tests, and "Before the first
   tag" in `scripts/release/README.md`. Then the license audit and its
   record sections.
4. STORY-63: the security review on `main` after steps 1 to 3 (decision
   18), the fixes of each critical or high finding, and the issues of the
   other findings.
5. STORY-62: the end of the period, the check of decision 6, and its record.
6. The planner sets each story `done` by decision 21. When FEATURE-22 is
   `done`, FEATURE-18 is no longer blocked, and its user steps U3 to U6 can
   start. Before U6: "Before the first tag".

Steps 1 and 3 can be one pull request when one implementer does both.

## Assumptions

- Decision 4 (the binary of a dry run): from FEATURE-18 spec decision 5,
  `release.yml`, and the build row of `docs/adlc.md`.
- Decision 13 (the npm packages have no third-party dependency): from
  `npm/jag/package.json` and `release.py` (`npm_platform_package` writes
  no dependency).
- Decision 15 (the repository, the crate, and `@airf/jag` need no
  third-party notice): they hold no code of a dependency. `Cargo.lock`
  holds only names, versions, and checksums. Cargo downloads each crate
  from crates.io with its own license.
- Decision 11 (CVSS v3.1 bands): RustSec gives a CVSS v3.1 vector, and the
  four advisories of decision 24 have one. The bands are the bands of the
  CVSS v3.1 specification.
- No crate of the tree has a `NOTICE` file: from a search of the crate
  sources in `rust:1.99` on 2026-10-15.
- The checks of this spec read GitHub with `gh api` (REST). On 2026-10-15
  the GraphQL limit of the account was reached, and REST still worked.

## Risks

- A new advisory fails the job `deps` of the release run. Signal: the job
  `deps` of the tag run is red. Response: a fix, or for medium or low, an
  `ignore` entry and an issue (decision 11). Then "Re-run failed jobs".
- A critical or high advisory has no fixed version. Signal: cargo-deny gives
  no "Solution". Response: the publish waits. The orchestrator sends an
  `ASK` with the options, for example a change of the crate.
- The notice file holds text that the scan of the packages finds, for
  example a name or an address in a license text of a dependency. Signal:
  the job `scan` fails at a line of the member `THIRD-PARTY-LICENSES`.
  Response: do not change the license text. The orchestrator sends an
  `ASK`. The test file of 2026-10-15 was not given to the scan.
- cargo-about cannot find the license of a new crate. Signal: the job
  `notices` fails (`--fail`). Response: a `clarify` entry in `about.toml`.
- The use period finds many bugs and lasts long. Signal: many open issues
  with `use-period`. Response: the user decides the end. Open issues meet
  CRITERION-105.
- The artifact of the binary expires. Signal: `gh run download` fails after
  14 days. Response: chores starts a new dry run.
- The review misses a finding. Signal: a later report. Mitigation: a second
  reviewer reviews the record, and the check before the first tag reviews
  each change after the review.
- The line of ADR-0020 is not updated before the STORY-64 build. Signal:
  the ADR adherence review of that pull request names ADR-0020. Response:
  the architect does decision 23 first. When the architect sends an `ASK`,
  STORY-64 goes back to `backlog` by rule 11 until the `ANSWER`.
- Before step U3, npm audit sees only the main package ("audited 1
  package"), because the platform packages are not on npm. After U3, it
  sees all five. A platform package has no dependency, so the result does
  not change.

## Out of scope

- The user steps U1 to U6 and the publish: FEATURE-18.
- The fix of each bug of the use period: each bug gets its own fix through
  the orchestrator.
- Other compliance topics, for example trademarks, export rules for the TLS
  code, the terms of the TypeSafe service, and a privacy notice. CRITERION-108
  and CRITERION-109 do not name them. A new story can add them when the user
  asks.
- A `SECURITY.md` file and private vulnerability reporting. The review can
  open an issue for them.
- A required status check for `deps`.
- cargo-audit: cargo-deny covers its check (decision 8).
- The macOS signature: FEATURE-18 spec decision 26.
- `jag-bench`: FEATURE-18 spec decision 27.
