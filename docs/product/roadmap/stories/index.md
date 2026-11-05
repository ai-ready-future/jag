# stories

## MILESTONE-02: Make it good

* [STORY-45: Run the held-out evaluation before release](STORY-45.md) - As an agent builder (PERSONA-01), I want the maintainer to run the Rust jag once on the private held-out split before release 1, so that I can trust that jag answers better than grep.
* [STORY-48: Download a prebuilt binary from GitHub Releases](STORY-48.md) - As a shell power user (PERSONA-04), I want a prebuilt binary for my platform on GitHub Releases, so that I can install jag in seconds with no runtime.
* [STORY-49: Install jag with cargo from crates.io](STORY-49.md) - As a shell power user (PERSONA-04), I want to install jag with cargo install, so that I can build jag from source with the Rust tools I use.
* [STORY-50: Install or run jag with npm](STORY-50.md) - As an agent builder (PERSONA-01), I want to install jag with npm or run it with npx, so that my agent setup gets jag with the tools it already uses.
* [STORY-51: Publish each channel from one tag](STORY-51.md) - As an agent builder (PERSONA-01), I want one version tag to publish jag on each channel with the same version and no private data, so that each channel gives the same jag.
* [STORY-81: Score with the old prompt as a new prompt version](STORY-81.md) - As the maintainer, I want jag to score with the prompt of the old tool as a new prompt version, so that jag can reach the release bar before the held-out run.
* [STORY-84: Set each non-secret setting in the config file](STORY-84.md) - As an agent builder, I want each non-secret setting of jag as a key in the config file, so that I set it one time, and not in each shell or job.
* [STORY-85: jag --setup shows each missing setting as a comment line with its default](STORY-85.md) - As an agent builder, I want jag --setup to show each non-secret setting in the config file as a comment line with its default, so that I see each setting.
* [STORY-86: Call the gateway base URL with no key when the user sets no gateway key](STORY-86.md) - As an operator, I want jag to call my model server with no key when I set no gateway key, so that a server with no key check works with no extra setting.
