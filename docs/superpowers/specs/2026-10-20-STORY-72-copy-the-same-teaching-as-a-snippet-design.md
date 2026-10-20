---
title: "Copy the same teaching as a snippet: the README snippet and the check that keeps it equal to the skill body"
feature: FEATURE-25
stories: [STORY-72, STORY-73]
kind: build
gh_issue: 185
lifecycle_state: draft
session: 4acb9f7a-4603-41fb-8e74-395e5117e288
---

# Copy the same teaching as a snippet: the README snippet and the check that keeps it equal to the skill body

## Problem

Some agents cannot install a skill. FEATURE-25: "The README shows a snippet
that the agent builder copies into the instructions of an agent that can't
install a skill. The snippet and the skill come from one source, so they
always teach the same behavior."

CAPABILITY-11: "jag ships one set of agent instructions in two forms. The
agent builder installs the skill into each agent that the install command
supports, such as Claude Code. For an agent that can't install a skill, the
agent builder copies the snippet into the instructions of the agent. Both
forms teach the same behavior."

The two Stories:

- STORY-72: "As an agent builder (PERSONA-01), I want to copy a snippet from
  the README into an agent that cannot install a skill, so that this agent
  also learns jag."
  - CRITERION-124: "The README shows a snippet that the agent builder can
    copy."
  - CRITERION-125: "The snippet teaches the same four behaviors as the
    skill. As the skill does, it tells the agent to give the user the cause
    and the fix that jag writes when jag cannot run, and it names no other
    search tool (DECISION-55)."
- STORY-73: "As an agent builder (PERSONA-01), I want the snippet and the
  skill to come from one source, so that they always teach the same
  behavior."
  - CRITERION-126: "The snippet and the skill are made from one source
    file."
  - CRITERION-127: "A change to the source shows in the snippet and in the
    skill."

ADR-0025 (accepted) makes the body of `skills/jag/SKILL.md` the one source.
It gives two things to this spec: "The FEATURE-25 spec sets how the snippet
is made from the body and the check that fails when the snippet does not
agree with the body."

FEATURE-24 (done) wrote the skill, the README install steps, and the content
tests of the body (CRITERION-115 to CRITERION-123).

## Decisions

