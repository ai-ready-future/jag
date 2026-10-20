---
title: "Install the jag skill with one README command: the skill file, the README command, and the tests"
feature: FEATURE-24
stories: [STORY-68, STORY-69, STORY-70, STORY-71]
kind: build
gh_issue: 181
lifecycle_state: draft
inherits: [IF-3, IF-4, IF-5, CV-1, CV-5, CV-6, CV-8]
session: 4acb9f7a-4603-41fb-8e74-395e5117e288
---

# Install the jag skill with one README command: the skill file, the README command, and the tests

## Problem

An agent builder (PERSONA-01) wants a coding agent to use jag well with no
instructions that the builder writes (FEATURE-24, CAPABILITY-11,
CONSTRAINT-08). CONSTRAINT-08: "The README of jag gives one command that
installs the jag skill into an agent. The agent builder runs that command
after the program is installed. The agent builder writes no instructions
and edits no file."

CAPABILITY-11 gives the four behaviors that the instructions teach:

1. The agent uses jag in place of rg, with a term and a plain-language
   question in `--ask`.
2. The agent gives jag a broad term and a clear plain-language question.
3. The agent sets a limit on the text that jag returns.
4. The agent reads the structured output and opens the file and the line of
   each result. It acts correctly when jag reports no answer, text that it
   left out, or a service failure.

If jag is not installed, the key is not set, or the model service is down,
the agent gives the user the cause and the fix that jag writes. The skill
says nothing about other tools, so the agent then does what it does by
default (CAPABILITY-11).

The user answered six questions in the explore session of 2026-10-19
(orchestrator TASK for FEATURE-24). This spec cites them as "user answer N":

1. The skill and the snippet teach the same thing, from one source.
2. The README shows how to install the skill with the `npx skills` command.
   The tool choice is a build choice, not in the PRD.
3. The skill serves each agent that the install command supports, not only
   Claude Code.
4. With no builder instructions, the agent must: choose jag over grep or rg
   when keyword search gives too much text; write a broad term and a clear
   question; set the size limit; read structured output and failures.
5. If jag is not installed, the key is not set, or the service is down, the
   agent tells the user what is wrong and how to fix it. Then it falls back
   to its own search tool (grep, rg, and others).
6. An agent skill is not an editor plugin. SCOPE-08 does not change
   (DECISION-53).

Two later user answers of 2026-10-20 replace parts of answers 4 and 5:

- DECISION-54: the skill tells the agent to use jag in place of rg, and it
  shows how jag and `--ask` work. It gives no advice on when to use jag and
  when to use another tool. This replaces "choose jag over grep or rg when
  keyword search gives too much text" of answer 4.
- DECISION-55: when jag cannot run, the skill tells the agent to give the
  user the cause and the fix that jag writes. It says nothing about other
  tools. This replaces "Then it falls back to its own search tool" of
  answer 5.

The stories and their criteria:

- STORY-68: install the skill with one command.
  - CRITERION-115: the README shows one command that installs the jag skill.
  - CRITERION-116: after the command runs, the jag skill is present in each
    agent that the install command supports, such as Claude Code.
- STORY-69: the agent uses jag in place of rg and sets a limit.
  - CRITERION-117: for a search, the agent runs jag in place of rg.
  - CRITERION-118: the agent gives jag a broad term and a clear question.
  - CRITERION-119: the agent sets a limit on the text that jag returns.
- STORY-70: the agent reads the structured output.
  - CRITERION-120: the agent uses the file and line of a result to read or
    cite the source.
  - CRITERION-121: the agent reads a failure in the output of jag and does
    not treat it as a result.
- STORY-71: the agent tells the user the cause and the fix.
  - CRITERION-122: when jag is not installed, the key is not set, or the
    model service is down, the agent tells the user what is wrong and how
    to fix it.
  - CRITERION-123: the skill names no other search tool for the case where
    jag fails. After the agent tells the user, it does what it does by
    default.

