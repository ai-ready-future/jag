# Release

One version tag publishes jag on three channels with the same version:

- GitHub Releases: one archive and one checksum file for each of four
  platforms.
- crates.io: the crate `jag-grep`.
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
  `jag-<version>-<target>/` with `jag`, `README.md`, and `LICENSE`.
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
the jobs `prepare`, `npm-main`, `build` (four platforms), `crate`, and
`scan`. It runs `cargo publish --dry-run` and publishes nothing. It needs no
environment and no registry account.

The release files are `release.yml`, `scripts/release/`, `npm/`,
`Cargo.toml`, `Cargo.lock`, and `rust-toolchain.toml`.

## The tests of the tools

```
$ python3 -I -m unittest discover -s scripts/release -p 'test_*.py'
$ node --test npm/jag/test/launcher.test.js
```

The job `release-tools` of `ci.yml` runs both on each pull request.

## A release

Only the user pushes a `v*` tag, or a session on a TASK of the orchestrator
that names the version. The jobs run in this order:

1. `prepare`, `npm-main`, `build`, `crate`, and `scan`: the dry run.
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

### A bad version

A published version cannot change. A fix that needs new code needs a new
patch version. Mark the bad version:

```
$ cargo yank --version <version> jag-grep
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
  - crates.io: open the crate `jag-grep`, then Settings, then Trusted
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

After the first release, mark the empty versions:

```
$ cargo yank --version 0.0.0 jag-grep
$ npm deprecate <package>@0.0.0 "This version holds no program."
```
