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

## MILESTONE-04: Local model option

* [STORY-87: A search works when a call is too long for the server](STORY-87.md) - As an operator, I want a search to work when a call is too long for my Clef server, so that a server that reads less text in one call still gives a result.
* [STORY-88: Show the cost and the failure of a Clef server correctly](STORY-88.md) - As an operator, I want jag to mark the cost of a Clef search as not available, and to give the host and the status when the server fails, so that I see the truth.
* [STORY-89: The report of the text that leaves the machine names the host](STORY-89.md) - As a knowledge seeker, I want the report of the text that leaves the machine to name the host that gets it, so that I know where my text goes.
* [STORY-90: The README tells how to use a Clef server](STORY-90.md) - As an operator, I want the README to tell how to use jag with a Clef server, and what jag does not promise for it, so that I set it up and know the limits.
* [STORY-91: Hold back each snippet that holds a key, a token or a password](STORY-91.md) - As an operator, I want jag to hold back each snippet that holds a key, a token or a password, so that no secret goes to a model service.
* [STORY-92: Add private patterns of the user in the config file](STORY-92.md) - As an operator, I want to add my own private patterns in the config file, for example for patient IDs, so that jag also holds back the private text of my team.
* [STORY-93: Turn the private-pattern check off with one setting](STORY-93.md) - As an operator, I want one setting that turns the private-pattern check off, for example for a Clef server that I trust, so that jag scores each snippet.
* [STORY-94: The preview lists the lines that the check holds back](STORY-94.md) - As a knowledge seeker, I want the preview of the text that leaves the machine to list each line that the check holds back, so that I see what jag does not send.
