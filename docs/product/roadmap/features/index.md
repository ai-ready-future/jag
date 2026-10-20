# features

## MILESTONE-01: Make it work

* [FEATURE-27: README examples for people](FEATURE-27.md) - The README gives worked examples that show a person how to search with jag at the shell, with the command to copy and what jag shows.

## MILESTONE-02: Make it good

* [FEATURE-18: Install jag from GitHub Releases, crates.io, or npm](FEATURE-18.md) - One version tag builds the jag binary for four platforms and publishes it on GitHub Releases, crates.io, and npm with the same version (ADR-0010).
* [FEATURE-22: Check that jag is ready before the first publish](FEATURE-22.md) - Before the first publish, the user uses jag on real work and fixes the bugs, and audits find no open high security finding and no license or compliance problem.
* [FEATURE-25: Copy the same teaching as a snippet](FEATURE-25.md) - For an agent that can't install a skill, the agent builder copies a snippet that holds the same teaching as the jag skill.
* [FEATURE-26: Prove that an agent with the skill uses jag well](FEATURE-26.md) - A repeatable test runs an agent with the jag skill and shows that it uses jag in place of rg, sets the limit, reads the results, and gives the user the cause and the fix when jag fails.
