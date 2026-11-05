# features

## MILESTONE-02: Make it good

* [FEATURE-18: Install jag from GitHub Releases, crates.io, or npm](FEATURE-18.md) - One version tag builds the jag binary for four platforms and publishes it on GitHub Releases, crates.io, and npm with the same version (ADR-0010).
* [FEATURE-21: Send the model calls through an AI gateway](FEATURE-21.md) - The user can set a base URL and a model name, so that jag sends the same model calls through an AI gateway. With no setting, jag calls TypeSafe directly.
* [FEATURE-29: The config file holds each non-secret setting, and setup shows each default](FEATURE-29.md) - The user sets each non-secret setting of jag in the config file one time, and jag --setup shows each missing setting as a comment line with its default.