| # | Decision | Choice | Source |
|---|---|---|---|
| 1 | Specs | One spec for STORY-72 and STORY-73. They cannot ship apart: a snippet with no check is a copy by hand, and ADR-0025 rejects that | Orchestrator TASK for FEATURE-25 ("1 feature = 1 spec = 1 PR"); ADR-0025 (rejected alternative "Two texts written by hand, with a review rule") |
| 2 | The source (CRITERION-126) | The body of `skills/jag/SKILL.md` is the one source. The skill is that file. The README snippet is made from the body. The frontmatter is not in the snippet. No other file holds its own copy of the instructions | ADR-0025 |
| 3 | The body text | The text after the line `---` that closes the frontmatter, with the line breaks at its start and at its end removed. The tests get it from `split` in `tests/agent_skill.rs`, then remove those line breaks. `bodyOf` in `tests/agent_skill/install.test.js` does not change: that test compares whole files and never reads the snippet | Inferred: `tests/agent_skill.rs` (`split`, `body`); `skills/jag/SKILL.md` (one blank line after the frontmatter, no line break at the end of the file) |
| 4 | The place in the README (CRITERION-124) | A new section `## Copy the instructions into an agent`, directly after `## Install` and before `## Store your TypeSafe key`. `## Install` keeps its three parts in the order of ADR-0024 | Inferred: ADR-0024 (the order of `## Install`); `tests/agent_skill.rs` `section` (a section ends at the next line that starts with `## `, also in a fence, so the body cannot go in `## Install` before the cargo line); FEATURE-27 spec (`## Examples` goes after `## Check that jag is ready`) |
| 5 | The text around the snippet | Before the snippet, one paragraph: "If your agent cannot install a skill, copy the text below into the instructions of the agent. It is the text of the jag skill, with no frontmatter. The agent also needs the jag program (step 1 of [Install](#install))." No text after the snippet. This text is not part of the instructions | ADR-0025 ("Text around the snippet, for example how to copy it, is not part of the instructions"); inferred: README line "The jag skill runs the command `jag`, so an agent needs one of the installs above" (FEATURE-24 row 27) |
| 6 | A pointer in `## Install` | Part 2 ("### 2. Add the jag skill to your agent") gets one sentence at its end: "If your agent cannot install a skill, copy the instructions in [Copy the instructions into an agent](#copy-the-instructions-into-an-agent)." The sentence does not start with `$ ` and is not in a fence | Inferred: FEATURE-25 ("For an agent that can't install a skill, the agent builder copies a snippet"); ADR-0025 (text around the snippet); `the_readme_gives_one_install_command` (it checks only for lines that must be present) |
| 7 | The form of the snippet (CRITERION-124) | One fenced code block with a fence of four backticks and the info string `markdown`. The body holds fences of three backticks, and a fence closes only at a fence of the same length or longer. GitHub shows a copy button on the block | Inferred: CommonMark 0.31.2, section 4.5 "Fenced code blocks"; `skills/jag/SKILL.md` (two fenced blocks of three backticks); `README.md` (the info string `yaml` on the blocks that are not shell commands) |
| 8 | The markers | Two HTML comment lines around the block. Start: `<!-- agent-instructions:start (made from skills/jag/SKILL.md by JAG_SNIPPET_BLESS=1 cargo test --locked --test agent_skill, do not edit) -->`. End: `<!-- agent-instructions:end -->`. The README holds each marker one time. A Markdown view shows no HTML comment | Inferred: `scripts/adr/adr_index.py` (`BEGIN` and `END` around the generated region of `docs/adr/README.md`); the roadmap pages (`<!-- STATUS:START (generated, do not edit) -->`) |
| 9 | The region | One function, `snippet_region(body)`, gives the full region: the start marker line, a blank line, the line `` ````markdown ``, the body of row 3, the line `` ```` ``, a blank line, and the end marker line. The make step (row 10) and the check (row 11) call the same function. So a change of a marker or of a fence line also fails the check | Inferred: `scripts/adr/adr_index.py` (one pair of `BEGIN` and `END` constants for `--write` and `--check`) |
| 10 | The make step (CRITERION-126, CRITERION-127) | The test `the_readme_snippet_is_the_skill_body` in `tests/agent_skill.rs`. When the variable `JAG_SNIPPET_BLESS` is set, to any value, the test replaces the region between the markers, the markers included, with `snippet_region(body)`, and it passes. It changes no other byte of `README.md`. It writes a temporary file in the folder of `README.md`, then renames it to `README.md`. So another test that reads `README.md` at the same time reads the old file or the new file, never a part. The name is not `JAG_README_BLESS`, because the FEATURE-27 spec uses that name for a variable that prints and writes no file | Inferred: `tests/rg_parity.rs` (`JAG_PARITY_BLESS`: set to any value, the test writes the expected files); ADR-0021 (the checks are cargo tests, with no second test runner); FEATURE-27 spec (`JAG_README_BLESS=1`) |
| 11 | The check (CRITERION-126) | When `JAG_SNIPPET_BLESS` is not set, the same test fails when the README does not hold exactly one start marker and one end marker after it, or when the region is not equal to `snippet_region(body)`. The failure message gives the number of the first README line that is different, and the fix: `JAG_SNIPPET_BLESS=1 cargo test --locked --test agent_skill`. The test runs in the job `rust` of `ci.yml` and in the job `build` of `release.yml`, with `cargo test --locked`. No workflow changes | ADR-0025 ("the check that fails when the snippet does not agree with the body"); inferred: FEATURE-24 spec row 22; `.github/workflows/ci.yml` and `.github/workflows/release.yml` (`cargo test --locked`) |
| 12 | Rules of the body for the snippet | The body has no line that starts with four or more backticks, because that line closes the block of row 7. The body holds no text `<!-- agent-instructions:`. The test of row 10 checks both, also when it makes the region | Inferred: CommonMark 0.31.2, section 4.5; rows 7 and 8 |
| 13 | A change of the body (CRITERION-127) | A pull request that changes the body also runs the make step and commits `README.md`, in the same pull request. So FEATURE-24 row 26 (a change of jag changes `SKILL.md` in the same pull request) also changes the snippet | ADR-0025 ("A pull request that changes the body also gives the snippet the change, in the same pull request"); FEATURE-24 spec row 26 |
| 14 | The fence lines and `the_readme_gives_one_install_command` | That test changes its "in a fence" state at each line that starts with three backticks. The region adds six such lines: the two lines of the outer fence and the four fence lines of the body. Each fenced block opens and closes, so the count is even and the state after the region is correct. The body holds no line that starts with `$ npx skills add`. The FEATURE-25 build does not change that test | Inferred: `tests/agent_skill.rs` (`the_readme_gives_one_install_command`, the loop over the README lines) |
| 15 | The release | `README.md` ships in the crate (`Cargo.toml` `include`), in `@airf/jag` (`scripts/release/release.py` copies it), and in each release archive. So the body text is on the crates.io page and on the npm page, and the README gets about 5 KB longer. `Cargo.toml`, `npm/`, `scripts/release/`, and `release.yml` do not change. The skill file is still in no crate and no package: the `skills` tool reads it from GitHub | Inferred: `Cargo.toml` (`include`, `readme`); `scripts/release/release.py` (`command_npm_main`); ADR-0020 (the archive holds `README.md`); ADR-0024 (the skill file) |
| 16 | Third-party dependencies | None. The test uses `std` only | ADR-0012 (each new outside crate needs a reason) |
| 17 | Security | The test writes only `README.md`, and only the region between the markers. It reads no key and no file in `~/.config/jag/`. The body holds no key and names no key variable | Inferred: FEATURE-24 spec row 16 |
| 18 | Links in the body | The body holds only absolute `https://` links, so each link also works in pasted text. The FEATURE-24 test `the_skill_frontmatter_is_for_each_agent` already fails for a relative link | ADR-0025 ("no relative link"); FEATURE-24 spec (that test) |
| 19 | A required check | The check is not a required check on `main`. Only `private-data` is | ADR-0021 |

