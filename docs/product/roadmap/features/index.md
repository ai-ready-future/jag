# features

## MILESTONE-02: Make it good

* [FEATURE-18: Install jag from GitHub Releases, crates.io, or npm](FEATURE-18.md) - One version tag builds the jag binary for four platforms and publishes it on GitHub Releases, crates.io, and npm with the same version (ADR-0010).
* [FEATURE-22: Check that jag is ready before the first publish](FEATURE-22.md) - Before the first publish, the user uses jag on real work and fixes the bugs, and audits find no open high security finding and no license or compliance problem.
* [FEATURE-23: Give the same output as rg for the same arguments](FEATURE-23.md) - An rg command and a jag command with the same arguments and no question give the same bytes, the same color, and the same exit code, and e2e tests check this in CI.
