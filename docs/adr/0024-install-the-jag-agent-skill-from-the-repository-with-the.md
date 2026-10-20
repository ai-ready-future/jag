---
type: adr
title: Install the jag agent skill from the repository with the skills tool
description: Open this when you change how users get the jag program or the jag skill, where the skill file lives, or the pinned version of the skills tool.
area: deployment
status: proposed
date: 2026-10-20
supersedes: null
superseded_by: null
amends: null
refines: null
deviates_from: null
related:
  - 0010-ship-jag-on-github-releases-crates-io-and-npm.md
  - 0019-give-the-new-public-repository-the-name-ai-ready-future-jag.md
parent_spec: docs/product/roadmap/features/FEATURE-24.md
model:
  status: out-of-scope
  reason: the agent skill is a text file in the repository, not an element of the jag program
---

# ADR-0024: Install the jag agent skill from the repository with the skills tool

## Context

CAPABILITY-11 and CONSTRAINT-08: the README of jag gives one command that installs the jag skill into an agent. The agent builder runs that command, writes no instructions, and edits no file. The user gave these answers:

- Explore session of 2026-10-19 (FEATURE-24 spec, user answers 2 and 3): the README shows how to install the skill with the `npx skills` command, and the skill serves each agent that the install command supports, not only Claude Code.
- Hitl `ANSWER` of 2026-10-20, card "How users get the jag skill, and what the skill holds", question 1, option 1: the command names the repository `ai-ready-future/jag`. The npm name stays `@airf/jag`.
- Hitl `ANSWER` of 2026-10-20, card "The one install step, and what the skill tells the agent", question 1, option 3: the README first installs the jag program. The skill command is a second step.
- Hitl `ANSWER` of 2026-10-20 on the acceptance card of this ADR, "change it": "decision should be that we are going to use npx skills as the distrivution mechanism for the skill and npm for the distro of the cli".
- Hitl `ANSWER` of 2026-10-20, card "Does the jag program also stay on crates.io and the release downloads?", option 1, with the words "others also in the readme": npm is the main install step of the program. crates.io and the release downloads stay, and the README also shows them.

Facts that the decision uses:

- The `skills` tool (npm package `skills`, source `vercel-labs/skills`) installs a skill from a code host (GitHub, GitLab, Azure Repos, or any git address), a local folder, a web address, or a download address. `skills add` cannot install a skill from an npm package. The experimental command `skills experimental_sync` links skills only from the npm packages that the `package.json` of a project declares, and a global `npm install -g @airf/jag` is not one of them. The shorthand `owner/repo` names a GitHub repository, so `npx skills add airf/jag` would read `github.com/airf/jag`, and that repository does not exist.
- In a repository, the tool finds a `SKILL.md` at the root, or in the folder `skills/`, in the folders of the agents (for example `.claude/skills/`), and then in other folders. A `SKILL.md` at the root stops the search.
- The tool puts the skill into the skill folder of each agent that the user selects, for example Claude Code (`.claude/skills/`), Codex, or Pi (`.agents/skills/`).
- The npm package `@airf/jag` holds the prebuilt jag program in one package for each platform, so it does not need crates.io or the release downloads. Each release also ships the program on crates.io and as GitHub release downloads, from the same tag (0010-ship-jag-on-github-releases-crates-io-and-npm.md). jag has no release yet.
- When users and guides start to use the README commands and the place of the file, a change of these breaks their installs. So the route is expensive to change.

## Decision