Criteria 117 to 122 are about what an agent does. This spec proves that the
skill text holds each instruction. FEATURE-26 builds the repeatable agent
test that records what an agent does (see Out of scope). CRITERION-123 is
about the text of the skill, so the test of this spec proves it.

## Decisions

No row needs a rule card. Each row has a source. Rows marked "inferred" are
in the Assumptions list, with the source that a reviewer can open.

| # | Decision | Choice | Source |
|---|---|---|---|
| 1 | Specs | One spec for STORY-68 to STORY-71. The four stories share one file, `skills/jag/SKILL.md`: STORY-69 to STORY-71 are its text, and STORY-68 installs it | Orchestrator TASK for FEATURE-24 ("1 feature = 1 spec = 1 PR") |
| 2 | The install tool | The `skills` command line tool (npm package `skills`, source `vercel-labs/skills`). The README command is `npx skills add ai-ready-future/jag`. The tests pin `skills@1.7.4` | User answer 2; ADR-0019 (the repository name) |
| 3 | The place and the name of the skill | One folder `skills/jag/` with one file `SKILL.md`. The skill name is `jag`, the name of the command. The `skills` tool finds a skill in the folder `skills/` of a repository | Inferred: the discovery order of the `skills` tool (github.com/vercel-labs/skills v1.7.4, docs/creating-skills.md, "Skill Discovery"); README line 4 (the command is `jag`) |
| 4 | The frontmatter | Only `name: jag` and `description`. No key of one agent, for example no `allowed-tools`. The description is one line in double quotes, so that a simple reader of each agent can read it. It is 1,024 characters or less. It says what jag is, as row 7 does, and gives no advice on when to use which tool | Inferred: user answer 3; the Agent Skills format (agentskills.io/specification, fields `name` and `description`). The words: DECISION-54 |
| 5 | One source | `skills/jag/SKILL.md` is the one source of the agent instructions. The body after the frontmatter is complete alone: one file, no folder `references/` or `scripts/`, no relative link, no word that only one agent knows. FEATURE-25 makes the README snippet from this body | Inferred: user answer 1; FEATURE-25 (CRITERION-126, CRITERION-127) |
| 6 | Words for one agent | The skill tells the agent to "run the command in a shell". It names no tool of one agent | Inferred: user answer 3 |
| 7 | What jag is (CRITERION-117) | The skill tells the agent to use jag in place of rg. jag is an enhanced rg: the same command form as rg, a term and a path, plus `--ask`, which ranks the matching lines by how well they answer a plain-language question. The skill gives no condition and no advice on when to use jag and when to use another tool | DECISION-54; ADR-0022 (with no `--ask`, rg is the reference for jag) |
| 8 | The command form (CRITERION-118, CRITERION-119) | `jag --json --max-chars N --ask "QUESTION" TERM PATH`. Always `--json`. Always `--max-chars`. Always a path, for example `.`. A question that starts with `-` uses `--ask="..."`. The term is one broad word or a short regex. The question is one sentence in plain language | ADR-0018 (the question only through `--ask`); README "Where jag searches" (an agent gives a path, because jag waits on an open pipe); README line 222 (`--ask=`); CAPABILITY-11 behavior 2 |
| 9 | The size limit (CRITERION-119) | The agent always sets `--max-chars`. It chooses N from the text that it can read now. The limit counts characters, not tokens. The smallest limit is 1,000. The default is 20,000 | README "The size limit" (FEATURE-09, built) |
| 10 | Options that the skill does not name | `-q`, `-l`, `-c`, `-n`, `-N`, `-H`, `-I`, `--heading`, and `--no-heading` do not work with `--ask` or `--json`: jag gives exit 2 and a fix, and row 12 covers code 2. `--no-max-chars` turns the limit off. The skill names none of them. It can name `-i`, `-w`, `-F`, and `-g GLOB`, which work with `--ask` | `src/cli/grep.rs` `refuse_plain_options` (FEATURE-13 decisions 19 to 22, FEATURE-23 decision 23, built); row 9 |
| 11 | The fields that the agent reads (CRITERION-120) | `exit_code`, `error.cause`, `error.fix`, `no_good_answer`, `results_found`, `left_out`, `size_limit_fix`, and in each result `path`, `first_line`, `last_line`, and in each line `number`, `text`, `is_match`. The agent opens `path` at `first_line` to `last_line`, or cites `path:number` | FEATURE-11 spec (built); ADR-0008 (fields are only added) |
| 12 | What each exit code tells the agent (CRITERION-121) | One table in the skill with each code of IF-4: 0, 1, 2, 3, 4, 5, 6, 7, 8, 126, 127, 130. Code 1 is not a failure: with `no_good_answer: true`, no match answers the question; with `results_found: 0`, the term matched nothing, so the agent tries a broader term. Codes 7 and 8 still carry valid results: the agent uses them, then raises the limit or narrows the term. Code 2: the agent fixes the command from `error.fix` | ADR-0007; IF-4 |
| 13 | The codes where jag cannot run (CRITERION-122, CRITERION-123) | Codes 3, 4, 5, 6, 126, 127, and "command not found" from the shell. The agent gives the user the cause and the fix one time in the task. It does not run jag again in that task, unless the user says that the fix is done. The skill names no other search tool and gives no rule for what the agent does next | DECISION-55; CAPABILITY-11 ("the agent gives the user the cause and the fix that jag writes"); row 18 and CV-5 (no run again) |
| 14 | The failure text | The skill does not copy the failure texts of jag. It tells the agent to give `error.cause` and `error.fix` to the user as jag writes them. The skill holds its own text for one case only: jag is not installed, so no JSON document exists | Inferred: CV-1 (each failure gives its cause and its fix) |
| 15 | jag is not installed | The agent does not install jag. It tells the user that jag is not installed, gives the README Install section (`https://github.com/ai-ready-future/jag#install`), and names `npm install -g @airf/jag` | User answer 5; ADR-0010 (the install channels) |
| 16 | The key | The agent never asks for the key in the chat, never puts it in a command, and never reads or prints the files in `~/.config/jag/`. For code 3 or 4, it tells the user to run `jag --setup` in their own terminal, or to set `JAG_TYPESAFE_API_KEY` | CV-6; ADR-0006; ADR-0013 (setup with no terminal gives exit 2) |
| 17 | No check before a search | The skill does not run `jag --check`. `--check` makes a paid call and has no JSON form. The agent runs the search and reads the exit code | Inferred: README "Check that jag is ready"; ADR-0006 |
| 18 | No retry loop | For code 6, the agent does not run jag again at once: jag already tried again in the same run | CV-5 |
| 19 | No pipe that cuts the output | The agent does not pipe the JSON document into `head` or a similar tool. It uses `--max-chars` | Inferred: IF-5 (one JSON document); row 9 |
| 20 | The README section (CRITERION-115) | A new section `## Add the jag skill to your agent`, after `## Install`. It holds one fenced block with one line: `$ npx skills add ai-ready-future/jag`. The text after the block says that the tool asks which agents get the skill and where it goes; that `--agent claude-code -y` installs with no question; that the tool writes `skills-lock.json` in the project; and that the `skills` tool, not jag, sends anonymous usage data and a security-audit request to skills.sh, which `DO_NOT_TRACK=1` stops | CONSTRAINT-08; user answer 2; CV-8 (jag sends no telemetry); github.com/vercel-labs/skills, README, "Telemetry" |
| 21 | README line 6 | Remove "This release has the key setup only. The search comes later." The skill teaches the search, so the line is wrong | Inferred: README line 6; FEATURE-14 and FEATURE-23 specs (built) |
| 22 | The content tests (CRITERION-115, 117 to 123) | A Rust test file `tests/agent_skill.rs`, one unit test in `src/cli.rs`, and one unit test in `src/output.rs`. They run in the job `rust` of `ci.yml` | Inferred: `src/cli.rs` test `the_hyphen_value_lists_agree_with_cli`; `src/output.rs` test `no_field_of_the_form_is_removed_or_renamed` |
| 23 | The install test (CRITERION-116) | A Node test `tests/agent_skill/install.test.js`. It runs `skills@1.7.4` on the local checkout, not on GitHub, so that it tests the pull request. A new job `agent-skill` in `ci.yml` runs it with Node 24 and `DO_NOT_TRACK=1` | Inferred: the job `release-tools` of `ci.yml` (`node --test npm/jag/test/launcher.test.js`) |
| 24 | "Each agent that the install command supports" | Each agent that `skills@1.7.4` installs to with `--agent '*'`. A new pin of the tool changes this list | Inferred: CRITERION-116; row 2 |
| 25 | The release | The skill is not in the crate and not in the npm package. The `skills` tool reads it from the repository. `Cargo.toml` `include` and `npm/` do not change | User answer 2; `Cargo.toml` `include` |
| 26 | A change of jag | A pull request that adds or changes an option, a field, or an exit code that the skill names changes `SKILL.md` in the same pull request. The tests of row 22 fail when the skill names an option or a field that jag does not have | Inferred: ADR-0007; ADR-0008; row 22 |

