# Release

One version tag publishes jag on three channels with the same version:

- GitHub Releases: one archive and one checksum file for each of four
  platforms.
- crates.io: the crate `jag-cli`.
- npm: the package `@airf/jag` and four packages for the platforms.

The workflow is `.github/workflows/release.yml`. The steps are in
`release.py`. The spec is
`docs/superpowers/specs/2026-10-14-STORY-48-install-jag-from-github-releases-crates-io-or-npm-design.md`.
The names of the archives and the npm packages are in
`docs/adr/0020-name-the-release-archives-and-the-npm-platform-packages-by.md`.

## The names

| Platform | Rust target | Runner | npm package |
|---|---|---|---|
| Linux x86_64 | `x86_64-unknown-linux-musl` | `ubuntu-24.04` | `@airf/jag-linux-x64` |
| Linux arm64 | `aarch64-unknown-linux-musl` | `ubuntu-24.04-arm` | `@airf/jag-linux-arm64` |
| macOS arm64 | `aarch64-apple-darwin` | `macos-15` | `@airf/jag-darwin-arm64` |
| macOS x86_64 | `x86_64-apple-darwin` | `macos-15-intel` | `@airf/jag-darwin-x64` |

- Archive: `jag-<version>-<target>.tar.gz`. It holds the folder
  `jag-<version>-<target>/` with `jag`, `README.md`, `LICENSE`, and
  `THIRD-PARTY-LICENSES`. Each platform npm package holds the same four
  files (and `package.json`). The crate and `@airf/jag` hold no third-party
  code, so they do not hold `THIRD-PARTY-LICENSES`.
- `THIRD-PARTY-LICENSES`: the license texts of the Rust crates in the
  binary. The job `notices` makes the file at release time with cargo-about
  0.9.2 (`about.toml` and `notices.hbs` in this folder). It is not stored in
  the repository. The same input gives the same bytes. If the job fails
  because cargo-about cannot find the license of a new crate, add a `clarify`
  entry to `about.toml` in a pull request.
- Checksum: `jag-<version>-<target>.tar.gz.sha256`. One line: the SHA-256 in
  hex, two spaces, and the archive name.

## The version

`version` in `Cargo.toml` is the one source. To change it, change `version`
in `Cargo.toml`, run `cargo check` so that `Cargo.lock` changes too, and
commit both in a normal pull request. Only the form `MAJOR.MINOR.PATCH` is
valid. The tag must be `v` and that version, and its commit must be on
`main`.

## A dry run

