---
type: adr
title: Install the jag agent skill from the repository with the skills tool
description: Open this when you change how an agent gets the jag skill, where the skill file lives, or the pinned version of the skills tool.
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

CAPABILITY-11 and CONSTRAINT-08: the README of jag gives one command that installs the jag skill into an agent. The agent builder runs that command, writes no instructions, and edits no file. The user gave these answers in the explore session of 2026-10-19 (FEATURE-24 spec, user answers 2 and 3):

- The README shows how to install the skill with the `npx skills` command.
- The skill serves each agent that the install command supports, not only Claude Code.

Facts that the decision uses:

- The `skills` tool (npm package `skills`, source `vercel-labs/skills`) installs a skill from a GitHub repository that the command names as `owner/repo`. It reads the files of the repository. It finds a `SKILL.md` at the root of the repository, or in the folder `skills/`, in the folders of the agents (for example `.claude/skills/`), and then in other folders. A `SKILL.md` at the root stops the search.
- The tool puts the skill into the skill folder of each agent that the user selects.
- A release of jag (0010-ship-jag-on-github-releases-crates-io-and-npm.md) holds the binary. jag has no release yet.
- When users and guides start to use the README command and the place of the file, a change of these breaks their installs. So the route is expensive to change.

## Decision

- An agent gets the jag skill with the command `npx skills add ai-ready-future/jag`. The README shows this one command. jag has no install code for skills.
- The skill is one file in the repository, `skills/jag/SKILL.md`. The `skills` tool reads it from the repository on GitHub, not from a release asset. No release archive, no crate, and no npm package holds the skill.
- The skill name is `jag`. The frontmatter has only `name` and `description`, the fields of the Agent Skills format, so that each agent that the tool supports can read it.
- The tests use the tool at the pinned version `skills@1.7.4`. "Each agent that the install command supports" means each agent that the pinned tool installs to with `--agent '*'`.

## Why

- One command serves each agent: the tool knows the skill folder of each agent, so jag needs no code for each agent.
- The tool reads the repository, so the skill is live when its pull request merges. It needs no release step, and it does not wait for the first release.
- The folder `skills/` is a place that the tool searches. A `SKILL.md` at the root would stop the search, and it would put the skill text at the root of the repository.
- A pinned version gives the same test result when the tool changes.

## Alternatives considered

- Put the skill in the release archives or the npm package, and add an option that copies it into the folder of each agent: jag gets new code for each agent, and the skill waits for a release.
- A plugin for one agent, for example a Claude Code plugin: it serves one agent only, and user answer 3 asks for each agent that the install command supports.
- The agent builder copies the file by hand: this breaks CONSTRAINT-08. The copy route is the snippet of FEATURE-25, for an agent that cannot install a skill.

## Consequences

- The skill comes from the default branch, and the binary comes from a release. A pull request that changes an option, a field, or an exit code that the skill names also changes `skills/jag/SKILL.md` (FEATURE-24 spec row 26). Until the next release, the skill can name an option that the released jag does not have.
- `Cargo.toml` `include`, `npm/`, and `release.yml` do not change for the skill.
- No other `SKILL.md` goes in the repository, also not in the folder of an agent such as `.claude/skills/`, because the tool also finds those files and installs them with the jag skill.
- A new name of the repository (0019-give-the-new-public-repository-the-name-ai-ready-future-jag.md), a move of the skill file, or a change of the install tool changes the README command, and needs a new ADR.
- The `skills` tool, not jag, sends an install count to skills.sh. `DO_NOT_TRACK=1` stops it. The README says so. jag itself still sends no telemetry (CV-8).
- A new pin of the tool is a pull request that changes the tests, and it can change the list of agents.