## Design

### Data model and storage: the skill file

New file `skills/jag/SKILL.md`. No other file goes in `skills/`.

The frontmatter:

```yaml
---
name: jag
description: "jag is a code and text search tool to use in place of rg. It takes a term and a path as rg does, and with --ask it ranks the matching lines by how well they answer a plain-language question. Use it for each search of code or text files."
---
```

The build can change the words of the description. The description says
what jag is and that the agent uses it in place of rg (row 7). It says
that jag ranks the matching lines by how well they answer a
plain-language question. The agent loads the skill for a search of code
or text files. The description gives no advice on when to use jag and
when to use another tool. It is one line in double quotes, with 1,024
characters or less.

The body has these sections, in this order. Each section is short. The body
names only the options, the fields, and the exit codes of rows 8 to 19.

1. **What jag is** (CRITERION-117, row 7). jag is an enhanced rg. It has
   the same command form as rg, a term and a path. `--ask` ranks the
   matching lines by how well they answer a plain-language question. Use
   jag in place of rg. The section gives no condition and no advice on
   when to use jag and when to use another tool.
2. **How to run jag** (CRITERION-118, CRITERION-119, rows 8 to 10, 19). Run
   the command in a shell. Give the form of row 8 with one example, for
   example `jag --json --max-chars 8000 --ask "Where does the code retry a
   failed request?" retry .`. Say that the term is broad and the question is
   one clear sentence. Say that `--max-chars` is always present, how to
   choose N, the smallest value, and the default. Say why a path is always
   present. Do not name the options of row 10 that the skill does not use.
