# features

## MILESTONE-02: Make it good

* [FEATURE-18: Install jag from GitHub Releases, crates.io, or npm](FEATURE-18.md) - One version tag builds the jag binary for four platforms and publishes it on GitHub Releases, crates.io, and npm with the same version (ADR-0010).
* [FEATURE-21: Send the model calls through an AI gateway](FEATURE-21.md) - The user can set a base URL and a model name, so that jag sends the same model calls through an AI gateway. With no setting, jag calls TypeSafe directly.
* [FEATURE-29: The config file holds each non-secret setting, and setup shows each default](FEATURE-29.md) - The user sets each non-secret setting of jag in the config file one time, and jag --setup shows each missing setting as a comment line with its default.

## MILESTONE-04: Local model option

* [FEATURE-30: Score with a Clef server that the user runs](FEATURE-30.md) - The user sets the base URL to a Clef or Clef-Flash server that the user runs, and jag scores each search there, with no TypeSafe key.
* [FEATURE-31: Hold private text back from the model](FEATURE-31.md) - Before each model call, jag checks the question and each snippet against a list of private patterns, and does not send a match.