- jag has two distribution channels for the two parts that a user installs. npm (`@airf/jag`) is the main channel of the jag program. The `skills` tool (`npx skills add ai-ready-future/jag`) is the channel of the jag skill.
- The README first gives the npm install of the program, `npm install -g @airf/jag`, as the main step. Then it gives the skill command, `npx skills add ai-ready-future/jag`, which the user runs for the agent that they use.
- crates.io (`cargo install --locked jag-grep`) and the GitHub release downloads stay. 0010-ship-jag-on-github-releases-crates-io-and-npm.md does not change. The README shows them after the npm step, in a part "Other ways to install", with the full command or steps for each.
- jag has no install code for skills, and the skill does not install the jag program.
- The skill is one file in the repository, `skills/jag/SKILL.md`. The `skills` tool reads it from the repository on GitHub, not from a release asset. No release archive, no crate, and no npm package holds the skill.
- The skill name is `jag`. The frontmatter has only `name` and `description`, the fields of the Agent Skills format, so that each agent that the tool supports can read it.
- The tests use the tool at the pinned version `skills@1.7.4`. "Each agent that the install command supports" means each agent that the pinned tool installs to with `--agent '*'`.

## Why

- These are the words of the user: `npx skills` distributes the skill, and npm distributes the program.
- Both steps use the npm tools, so a user with Node needs no other tool. Many agent setups already install tools with `npm` or `npx` (0010-ship-jag-on-github-releases-crates-io-and-npm.md).
- The user asked for the other routes in the README too. Users with no Node, and Rust users, still have a route, and the release workflow does not change.
- One command serves each agent: the tool knows the skill folder of each agent, so jag needs no code for each agent.
- The program comes first, so the agent finds an installed `jag` when the user follows the README, and the skill needs no install steps of its own.
- The tool reads the repository, so the skill is live when its pull request merges. It needs no release step.
- `ai-ready-future/jag` works today. A move to `airf/jag` would replace the repository name of 0019-give-the-new-public-repository-the-name-ai-ready-future-jag.md and the trusted publishing of npm and crates.io.
- The folder `skills/` is a place that the tool searches. A `SKILL.md` at the root would stop the search, and it would put the skill text at the root of the repository.
- A pinned version gives the same test result when the tool changes.

## Alternatives considered

- npm is the only channel of the program, and crates.io and the release downloads stop: one channel to publish, but each user needs Node, and the user asked for the other routes in the README.
- npm, cargo, and the release download as three equal ways in step 1: the user named npm as the channel of the program.
- The skill as the only step, and the skill tells the agent to run jag with `npx -y @airf/jag` when it is not installed: one step, but the user chose the program first.
- `jag --setup` runs `npx skills add`: jag needs Node and code for a second tool, and setup can fail for a reason that is not jag.
- `npx skills add airf/jag`, with the repository moved to `airf`, or with a second repository that holds only the skill: the user kept `ai-ready-future/jag`.
- Put the skill in the release archives or the npm package, and add an option that copies it into the folder of each agent: jag gets new code for each agent, and the skill waits for a release.
- A plugin for one agent, for example a Claude Code plugin: it serves one agent only.

## Consequences

- The README Install text starts with `npm install -g @airf/jag`. The skill command comes after it. The cargo command and the release download steps come after these, under "Other ways to install". The FEATURE-24 spec gives the exact text.
- When the agent finds no `jag`, the skill names `npm install -g @airf/jag` and the README Install link.
- The skill comes from the default branch, and the binary comes from a release. A pull request that changes an option, a field, or an exit code that the skill names also changes `skills/jag/SKILL.md` (FEATURE-24 spec row 26). Until the next release, the skill can name an option that the released jag does not have.
- `Cargo.toml` `include`, `npm/`, and `release.yml` do not change for the skill.
- No other `SKILL.md` goes in the repository, also not in the folder of an agent such as `.claude/skills/`, because the tool also finds those files and installs them with the jag skill.
- A new name of the repository (0019-give-the-new-public-repository-the-name-ai-ready-future-jag.md), a move of the skill file, a change of the install tool, or a new main channel of the program changes the README commands, and needs a new ADR.
- The `skills` tool, not jag, sends anonymous usage data and a security-audit request to skills.sh. `DO_NOT_TRACK=1` stops both. The README says so. jag itself still sends no telemetry (CV-8).
- A new pin of the tool is a pull request that changes the tests, and it can change the list of agents.