3. **How to read the output** (CRITERION-120, row 11). Read `exit_code`
   first. Then read the results in rank order. Open `path` from `first_line`
   to `last_line` before you change code, and cite `path:number` in an
   answer. `is_match` marks the lines that hold the term. When `left_out` is
   more than 0, `size_limit_fix` tells how to get the rest.
4. **What each exit code means** (CRITERION-121, row 12). One Markdown table
   with the columns "Code", "Meaning", and "What to do", and one row for each
   code of row 12. The rows of codes 3, 4, 5, 6, 126, and 127 say "jag
   cannot run: see the next section".
5. **When jag cannot run** (CRITERION-122, CRITERION-123, rows 13 to 18).
   Give the cause and the fix to the user: `error.cause` and `error.fix` as
   jag writes them; for 126, 127, or "command not found", the text of row
   15. Say this one time in the task. Do not run jag again in the task,
   unless the user says that the fix is done. The section names no other
   search tool and gives no rule for what to do next. The rules of the key
   (row 16) and the rule of code 6 (row 18) are in this section.

### Interfaces: the README command

`README.md` changes in two places:

- Line 6 goes (row 21).
- A new section `## Add the jag skill to your agent` goes after `## Install`
  (row 20). It has one fenced block:

  ```
  $ npx skills add ai-ready-future/jag
  ```

  The block uses the `$ ` prefix of the other README commands. The other
  forms of the command (`--agent claude-code -y`, `DO_NOT_TRACK=1`) are in
  the text, in inline code, not in a fenced block.

