---
type: adr
title: Use the body of the jag skill file as the one source of the agent instructions
description: Open this when you change the text of the jag skill or of the README agent snippet, or how the two stay the same.
area: conventions
status: accepted
date: 2026-10-20
supersedes: null
superseded_by: null
amends: null
refines: null
deviates_from: null
related:
  - 0024-install-the-jag-agent-skill-from-the-repository-with-the.md
parent_spec: docs/product/roadmap/features/FEATURE-25.md
model:
  status: out-of-scope
  reason: the agent instructions are text in the repository, not an element of the jag program
---

# ADR-0025: Use the body of the jag skill file as the one source of the agent instructions

## Context

CAPABILITY-11: jag ships one set of agent instructions in two forms. The agent builder installs the skill (FEATURE-24), or copies a snippet from the README into the instructions of an agent that cannot install a skill (FEATURE-25). Both forms teach the same behavior. The user answered in the explore session of 2026-10-19 (FEATURE-24 spec, user answer 1): the skill and the snippet teach the same thing, from one source. FEATURE-25 asks for this in STORY-73 (CRITERION-126 "One source makes both", CRITERION-127 "A change to the source changes both").

The rule binds two features: FEATURE-24 writes the source, and FEATURE-25 makes the snippet from it. When two texts teach the same thing and each one changes by hand, they become different.

The skill is the file `skills/jag/SKILL.md` (0024-install-the-jag-agent-skill-from-the-repository-with-the.md). Its frontmatter tells an agent when to load the skill. The body after the frontmatter holds the instructions.

## Decision

- The body of `skills/jag/SKILL.md`, the text after the frontmatter, is the one source of the agent instructions of jag. No other file holds its own copy of the instructions.
- The README snippet of FEATURE-25 is made from that body, with the same teaching. The frontmatter is not part of the snippet.
- The body is complete alone, so that an agent builder can paste it into the instructions of any agent: one file, no folder `references/` or `scripts/`, no relative link, and no word that only one agent knows.
- A change of the instructions is a change of the body. The FEATURE-25 spec sets how the snippet is made from the body and the check that fails when the snippet does not agree with the body.

## Why

- One source cannot give two different texts. A rule that asks a reviewer to change two texts can fail with no signal.
- The body is the text that the `skills` tool installs and that the FEATURE-24 content tests check. So the source is the text that is already tested.
- A body that is complete alone works as a skill and as pasted text. A relative link or a second file does not work in pasted text.

## Alternatives considered

- Two texts written by hand, with a review rule: no build step, but the texts become different with no signal.
- The README snippet as the source, and the skill made from it: the `skills` tool reads the skill file from the repository, so the skill must be a real file. It would then be a made file, and each change needs a second step before the tool sees it.
- A third source file that makes both: one more file and one more step for each change, for no gain over the skill body.

## Consequences

- `skills/jag/SKILL.md` keeps the rules of the body: one file, no relative link, and no word of one agent. The FEATURE-24 tests check them.
- A pull request that changes the body also gives the snippet the change, in the same pull request. The check of FEATURE-25 fails when the two do not agree.
- The README snippet has no words of its own inside the copied text. Text around the snippet, for example how to copy it, is not part of the instructions.
- A second form of the instructions, for example for a new agent, is also made from the body.
