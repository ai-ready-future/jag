# features

## MILESTONE-02: Make it good

* [FEATURE-18: Install jag from GitHub Releases, crates.io, or npm](FEATURE-18.md) - One version tag builds the jag binary for four platforms and publishes it on GitHub Releases, crates.io, and npm with the same version (ADR-0010).

## MILESTONE-04: Local model option

* [FEATURE-30: Score with a Clef server that the user runs](FEATURE-30.md) - The user sets the base URL to a Clef or Clef-Flash server that the user runs, and jag scores each search there, with no TypeSafe key.
* [FEATURE-31: Hold private text back from the model](FEATURE-31.md) - Before each model call, jag checks the question and each snippet against a list of private patterns, and does not send a match.
* [FEATURE-32: Say which text leaves the machine](FEATURE-32.md) - Before a search, jag shows the text that it sends to the model service, and after each search it reports which text left the machine.