The CLI of jag, the JSON document, and the exit codes do not change.

### Migration and compatibility

Nothing to migrate. A user who installed the skill gets a new version only
when they run the command again. `npx skills update` is the tool's command
for this. The README does not name it, because CONSTRAINT-08 asks for one
command.

Row 26 keeps the skill and jag the same. The skill comes from the default
branch. The binary comes from a release. See Risks.

### Error handling

The skill teaches the error handling of rows 12 to 18. jag itself does not
change.

### Test strategy

| Criterion | Test | What it proves |
|---|---|---|
| CRITERION-115 | `tests/agent_skill.rs`, `the_readme_gives_one_install_command`. It reads `README.md`, finds each fenced block, and finds each line that starts with `$ npx skills add`. Exactly one such line exists, and it is `$ npx skills add ai-ready-future/jag`. The repository name is the name of ADR-0019 | The README shows one command |
| CRITERION-116 | `tests/agent_skill/install.test.js`, run with `node --test`. It makes an empty project folder and an empty home folder in a temporary folder. In the project folder, with `HOME` set to the home folder and `DO_NOT_TRACK=1`, it runs `npx --yes skills@1.7.4 add <checkout> --all --json`. `<checkout>` is the path of the repository. Then it runs `npx --yes skills@1.7.4 list --json` in the same folder. It checks: the add command exits with 0, and its output has no entry with `"status": "failed"`; the list has one skill only, `jag`; the `agents` of that entry are each agent that the tool installs to with `--agent '*'` (row 24); and for each agent, the `SKILL.md` file at its path has the same bytes as `skills/jag/SKILL.md`. It also checks the path `.claude/skills/jag/SKILL.md` by name, for Claude Code | The command installs the skill into each agent that the tool supports |
| CRITERION-117 | `tests/agent_skill.rs`, `the_skill_says_to_use_jag_in_place_of_rg`. The section "What jag is" exists and holds the words "in place of rg" and `--ask`. With no regard to letter case, the skill holds none of the phrases "plain search", "too many lines", "too much text", "more lines than", and "when to use" | The skill holds the rule of row 7 and no advice on when to use which tool. FEATURE-26 proves what an agent does |
| CRITERION-118 | `tests/agent_skill.rs`, `the_skill_gives_the_command_form`. The skill holds `--ask`, `--json`, a path in the example, and the `--ask=` form | The skill holds the form of row 8 |
| CRITERION-119 | Same test: the skill holds `--max-chars`, `1,000`, and `20,000`. And `src/cli.rs`, `the_agent_skill_names_only_options_of_jag`: in each code span of `SKILL.md` whose first word is `jag`, each word that starts with `-` is a short or a long option of `Cli::command()`, and no option of row 10 is in the skill. A code span of grep or rg is not checked | The skill holds the limit, and it names no option that jag does not have |
| CRITERION-120 | `src/output.rs`, `the_agent_skill_names_only_fields_of_the_document`. It holds the field list of row 11. It checks that each field is in `SKILL.md` and is a field of the document (the lists `TOP_LEVEL`, `ERROR`, `RESULT`, and `LINE` of the test module) | The skill names the fields of the file and the line, and each one exists |
| CRITERION-121 | `tests/agent_skill.rs`, `the_skill_has_one_row_for_each_exit_code`. It reads the rows of the exit-code table. The set of codes is exactly 0, 1, 2, 3, 4, 5, 6, 7, 8, 126, 127, 130 | The skill tells the meaning of each code |
| CRITERION-122 | `tests/agent_skill.rs`, `the_skill_tells_the_user_the_cause_and_the_fix`. The section "When jag cannot run" names `error.cause`, `error.fix`, `jag --setup`, `JAG_TYPESAFE_API_KEY`, `command not found`, and the README Install link. It holds no text of `src/` that starts with "the TypeSafe API" (row 14) | The skill holds the rules of rows 13 to 16 |
| CRITERION-123 | Same file, `the_skill_names_no_other_search_tool_after_a_failure`. The section "When jag cannot run" and the table rows of codes 3, 4, 5, 6, 126, and 127 hold none of the words `grep`, `rg`, `ripgrep`, `ag`, and `ack`, and none of the phrases "fall back" and "fallback", as whole words with no regard to letter case | The skill gives no fallback rule |