## Design

### Data model and storage: the source and the body

The source is `skills/jag/SKILL.md` (rows 2 and 3). This spec does not change
that file. The body is the text of `split(&skill()).1` in
`tests/agent_skill.rs`, with `trim_matches('\n')`. Today that removes one
line break at the start (the blank line after the frontmatter) and none at
the end (the file has no line break at its end).

The FEATURE-24 tests (CRITERION-117 to CRITERION-123) read the same body.
So the text that they check is the text of the snippet.

### Interfaces: the README section

`README.md` changes in two places.

1. At the end of part 2 of `## Install`, the sentence of row 6.
2. A new section directly after `## Install`, in this form (rows 4, 5, 7, 8,
   and 9). `<body>` is the body of row 3:

~~~
## Copy the instructions into an agent

If your agent cannot install a skill, copy the text below into the
instructions of the agent. It is the text of the jag skill, with no
frontmatter. The agent also needs the jag program (step 1 of
[Install](#install)).

<!-- agent-instructions:start (made from skills/jag/SKILL.md by JAG_SNIPPET_BLESS=1 cargo test --locked --test agent_skill, do not edit) -->

````markdown
<body>
````

<!-- agent-instructions:end -->

## Store your TypeSafe key
~~~

The test code holds the two markers as constants, `SNIPPET_START` and
`SNIPPET_END`. They are the one place in the tests that holds them.

The body holds `##` lines (for example `## What jag is`). In the README they
are in a fence, so a Markdown view does not show them as headings. The
helper `section` in `tests/agent_skill.rs` does not know fences. No test
may call `section(&readme, ...)` for the new section or for a heading of the
body. The new tests find the region by its markers.

### Migration and compatibility

- The first build writes the section with the two markers and nothing
  between them, then runs the make step one time to fill the region.
- Two other changes edit the same files at about the same time: the change
  of row 16 of FEATURE-24 in `skills/jag/SKILL.md`, and the crate name
  `jag-cli` in `README.md` (ADR-0026, CRITERION-79). The pull request that
  merges later merges `main` first, then runs the make step again (row 13).
- After this change, each change of the body runs the make step (row 13).
  The failure message of the check (row 11) gives the command.

### Test strategy

| Criterion | Test | What it proves |
|---|---|---|
| CRITERION-124 | `tests/agent_skill.rs`, `the_readme_shows_the_snippet_after_install`. In `README.md`, the first line after `## Install` that starts with `## ` is `## Copy the instructions into an agent`. After that heading and before the next line that starts with `## ` outside the region, the README holds the start marker, then a line `` ````markdown ``, then a line `` ```` ``, then the end marker. Part 2 of `## Install` holds the link `#copy-the-instructions-into-an-agent` | The README shows the snippet as one block to copy, in the place of row 4, and Install points to it |
| CRITERION-125 | The equality of `the_readme_snippet_is_the_skill_body` (row 11), with the FEATURE-24 tests of the body: `the_skill_says_to_use_jag_in_place_of_rg`, `the_skill_gives_the_command_form`, `the_agent_skill_names_only_options_of_jag`, `the_agent_skill_names_only_fields_of_the_document`, `the_skill_has_one_row_for_each_exit_code`, `the_skill_tells_the_user_the_cause_and_the_fix`, and `the_skill_names_no_other_search_tool_after_a_failure` | The snippet is the body, byte for byte, and the body teaches the four behaviors and the failure rule of DECISION-55. No second content test is necessary |
| CRITERION-126 | `tests/agent_skill.rs`, `the_readme_snippet_is_the_skill_body` (rows 10 to 12). With no `JAG_SNIPPET_BLESS`, the README region is equal to `snippet_region(body)`, and the body obeys row 12 | The snippet and the skill come from the one file `skills/jag/SKILL.md` |
| CRITERION-127 | `tests/agent_skill.rs`, `a_change_of_the_body_changes_the_snippet`. In memory only, it adds the line `A changed line of the source.` to the end of the body. Then: (1) `snippet_region` of the changed body is not equal to the region in `README.md`, so the check of row 11 fails for it; (2) `replace_region` with the changed body gives a README whose region holds the new line, and the check passes for the changed body; (3) the bytes before and after the region are the same as in `README.md`. The test writes no file | A change of the source changes the snippet through the make step, and the check finds a snippet that did not change. The skill is the source file, so the change is in the skill |

The tests use two helpers in `tests/agent_skill.rs`:

- `snippet_region(body: &str) -> String`: the region of row 9.
- `replace_region(readme: &str, region: &str) -> Result<String, String>`:
  the README with the region from the start marker line to the end marker
  line replaced. It gives an error when the README does not hold exactly one
  start marker and one end marker after it.

The make step and the check call these two helpers (row 9).

### Rollout

The snippet ships with the README (row 15). It is in the next release on
the crates.io page and on the npm page. On GitHub, it is live when the pull
request merges. The check is not required on `main` (row 19). The job
`build` of `release.yml` runs it, so a release stops when the snippet does
not agree with the body.

### Security

Row 17. The make step writes only the region of `README.md`.

### Third-party dependencies

Row 16. No new crate and no new tool.

## Build order

1. STORY-73: in `tests/agent_skill.rs`, add `SNIPPET_START`, `SNIPPET_END`,
   `snippet_region`, and `replace_region`. Add the tests
   `the_readme_snippet_is_the_skill_body` and
   `a_change_of_the_body_changes_the_snippet`. In `README.md`, add the
   section of row 4 with the text of row 5 and the two markers. Run
   `JAG_SNIPPET_BLESS=1 cargo test --locked --test agent_skill` one time to
   fill the region. Read the diff of `README.md`: only the new section
   changes.
2. STORY-72: add the sentence of row 6 to part 2 of `## Install`. Add the
   test `the_readme_shows_the_snippet_after_install`. Open the README of the
   pull request on GitHub. Check that the snippet shows as one block with a
   copy button, and that the two code blocks of the body are in it.
3. Run `cargo fmt --check`, `cargo clippy --locked -- -D warnings`, and
   `cargo test --locked`, with no `JAG_SNIPPET_BLESS`. Then change one word
   of the body in the working tree, run
   `cargo test --locked --test agent_skill`, and check that
   `the_readme_snippet_is_the_skill_body` fails with the fix command of row
   11. Undo the change.

## Assumptions

- Row 3 (the body text) comes from `split` and `body` in
  `tests/agent_skill.rs`, and from the bytes of `skills/jag/SKILL.md`.
- Row 4 (a new section after `## Install`) comes from ADR-0024, from
  `section` in `tests/agent_skill.rs`, and from the FEATURE-27 spec.
- Row 5 (the text around the snippet) comes from ADR-0025 and from the
  README line of FEATURE-24 row 27.
- Row 6 (the pointer in `## Install`) comes from the FEATURE-25 page, from
  ADR-0025, and from `the_readme_gives_one_install_command`.
- Row 7 (four backticks and `markdown`) comes from CommonMark 0.31.2,
  section 4.5, from the fences of `skills/jag/SKILL.md`, and from the
  `yaml` blocks of `README.md`.
- Rows 8 and 9 (the markers and one region function) come from
  `scripts/adr/adr_index.py` and the roadmap pages.
- Row 10 (the make step as a cargo test with `JAG_SNIPPET_BLESS`) comes from
  `tests/rg_parity.rs` and ADR-0021.
- Row 11 (the CI jobs) comes from FEATURE-24 row 22 and from `ci.yml` and
  `release.yml`.
- Row 12 (the rules of the body) comes from CommonMark 0.31.2 and rows 7
  and 8.
- Row 14 (the fence count) comes from `the_readme_gives_one_install_command`.
- Row 15 (the release) comes from `Cargo.toml`,
  `scripts/release/release.py`, ADR-0020, and ADR-0024. ADR-0024 says that no
  crate and no package holds the skill. That stays true for the file
  `skills/jag/SKILL.md`. The README holds the body text, as ADR-0025 says.
- Row 17 (security) comes from FEATURE-24 row 16.
- Row 18 (absolute links) is already true: the body holds only `https://`
  links, and a FEATURE-24 test checks it.

## Risks

| Risk | Signal |
|---|---|
| The job `rust` is not a required check (row 19). A pull request can change the body with no make step and merge with a red `rust` job | The job `rust` fails on `main` after a merge that changed `skills/jag/SKILL.md`, or a release stops at the job `build` |
| The change of FEATURE-24 row 16 to the body, or the `jag-cli` change to `README.md`, merges after the FEATURE-25 build and does not run the make step | A merge conflict in `README.md`, or `the_readme_snippet_is_the_skill_body` fails on `main` |
| The README on crates.io and npm is the text of the last release. The skill on GitHub is the text of `main`. Between two releases they can be different | A user report that the snippet on the npm page or the crates.io page is not the same as the skill |
| A tool that reads the README headings does not know fences, and it finds the `##` lines of the body as README sections | A test or a tool that finds two sections with one name, or a section `What jag is` in the README |
| crates.io or npm shows an HTML comment as text | The marker text shows on the crates.io page or on the npm page after the next release |
| A shell keeps `JAG_SNIPPET_BLESS` set. Then the test writes and never fails on that computer | A local run passes and the job `rust` fails |
| An agent builder pastes the snippet into a file that the agent reads at each turn. The body is about 5 KB of each context | A user report about the size of the snippet |

## Out of scope

- The teaching itself: FEATURE-24 (done). A change of the teaching is a
  change of the body (row 13).
- The test of what an agent does with the instructions: FEATURE-26
  (STORY-74, STORY-75).
- The README examples for people: FEATURE-27 (STORY-76).
- The crate name `jag-cli` in `README.md`: ADR-0026 and CRITERION-79.
- An option of jag that prints the instructions. No ADR records one, and
  ADR-0024 rejected an option that copies the skill.
- A second form of the instructions for another agent. ADR-0025 says that
  it is also made from the body.
- `bodyOf` in `tests/agent_skill/install.test.js` (row 3).

## For the architect

- ADR-0025 gives the make step and the check to this spec. This spec writes
  no ADR, because the form (two markers and a cargo test with
  `JAG_SNIPPET_BLESS`) is easy to change: one test and one README section.
  If the architect sees the form as hard to undo, it needs an ADR.