A pull request that changes a release file starts a dry run. A manual run of
the workflow on `main` (`workflow_dispatch`) does the same. A dry run runs
the jobs `prepare`, `npm-main`, `notices`, `build` (four platforms),
`crate`, and `scan`. It also runs the job `deps` (see "The dependency
checks"). It runs `cargo publish --dry-run` and publishes nothing. It needs
no environment and no registry account. The step "Scan the packages" of the job `scan` runs
only for a tag and for a manual run on `main`; a dry run of a pull request
skips it, because it gets no secret.

The release files are `release.yml`, `scripts/release/`, `scripts/deps/`,
`npm/`, `Cargo.toml`, `Cargo.lock`, `deny.toml`, and `rust-toolchain.toml`.

## The dependency checks

`scripts/deps/check.sh` checks the dependencies of jag. It runs on each pull
request and on each push to `main` (the workflow `deps.yml`), and in the job
`deps` of each release run. It is not a required check: a new advisory can
fail it with no change in the pull request. It runs:

1. `cargo-deny` (version 0.20.2, checked with a fixed SHA-256) with the
   policy in `deny.toml`: the Rust advisories, the licenses, the bans, and the
   sources, for the four release targets, with no dev-dependency.
2. `npm audit` of `npm/jag/package.json`, in a new folder.
3. `release.py check-npm-deps`: the npm packages have no third-party
   dependency.

```
$ scripts/deps/check.sh
```

The script needs `cargo`, `npm`, `curl`, `tar`, and `sha256sum`.

The level of a finding comes from its CVSS v3.1 base score: critical 9.0 to
10.0, high 7.0 to 8.9, medium 4.0 to 6.9, low 0.1 to 3.9. A critical or high
finding blocks the first publish until a fix is on `main`. For a medium or
low RustSec advisory, add an `ignore` entry to `deny.toml` with the number of
an open issue with the label `security` in `reason`. The job `deps` is a
`needs` of `github-draft`, so a release with a finding stops before the draft.

## The tests of the tools

```
$ python3 -I -m unittest discover -s scripts/release -p 'test_*.py'
$ node --test npm/jag/test/launcher.test.js
```

The job `release-tools` of `ci.yml` runs both on each pull request.

## A release

Only the user pushes a `v*` tag, or a session on a TASK of the orchestrator
that names the version. The jobs run in this order:

1. `prepare`, `npm-main`, `notices`, `build`, `crate`, `deps`, and `scan`:
   the dry run.
2. `github-draft`: a draft GitHub Release with the 8 files.
3. `crates-io`: `cargo publish` with trusted publishing.
4. `npm`: the four platform packages, then the main package.
5. `verify`: on each of the four runners, `npm install`, `npx`, and (Linux
   x86_64 only) `cargo install`.
6. `github-publish`: checks the files of the draft, then publishes it.

### A re-run

Each publish job asks its channel for the version first. When the channel
has the version, the job skips the publish. So "Re-run failed jobs" ends a
release that stopped part way. A new version can take some minutes to show on
a registry. If a re-run fails because the version exists, wait some minutes
and run it again.

Use only "Re-run failed jobs". "Re-run all jobs" builds the binaries again,
and the draft then gets binaries that are not the binaries on npm.

`first-publish.sh` also skips a version that a registry has already, so a
second run ends a first publish that stopped part way.

### A bad version

A published version cannot change. A fix that needs new code needs a new
patch version. Mark the bad version:

```
$ cargo yank --version <version> jag-cli
$ npm deprecate @airf/jag@<version> "<reason>"
```

Do the same for the four platform packages.

## The user steps before the first tag

The chores role guides the user through these steps. Each step uses the
account that the user logs in with.

- **U1.** The npm organization `airf` exists, and the user owns it. Check:
  `npm org ls airf`.
- **U2.** In the repository, after the repository is public:
  - the environment `release`, with a deployment rule for the tags `v*` only
  - a tag ruleset for `refs/tags/v*` with the rules "update" and "deletion",
    and no bypass
  - immutable releases on

  Check: `gh api repos/ai-ready-future/jag/environments/release` and
  `gh api repos/ai-ready-future/jag/rulesets`.
- **U3.** The first publish. On the machine of the user, run
  `scripts/release/first-publish.sh`. It makes an empty version 0.0.0 of the
  crate and of the five npm packages. First run
  `scripts/release/first-publish.sh --dry-run`: it publishes nothing. The
  real run needs a crates.io token that the user makes for this step only
  (`CARGO_REGISTRY_TOKEN`, or the script asks for it). npm asks for the 2FA
  code at each publish. The script writes no token and reads no key file.
- **U4.** The trusted publishers.
  - crates.io: open the crate `jag-cli`, then Settings, then Trusted
    Publishing. Add a GitHub publisher: owner `ai-ready-future`, repository
    `jag`, workflow `release.yml`, environment `release`.
  - npm: use npm 11.15.0 or later, with 2FA on. For each of the five
    packages:

    ```
    $ npm trust github <package> --file release.yml --repo ai-ready-future/jag --env release --allow-publish
    ```
- **U5.** Close the token routes.
  - crates.io: in the settings of the crate, turn on "Trusted Publishing
    only". Then revoke the token of U3.
  - npm: in the settings of each package, under Publishing access, choose
    "Require two-factor authentication and disallow tokens".
  - Check: `gh secret list` and `gh secret list --env release` show no
    publish token.
- **U6.** The first tag, in the 2 days after U4. A new npm trusted publisher
  expires when no publish uses it in 2 days. Before the tag, the dry run on
  `main` must be green. Then:

  ```
  $ git tag -a v<version> -m "jag <version>" <commit of main>
  $ git push origin v<version>
  ```

### Before the first tag

Before U6, chores does this list on the commit of the tag, and writes the
result in `docs/release-readiness.md`:

1. The run of `deps.yml` on the commit is green.
2. The release dry run on the commit is green. It checks the archives, the
   platform packages (`check-npm-tarball`), the crate (`check-crate-metadata`
   and `check-crate-list`), and the notice file (`check-notices`). After a
   change of `Cargo.lock` or `release.yml`, run a dry run by hand on `main`
   (`gh workflow run release.yml --ref main`): the dry run of a pull request
   does not scan the built files, because it gets no `PRIVATE_PATTERNS`.
3. The security review: compare the commit with the reviewed commit
   (`git diff --stat <reviewed commit> <commit>`) for `src/`, `npm/`,
   `.github/workflows/`, `scripts/release/`, `scripts/deps/`, `deny.toml`,
   `Cargo.toml`, `Cargo.lock`, and `rust-toolchain.toml`. When a file changed,
   a reviewer reviews the change and adds a dated line to the record.
4. Each issue with the labels `bug` and `use-period` is open, or closed as
   `completed` by a merged pull request or a commit on `main`, or closed as
   not planned with the label `duplicate` or `invalid`.

A pull request that changes only `docs/release-readiness.md` can merge after
the list. Then the files that differ between the commit and the new commit
(`git diff --name-only <commit> <new commit>`) must all be in `docs/`. If
not, do the list again on the new commit. The tag then goes on the new
commit. The job `deps` of the release run checks the advisories again on the
commit of the tag.

After the first release, mark the empty versions:

```
$ cargo yank --version 0.0.0 jag-cli
$ npm deprecate <package>@0.0.0 "This version holds no program."
```