One more test in `tests/agent_skill.rs`, `the_skill_frontmatter_is_for_each_agent`,
checks rows 3 to 5: the folder is `skills/jag`; the frontmatter has only
`name` and `description`, one line each; `name` is `jag`; the description
has 1 to 1,024 characters; the body has no relative link (a Markdown link whose target does
not start with `https://`).

The Rust tests read the files at run time from
`env!("CARGO_MANIFEST_DIR")`. They do not use `include_str!`, because the
crate package does not hold `skills/`.

The verifier also runs one manual check for CRITERION-116 before the merge:
`npx skills add <checkout> --agent claude-code -y` in an empty folder, then
a look at `.claude/skills/jag/SKILL.md`.

### Rollout

The skill is live when the pull request merges, because the `skills` tool
reads the default branch. No release step. No flag.

`ci.yml` gets one new job:

```yaml
  # The install test of the agent skill (FEATURE-24). Not a required check.
  agent-skill:
    name: agent-skill
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      - uses: actions/setup-node@949feb2413d6458794dcd2491c4babbbce0c15c1 # v7.1.0
        with:
          node-version: 24
      - run: node --test tests/agent_skill/install.test.js
        env:
          DO_NOT_TRACK: "1"
```

The orchestrator decides if `agent-skill` becomes a required check.

### Security

The skill teaches the rules of row 16 and CV-6. The skill holds no key, no
address, and no example key. The install test sets `HOME` to a temporary
folder, so the tool writes nothing in the real home folder.

### Third-party dependencies

`skills@1.7.4` runs only in the test, through `npx`. jag gets no new crate
and no new npm dependency. The user runs the tool through `npx`. The tool
sends anonymous usage data and a security-audit request to skills.sh for a public GitHub repository (row 20).

## Build order

1. STORY-68, part 1: write `skills/jag/SKILL.md` with the frontmatter and the
   five section headings. Write `tests/agent_skill.rs` with
   `the_skill_frontmatter_is_for_each_agent` and
   `the_readme_gives_one_install_command`. Change `README.md`: remove line 6
   (row 21) and add the section of row 20.
2. STORY-68, part 2: write `tests/agent_skill/install.test.js` and the job
   `agent-skill`. Confirm the JSON form of `skills@1.7.4 add --json` and
   `list --json` on the real tool, and confirm that `list --json` names each
   agent of `--agent '*'`. Confirm also that `--agent claude-code -y` asks no
   question in an empty folder, as the README says (row 20). If it asks one,
   change the README text to the form that asks none. If it does not, the test reads the agent list from
   the add output, or from a file `tests/agent_skill/agents-1.7.4.txt` that
   the build copies from the agent list of the tool source at tag `v1.7.4`.
   The file name holds the version, so a new pin needs a new file.
3. STORY-69: write sections 1 and 2 of the skill. Add
   `the_skill_says_to_use_jag_in_place_of_rg`,
   `the_skill_gives_the_command_form`, and
   `the_agent_skill_names_only_options_of_jag`.
4. STORY-70: write sections 3 and 4. Add
   `the_agent_skill_names_only_fields_of_the_document` and
   `the_skill_has_one_row_for_each_exit_code`.
5. STORY-71: write section 5. Add
   `the_skill_tells_the_user_the_cause_and_the_fix` and
   `the_skill_names_no_other_search_tool_after_a_failure`.
