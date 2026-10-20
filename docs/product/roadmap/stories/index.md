# stories

## MILESTONE-01: Make it work

* [STORY-76: Learn jag from worked examples in the README](STORY-76.md) - As a person at the shell (PERSONA-02, PERSONA-03, PERSONA-04), I want worked examples in the README, so that I can copy a command and see what jag shows before I read each option.

## MILESTONE-02: Make it good

* [STORY-45: Run the held-out evaluation before release](STORY-45.md) - As an agent builder (PERSONA-01), I want the held-out evaluation run done before release 1, so that I can trust that jag answers better than grep.
* [STORY-48: Download a prebuilt binary from GitHub Releases](STORY-48.md) - As a shell power user (PERSONA-04), I want a prebuilt binary for my platform on GitHub Releases, so that I can install jag in seconds with no runtime.
* [STORY-49: Install jag with cargo from crates.io](STORY-49.md) - As a shell power user (PERSONA-04), I want to install jag with cargo install, so that I can build jag from source with the Rust tools I use.
* [STORY-50: Install or run jag with npm](STORY-50.md) - As an agent builder (PERSONA-01), I want to install jag with npm or run it with npx, so that my agent setup gets jag with the tools it already uses.
* [STORY-51: Publish each channel from one tag](STORY-51.md) - As an agent builder (PERSONA-01), I want one version tag to publish jag on each channel with the same version and no private data, so that each channel gives the same jag.
* [STORY-62: Use jag on real work before the first publish](STORY-62.md) - As a shell power user (PERSONA-04), I want the maintainer to use jag on real work for a period before the first publish, so that the first release has no known bug that this use found.
* [STORY-68: Install the skill with one command](STORY-68.md) - As an agent builder (PERSONA-01), I want to install the jag skill with one command from the README, so that my agent learns jag without manual setup.
* [STORY-69: The agent uses jag in place of rg and sets a limit](STORY-69.md) - As an agent builder (PERSONA-01), I want my agent to use jag in place of rg and to set a limit on the text, so that the agent does not fill its context.
* [STORY-70: The agent reads the structured output](STORY-70.md) - As an agent builder (PERSONA-01), I want my agent to read the structured output and the failures of jag, so that it uses the file and line of each result.
* [STORY-71: The agent tells the user and falls back](STORY-71.md) - As an agent builder (PERSONA-01), I want my agent to tell the user what is wrong and continue with its own search tool when jag cannot run, so that the user is never blocked.
* [STORY-72: Copy the snippet from the README](STORY-72.md) - As an agent builder (PERSONA-01), I want to copy a snippet from the README into an agent that cannot install a skill, so that this agent also learns jag.
* [STORY-73: The snippet and the skill share one source](STORY-73.md) - As an agent builder (PERSONA-01), I want the snippet and the skill to come from one source, so that they always teach the same behavior.
* [STORY-74: Run one repeatable agent test](STORY-74.md) - As an agent builder (PERSONA-01), I want to run one repeatable test that gives an agent with the jag skill a set of tasks, so that I can check the skill at any time.
* [STORY-75: The test records what the agent did](STORY-75.md) - As an agent builder (PERSONA-01), I want the test to record the choices of the agent for each task, so that I can see if the agent uses jag well.
