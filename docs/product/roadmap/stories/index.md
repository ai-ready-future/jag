# stories

## MILESTONE-02: Make it good

* [STORY-45: Run the held-out evaluation before release](STORY-45.md) - As an agent builder (PERSONA-01), I want the held-out evaluation run done before release 1, so that I can trust that jag answers better than grep.
* [STORY-48: Download a prebuilt binary from GitHub Releases](STORY-48.md) - As a shell power user (PERSONA-04), I want a prebuilt binary for my platform on GitHub Releases, so that I can install jag in seconds with no runtime.
* [STORY-49: Install jag with cargo from crates.io](STORY-49.md) - As a shell power user (PERSONA-04), I want to install jag with cargo install, so that I can build jag from source with the Rust tools I use.
* [STORY-50: Install or run jag with npm](STORY-50.md) - As an agent builder (PERSONA-01), I want to install jag with npm or run it with npx, so that my agent setup gets jag with the tools it already uses.
* [STORY-51: Publish each channel from one tag](STORY-51.md) - As an agent builder (PERSONA-01), I want one version tag to publish jag on each channel with the same version and no private data, so that each channel gives the same jag.
* [STORY-62: Use jag on real work before the first publish](STORY-62.md) - As a shell power user (PERSONA-04), I want the maintainer to use jag on real work for a period before the first publish, so that the first release has no known bug that this use found.
* [STORY-63: Audit jag for security vulnerabilities before the first publish](STORY-63.md) - As an agent builder (PERSONA-01), I want a security audit of the code, the dependencies, and the release workflow before the first publish, so that the first release has no known high vulnerability.
* [STORY-64: Audit the license and the compliance before the first publish](STORY-64.md) - As an agent builder (PERSONA-01), I want a license and compliance audit before the first publish, so that I can use jag and its packages with no license problem.
* [STORY-65: Compare rg and jag output on a parity test set](STORY-65.md) - As a shell power user (PERSONA-04), I want rg and jag with the same arguments to give the same bytes, in a pipe and with color, so that I can use jag in place of rg.
* [STORY-66: Run the rg parity cases as e2e tests in CI](STORY-66.md) - As a shell power user (PERSONA-04), I want the rg parity cases to run again as automated e2e tests in CI, so that a later change cannot break the parity with no failed check.