6. Run `cargo fmt --check`, `cargo clippy --locked -- -D warnings`,
   `cargo test --locked`, and `node --test tests/agent_skill/install.test.js`.

## Assumptions

- Row 3 (`skills/jag/SKILL.md`) comes from the discovery order of the
  `skills` tool: github.com/vercel-labs/skills v1.7.4, docs/creating-skills.md, "Skill Discovery".
  A `SKILL.md` at the root of the repository wins over `skills/`, so the
  root gets none.
- Row 4 (only `name` and `description`) comes from user answer 3 and the
  Agent Skills format (agentskills.io/specification).
- Row 5 (one self-contained body) comes from user answer 1 and FEATURE-25
  (CRITERION-126 and CRITERION-127).
- Row 6 (no word of one agent) comes from user answer 3.
- Row 14 (the skill does not copy the failure texts) comes from CV-1.
- Row 17 (no `jag --check` before a search) comes from the README section
  "Check that jag is ready" and ADR-0006.
- Row 19 (no pipe into `head`) comes from IF-5 and row 9.
- Row 21 (README line 6) comes from the README and the built FEATURE-14 and
  FEATURE-23 specs.
- Row 22 (the content tests) comes from `src/cli.rs`
  (`the_hyphen_value_lists_agree_with_cli`) and `src/output.rs`
  (`no_field_of_the_form_is_removed_or_renamed`).
- Row 23 (the Node test and the CI job) comes from the job `release-tools`
  of `.github/workflows/ci.yml`.
- Row 24 (the meaning of "each agent that the install command supports")
  comes from CRITERION-116 and row 2.
- Row 26 (one pull request changes jag and the skill) comes from ADR-0007,
  ADR-0008, and row 22.

## Risks

| Risk | Signal |
|---|---|
| The skill comes from the default branch, and the binary comes from a release. The skill can name an option that the installed jag does not have yet | An agent gets exit 2 with "unexpected argument" for an option that the skill names |
| jag has no release yet (FEATURE-18 is not built). The install commands of row 15 fail until the first release | `npm install -g @airf/jag` fails with "not found" |
| A new version of `skills` changes the discovery, the agent list, or the JSON output | The job `agent-skill` fails after a pin change, or a user report that `npx skills add ai-ready-future/jag` finds no skill |
| The `skills` tool sends anonymous usage data and a security-audit request to skills.sh. A user expects no telemetry, because jag sends none (CV-8) | A user report or an issue about telemetry |
| The agent reads 20,000 as tokens, or chooses a limit that is too large for its context | A FEATURE-26 run that records a limit above the free context of the agent |
| The agent treats exit 1 as a failure and tells the user that jag cannot run, when it should try a broader term | A FEATURE-26 run that records a message to the user after exit 1 |
| The agent uses jag in place of rg for each search, and row 8 always has `--ask`. So each search makes a paid call to the model, also for an exact text | The cost of an agent task with the skill is much higher than the cost of the same task with rg |
| The system design says that CAPABILITY-11 "is a text file that ships with the release". Row 25 puts the skill in the repository only | The architect changes the system design (see the pull request text) |
| The job `agent-skill` downloads `skills` from npm. An npm outage makes it fail | The job fails at `npx --yes skills@1.7.4` with a network error |
| The change of `ci.yml` needs the `workflow` scope of the token | The push of the build pull request fails |

## Out of scope

- The README snippet of the same instructions: FEATURE-25 (STORY-72,
  STORY-73). This spec keeps one source for it (row 5).
- The repeatable agent test that shows that an agent with the skill uses
  jag in place of rg, sets the limit, reads the results, and gives the user
  the cause and the fix when jag fails: FEATURE-26 (STORY-74, STORY-75).
  This spec proves CRITERION-117 to CRITERION-122 by the text of the skill
  only.
- An editor plugin: SCOPE-08 does not change (DECISION-53).
- A change of the CLI, the JSON document, or the exit codes of jag.
- The release of jag: FEATURE-18.
- A command that installs jag itself from the skill.
