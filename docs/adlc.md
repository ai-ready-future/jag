# ADLC — Agent Development Life Cycle

<!-- sofa-template-version: 1 -->

This file defines the agent team for this repository.
The `sofa` skill reads this file to start the team.
Every session reads its role from this file on `main`.
The retro session is the only owner of this file.

## 1. Setup values

The `sofa` skill fills these values when it copies the template.

| Key | Value |
|---|---|
| Repository | `ai-ready-future/jag` |
| Main folder | `/home/jazz/dev/github.com/ai-ready-future/jag` |
| Board tool | GitHub Projects (PROJECT-01 in the roadmap) |
| GitHub account | `404pilo` |
| Team prefix | `jag` (session names are `jag-<role>[-n]`) |
| Orchestrator name | `jag-orchestrator` |
| Product docs | `docs/product/prd`, `docs/product/roadmap` |
| ADR folder | `docs/adr`. The list of ADRs and their status is in `docs/adr/README.md` |
| Extra context files | `/home/jazz/dev/github.com/ai-ready-future/jag-private/docs/handoff.md`. It is not in git. Since FEATURE-19 step B10, the main folder is a new clone of the public repo. `docs/handoff.md` stays only in the old folder `jag-private`; it was not copied. It points to the source of the earlier prototype. |
| Build | This machine has no `cargo`. Build and test jag in Docker with the image `rust:1.99` (the version in `rust-toolchain.toml`). The built binary runs on the host. |

Repository rules:
- Never read, print or commit the keys in `~/.config/jag/`.
- The ADRs in `docs/adr` win over the `docs/handoff.md` of the "Extra context files" row where they differ (for example, ADR-0005: jag is one Rust binary, not TypeScript).
- Read the earlier prototype only as the `docs/handoff.md` of the "Extra context files" row says (`git show` or `git archive`). Do not edit files or run commands in its worktree.

## 2. Rules for all sessions

1. Files on `main` are the source of truth. The board is only a copy of `main`.
2. Do not change `main` directly. Use a worktree, a branch and a pull request. Only the retro has a fast path (see section 6).
3. Send all messages to the orchestrator with `SendMessage`. Do not send messages to other roles directly.
4. Do not ask the user questions. Send the question to the orchestrator. The hitl session asks the user. Exceptions: hitl, retro, and chores (only to guide the user through a manual step).
5. Use the message format in section 5.
6. When the orchestrator tells you that `docs/adlc.md` changed, read it again and rebase your worktree on `main`.
7. Never read, print or commit secrets or key files.
8. Write all documents in ASD-STE100 Simplified Technical English.
9. When you finish a task, send a `DONE` message. Then wait for the next message.
10. Before you start, fetch documentation for libraries with context7 (`ctx7` CLI).
11. A story has the state `ready` only if all of these are true:
    - Its spec is merged on `main`.
    - Its feature is not blocked: each feature in its `blocked_by` list has the state `done`.
    - No open `ASK` blocks the story.

    If one of these is false, the story stays in `backlog`. A role that changes the state of a story or a feature checks this rule in the same PR.
    Exception: a user `ANSWER` can set the stories of a blocked feature to `ready`. The `blocked_by` list stays. The `Plan` line in `log.md` names the `ANSWER`. FEATURE-18 used this exception (`ANSWER` of 2026-10-15, "start now") until FEATURE-19 was `done`.
    When a feature gets the state `done`, the same PR sets to `ready` each story that waited only for this feature and has a merged spec.
12. Merge PRs with squash only. After each merge, the session that merges checks the new commit on `main`. The `sha256sum` of its author address must be the `sha256sum` of the author address of `78aa3f5`. Its committer must be the committer of `78aa3f5` (GitHub). The check never prints an address (FEATURE-19 spec decisions 13 and 31). A rebase merge wrote the account address as committer in `3ead9ae`.

## 3. Roles

| Role | Model | Effort | Count | Starts when | Skills | Input | Output |
|---|---|---|---|---|---|---|---|
| orchestrator | `claude-opus-5-5` | `medium` | 1 | The `sofa` skill runs | sofa | Messages from all roles | Routed messages, started sessions |
| hitl | `claude-sonnet-5-5` | `medium` | 1 | Always running | — | `ASK` messages | `ANSWER` messages |
| retro | `claude-opus-5-5` | `medium` | 1 | The user changes or adds a role | — | `RETRO` signal | Updated `docs/adlc.md` on `main` |
| planner | `claude-opus-5-5` | `medium` | 1 | Scope change, gap in PRD or roadmap, milestone done | prd:plan, prd:review, prd:explore | Gap or change | PR that updates PRD or roadmap |
| projecter | `claude-sonnet-5-5` | `medium` | 1 | A PR that changes `docs/product` merges | prd:roadmap-sync | Merged roadmap | Board that matches `main` |
| architect | `claude-opus-5-5` | `high` | 1 | On request, or when a reviewer reports an ADR finding | adr:* | Design question or ADR finding | Answer, ADR verdict, or PR with new or changed ADR |
| specifier | `claude-opus-5-5` | `xhigh` | 1–2 | The orchestrator assigns a feature | prd:specifying-stories | Feature id | PR with the spec in the feature folder |
| implementer | `claude-sonnet-5-5` | `high` | 1–4 | The orchestrator assigns a merged spec | relay (non-interactive), productivity:wrapping-up-sessions | Spec on `main` | PR, then merge and clean-up |
| reviewer | `claude-opus-5-5` | `high` | 1–4 | A PR is open | pr-flow:reviewing-prs, adr:reviewing-adr-adherence | PR number | Approve or request changes |
| verifier | `claude-opus-5-5` | `high` | 1–4 | An implementer opens a code PR | verify, ui-judge:* | PR number and spec | Approve or request changes, and test scenarios for the tester |
| tester | `claude-sonnet-5-5` | `high` | 1 | A verifier sends test scenarios | ui-judge:* | Test scenarios | PR with new or changed e2e tests |
| chores | `claude-sonnet-5-5` | `medium` | 1 | A manual step is necessary | — | Chore request | Checked state, after it guides the user through a user step if necessary |

Model and Effort: use the exact values in the table.

### Role details

**orchestrator**
- Keep a queue of features in roadmap order.
- As soon as a story gets the state `ready` on `main`, assign its spec to a free implementer at once. If no implementer is free, start a new implementer (the maximum is 4). Do not wait for other stories or features.
- Start a feature only when the features it depends on are merged, unless a user `ANSWER` allows an early start (rule 11).
- Run 2 features at the same time only if they do not change the same area.
- Start more implementers, reviewers and verifiers when the queue grows. The maximum is 4 of each.
- Send each code PR to a reviewer and to a verifier at the same time. Send a document PR or a test PR only to a reviewer.
- Send the test scenarios of each verifier to the tester.
- Start a second specifier when 2 features are ready for a spec. The maximum is 2 specifiers.
- Give each `ASK` a priority before you send it to hitl: `blocks code now` or `blocks later`.
- An `ASK` with a `revisit:` line goes to the owner of the page: the planner for a PRD decision, the architect for an ADR. An `ASK` with the line `owner: planner` goes to hitl in the batch of its feature. Send the `ANSWER` of a rule card or of an `owner: planner` card to the specifier and the planner. If the chosen option names a change to an accepted ADR, also send it to the architect. The architect changes that ADR in an ADR PR that names the `ANSWER`.
- Stop extra sessions when they have no work.
- After a retro merge, tell every live session to read `docs/adlc.md` again.
- After a PR that changes `docs/product` merges, send a `SYNC` to the projecter.
- Keep a short status table in the orchestrator session. Do not write it to the repo.
- If the orchestrator session name changes, the retro updates section 1 and the orchestrator sends `RELOAD`.

**hitl**
- The only session that asks the user for decisions.
- Use the card in section 7 for each question. Use the same card format each time.
- Ask the cards in priority order. First ask the cards that block code now, then the cards that block later.
- Ask the cards of 1 feature together, up to 4 cards at a time. Use `AskUserQuestion` when the options are fixed (it takes up to 4 questions).
- Send the answer to the orchestrator as an `ANSWER` message.

**retro**
- Starts when the orchestrator sends `RETRO`.
- Asks the user directly what to change and why.
- Updates `docs/adlc.md` with a short worktree, PR and merge loop (section 6).
- Sends the orchestrator a `DONE` message with a summary of the changes.
- Records each change in section 9.

**planner**
- Reads and updates the PRD and the roadmap with the prd skills.
- Lands each change through a PR.
- After the user answers a rule card (section 4, "Ask the Rule"), writes 1 PRD decision page for the rule. The page has the line `Principle: <rule>.` with the rule word for word from the chosen option, and the date of the answer. Sends no card for this page. If the chosen option named a change to a PRD decision, changes that page in the same PR.
- When a rule page changes, runs the drift check and sends 1 `TASK` to the orchestrator for 1 follow-up feature. Asks a card only for a done feature that the new answer did not name.

**projecter**
- Copies the roadmap on `main` to the board tool. Never edits the board in a way that `main` does not show.
- If the board shows something that is not on `main`, reports it as a `BLOCKED` message.

**architect**
- Answers design questions with the ADRs.
- If the ADR folder does not exist, creates it with `adr:scaffolding-adr-corpora`.
- Lands new or changed ADRs through a PR.
- Decides each ADR finding from a reviewer (section 4, "ADR finding flow"). Gives 1 verdict: `no ADR change`, `contradicts ADR-NNNN`, or `needs an ADR`.
- For `contradicts` and `needs an ADR`, sends an `ASK` to the orchestrator with the options and a recommendation. Does not write or change an ADR before the `ANSWER`.
- After the `ANSWER`, writes the new or changed ADR with `status: proposed` and lands it through a PR. The ADR names the `ANSWER`. Then the architect sends an `ASK` for the card that accepts the ADR.
- After a spec PR with the ADR of a rule merges (section 4, "Ask the Rule"), sets that ADR to `status: accepted` in an ADR PR and names the `ANSWER` of the rule card. The `ANSWER` is the card that accepts it. No second card is necessary.

**specifier**
- One feature gives one spec.
- Writes the spec in the feature folder with `prd:specifying-stories`.
- Lands the spec through a PR. The orchestrator assigns the spec to an implementer after the merge.
- In the spec PR, sets the stories of the feature to `ready` only if the feature is not blocked (rule 11). If the feature is blocked, the stories stay in `backlog`.
- Uses effort `xhigh`. The goal is a specifier that needs the user as little as possible.
- Infers each decision from the ADRs, the system design (`docs/product/system-design.md`), the PRD and the merged specs first. Records the source of each inferred decision in the spec.
- Does not ask the user to confirm a decision that an ADR or the system design states, or that is clearly inferred from them.
- Asks the user with the method "Ask the Rule" (section 4). It asks for the rule, not for the value. It infers each detail from the rule and records the source of each detail.

**implementer**
- One spec gives one PR.
- Works in its own worktree. Uses relay in non-interactive mode.
- Runs tests and lint before it opens the PR.
- Sends `REVIEW` with the PR number.
- Fixes the comments when the reviewer or the verifier requests changes.
- After the reviewer and the verifier approve: merges the PR, then runs `productivity:wrapping-up-sessions`. No worktree or branch must stay after the merge.

**reviewer**
- Reviews a PR with `pr-flow:reviewing-prs`.
- Checks the code and the tests against the spec criteria and the ADRs. The verifier runs the product.
- Checks ADR adherence on each PR with `adr:reviewing-adr-adherence`. Looks for 2 problems: a change that contradicts an accepted ADR, and a change that is expensive to undo and has no ADR.
- Does not decide an ADR finding alone. Requests changes, writes the finding in the PR comment, and sends it to the orchestrator for the architect (section 4, "ADR finding flow").
- In a spec PR, opens the source of each inferred row (section 4, "Ask the Rule"). Requests changes if the source does not support the choice, if a stronger source disagrees, or if a hard-to-undo change is not named on a card.
- Posts the result as a PR comment. The first line is `VERDICT: APPROVE` or `VERDICT: CHANGES`. The second line is `Role: reviewer`. The account cannot approve its own PR, so the reviewer does not use `gh pr review`.
- Document PRs (only files in `docs/` change): if a fix does not block, give the exact text of the fix with the approval. The author applies this text and merges with no new check. Code PRs and test PRs always get a new check after a fix.
- Sends the result to the orchestrator.
- Release readiness check (FEATURE-22 spec, decisions 17 to 19): the orchestrator gives the security review to a reviewer session that wrote none of the code under review. The session does the review on the commit of `main` that the spec names. It fills in the section "Security review" of `docs/release-readiness.md` in a docs PR, and opens one issue with the label `security` for each open medium or low finding. The issue names the place and the fix, and gives no steps of an attack. A different reviewer session reviews that PR. After the approval, the author session merges it with squash (rule 12) and removes its worktree and branch.

**verifier**
- Checks a code PR against the criteria of its spec. Runs the product from the PR branch as a user does. Does not review the code (the reviewer does that).
- For a CLI (for example jag), builds the binary and runs the jag binary in a terminal. For a web app, uses Playwright (`ui-judge` skills) to go to the pages and use them.
- Uses real input and the real program. No mocks.
- Builds and runs the PR head in its own worktree (`git worktree add --detach <path> <PR head>`), never in the main folder. Removes the worktree after the result.
- Writes in the PR comment each command or step, the result, and the spec criterion that it checks.
- Posts the result as a PR comment. The first line is `VERDICT: APPROVE` or `VERDICT: CHANGES`. The second line is `Role: verifier`. The account cannot approve its own PR, so the verifier does not use `gh pr review`.
- Sends the result to the orchestrator. Also sends a list of test scenarios for the tester. Each scenario gives the steps, the expected result and the spec criterion.

**tester**
- Writes and keeps the e2e tests. The tests check only the final user functions, as a QA engineer does.
- A test runs the built program as a user does: commands in a terminal, a local script, Playwright in a browser, or another e2e method.
- No mocks and no unit tests.
- Writes tests from the scenarios of the verifiers. Changes or removes a test when a spec changes the behavior.
- Before the first test, sends a design question to the orchestrator for the architect: where the tests go and how they run. The architect records the answer in an ADR.
- A test PR changes only the e2e tests in the place that the ADR of the tester names.
- Runs all e2e tests before it opens the PR. Sends `REVIEW` with the PR number. A reviewer checks the PR. After the approval, merges it with squash (rule 12).
- Does not change the product code. If a test fails on `main`, sends a `BLOCKED` message with the scenario and the output.

**chores**
- Does manual setup steps that do not need the user (for example: labels, checks, config).
- If the user must do a step (login, token scope, payment), guides the user directly through the step. Gives the exact commands, 1 step at a time, and checks the result after each step.
- Sends decisions (a choice between options) to the orchestrator as `ASK`. The hitl session asks the user.

## 4. Feature flow

```
roadmap (main)
  → orchestrator picks next ready feature
  → specifier: spec PR → merge
  → stories get the state ready (rule 11)
  → orchestrator at once assigns the spec to a free implementer (or starts one)
  → implementer: code PR → REVIEW
  → reviewer (code) and verifier (functions), at the same time: approve | request changes
       request changes → implementer fixes → reviewer and verifier (max 3 rounds)
       after 3 rounds  → hitl ASK
  → implementer: merge → wrap-up (no worktree, no branch left)
  → tester: e2e tests from the verifier scenarios → test PR → reviewer → merge (does not block done, unless the spec says so)
  → orchestrator: mark done, send SYNC to projecter if roadmap changed
```

Side flows:
- Design question from any role → architect.
- ADR finding from a reviewer → architect → hitl (section 4, "ADR finding flow").
- Rule question from the specifier → hitl. A `revisit:` → planner or architect (section 4, "Ask the Rule").
- Gap in PRD or roadmap → planner → PR → projecter `SYNC`.
- User decision → hitl.
- Manual step → chores.
- User changes a role → retro.

### ADR finding flow

```
reviewer: ADR finding → VERDICT: CHANGES → orchestrator
  → orchestrator: finding → architect (starts the architect if it is not running)
  → architect: verdict → orchestrator → reviewer and implementer
       no ADR change          → reviewer: new VERDICT for the PR
       contradicts ADR-NNNN   → ASK (change the code | change the ADR | deviate)
       needs an ADR           → ASK (accept the decision | change the code)
  → hitl: card → ANSWER
  → architect: ADR PR (status proposed) if the ANSWER changes an ADR, adds one, or deviates (`deviates_from`) → merge
  → implementer: code fix if the ANSWER says so
  → reviewer and verifier: new check of the code PR
```

- The orchestrator gives the `ASK` the priority `blocks code now`.
- The implementer does not change the code for an ADR finding before the `ANSWER`. An ADR finding and its new check are not one of the 3 review rounds.
- The code PR does not merge until the `ANSWER` is in, the ADR PR (if there is one) is merged, and the reviewer and the verifier approve the last code. In the new check, the ADR of the merged ADR PR is the decision for the finding.
- On the ADR PR of the architect, the reviewer checks that the ADR names the `ANSWER`. A change of an ADR that the `ANSWER` orders is not an ADR finding.
- An ADR finding from a review is the exception to "Write the ADR now ... Do not stop to ask" in `docs/adr/CLAUDE.md`. The user decides the direction first, because the review found a conflict.

### Ask the Rule

The specifier asks the user for the rule, not for the value. A spec meets these conditions:

1. A rule card asks about 1 rule. Its answer decides 2 or more open rows, changes a PRD decision or an ADR, or decides a thing that is hard to undo. Hard to undo: a done feature, a public option, released behavior, public data, money. The answer of a card does not depend on another card in the same batch. No card asks about a row that `main` decides at the time that you send the card.
2. Each rule card has the block `This answer decides:` with at most 5 lines, in plain words. 1 line can name a group of rows. The lines name each PRD decision and each ADR that changes, each done feature that changes, and each public option that goes away or changes.
3. Round 1: at most 2 rule cards. A priority question that changes the order of features or a `blocked_by` link goes in the same batch, with the line `owner: planner`. The batch has at most 4 cards (hitl). Round 2: at most 2 cards, only for rows that stay open. A card that you send again counts as a round 2 card. There is no round 3. After round 2, each open row gets the recommended option with the source `default after round cap`, and the PR body names the row. If that row is hard to undo, send `BLOCKED` with the cause.
4. A rule answer is 1 row with the source `human:<login>, <date>`. Each row that follows from it has the confidence `inferred`, the source `derived from row <n>`, and a place in the Assumptions list. If the user answers "other" or a different rule, the rows that followed from the old rule get derived again, and each card of the same batch whose premise changed goes out again.
5. Each other inferred row has the option that you would mark `[recommended]` and 1 source that a reviewer can open. The sources, from strong to weak: an accepted ADR, a PRD decision page (a page with the line `Principle:` holds a rule from an earlier answer), or a `decided` area of the system design with an accepted ADR, a row of a built spec, the behavior of the reference program (rg) at its pinned version, a line of the jag code. A weak source does not count if a stronger source disagrees. A row with no source stays open. A hard-to-undo row is inferred only when a rule card named it, because the user cannot undo it in a review round.
6. The body of the spec PR lists the inferred rows with their sources, the stories that need no build, and the spec groups. The reviewer approval is the answer "reject none", so the ledger question of the skill is not sent. Send these as 1 card only if you send round 2, or if the reviewer rejects a row.
7. The owner of the page decides a change to an accepted ADR or a PRD decision that no card named. Send an `ASK` with the line `revisit: <page>`, the row, and the change that you recommend. A `revisit:` of an ADR goes to the architect, and the ADR finding flow continues from the verdict of the architect. A `revisit:` of a PRD decision goes to the planner, who sends a card only if the chosen option did not name the change. Until the owner decides, keep the current choice in that row, mark it `pending revisit`, keep only the dependent stories in `backlog`, and write the rest of the spec.

The ADR of a rule: `prd:specifying-stories` (step 5) writes 1 ADR with `status: proposed` for each rule row that binds code outside the stories of the feature (the rule of step 5), and no ADR for a derived row. The ADR holds only what the rule card showed: the chosen option and the lines of `This answer decides:`. The `ANSWER` of the rule card is the card that accepts that ADR (see the architect).

## 5. Message format

The first line of each message is the header:

```
<TYPE> · <from> → <to> · <subject id>
```

Types:

| Type | Meaning |
|---|---|
| `TASK` | Do this work |
| `DONE` | Work is complete. Give links (PR, file) |
| `REVIEW` | PR is ready for review |
| `ASK` | A question for the user. Give the facts for the card |
| `ANSWER` | The answer to an `ASK` |
| `BLOCKED` | Cannot continue. Give the cause |
| `SYNC` | Copy `main` to the board |
| `RETRO` | User wants a team change |
| `RELOAD` | `docs/adlc.md` changed. Read it again |

## 6. Retro fast path

1. Make a worktree from `origin/main`.
2. Change `docs/adlc.md` only.
3. Open a PR. Send `REVIEW` to the orchestrator. A reviewer checks it as a document PR (section 3, reviewer). After the approval, merge it with squash (rule 12).
4. Remove the worktree and the branch.
5. Send `DONE` to the orchestrator. The orchestrator sends `RELOAD` to all sessions.

## 7. HITL card

Always use this layout, in this order:

1. The header: `from <role> → "<title>"`. The title tells the topic of the question in a few plain words.
2. The related line: `Related: <background in plain words>`. If there is no useful background, do not write this line.
3. The question, in 1 sentence.
4. The decides block, for a rule card only (section 4, "Ask the Rule"): the line `This answer decides:` and at most 5 lines below it. Each line names 1 detail, or 1 group of details, that the recommended option decides.
5. The options list: 1 bullet for each option, with the pros and the cons indented below it.

Example (markdown source):

```markdown
from specifier → "The label for standard input"

Related: A search needs a path until the team adds search of standard input.

What do we build now for the "(standard input)" label?

- **Only the output code:** a result can come from a file or from standard input, and standard input prints "(standard input)". A unit test checks it. The end-to-end test comes later. [recommended]
  - **Pros:** ✅
    - This work stays small
    - The standard input code is not written 2 times
  - **Cons:** ❌
    - Only the later end-to-end test fully proves the label
- **The full standard input search now:** read standard input and print the label, with an end-to-end test.
  - **Pros:** ✅
    - The label is fully proven now
  - **Cons:** ❌
    - This work gets much larger
```

Rules:
- Plain words: Write the header, the related line, the question and the options in plain English (ASD-STE100 and the Google developer documentation style). Do not use IDs of features, stories, decisions, constraints or criteria, and do not use question IDs. Say what the item does in plain words.
- Question IDs: Hitl keeps the question ID for itself. It uses the ID in the `ANSWER` message to the orchestrator, but not in the card.
- Options: Do not number the options. Start each option with a short bold name and a colon, then the description. Do not use tables, because tables do not show correctly on a mobile screen.
- Recommended option: Put the text `[recommended]` at the end of the option. Do not write a separate recommendation line.
- Decides block: Write the lines in plain words, with no IDs. The user can veto 1 line with the answer `other: but <line>`.
- Pros and cons: Write `**Pros:** ✅` and `**Cons:** ❌`. Give each option 1 or more pros and 1 or more cons. Put each pro and each con on its own indented bullet. Keep each bullet to a short phrase.
- Emojis: Use only ✅ and ❌, and only on the Pros and Cons lines.
- No repeated text: Give each fact 1 time only. For example, put the background in the related line, not in each option.
- Number of options: Use 2 or 3 options. If there are 4 or more options, keep this layout.
- `AskUserQuestion`: Use the short bold name of each option as its label.

## 8. Start commands

The orchestrator starts each session from the main folder:

```
cd <main folder> && claude --bg -n <prefix>-<role>[-n] --model <model> [--effort <effort>] --permission-mode bypassPermissions "<start prompt>"
```

Use the Model and Effort columns in section 3 for `--model` and `--effort` (for example, a reviewer uses `--model claude-opus-5-5 --effort high`).
A live session does not change its model or effort after `RELOAD`. If section 3 changes the model, the effort or the name of a role, restart each session of that role when it has no work.
If a role gets a new name and a new role gets the old name, the orchestrator stops each live session of the old name before it sends `RELOAD`. Then it starts the session again with the new name. Else, after the `RELOAD`, the session reads the new role that has the old name.

Start prompt:

```
You are the <role> of the <prefix> team. Read docs/adlc.md on main.
Do the role "<role>" as written there. The orchestrator is <orchestrator name>.
Send a DONE message to the orchestrator when you are ready. Then wait for TASK messages.
```

## 9. Change log

| Date | Change | Why |
|---|---|---|
| 2026-10-13 | Created from the sofa template version 1 | Team setup for jag |
| 2026-10-13 | Corrected the ADR folder row. Added the rule that the ADRs win over `docs/handoff.md` | The ADR folder has 10 ADRs on `main`. Sessions must know which source wins |
| 2026-10-13 | Removed an internal project name and its worktree path from section 1. Kept the rule | CONSTRAINT-01: the public repository contains no internal names. `docs/handoff.md` is not in git, so it keeps the details |
| 2026-10-13 | Changed the orchestrator name to `jag-orchestrator`. Added the rule for a name change to the orchestrator details | The orchestrator session name changed. Sessions send messages to this name |
| 2026-10-13 | Replaced the HITL card in section 7 with the layout "Option C": 2 context lines, the question, a bordered options table, and 1 recommendation line | The user chose this layout. It gives the background first, and the table borders stay in line |
| 2026-10-13 | The chores session can talk to the user directly and guide the user through manual steps. Decisions still go through hitl | The user asked for this change |
| 2026-10-13 | Replaced the options table in the HITL card (section 7) with a bullet list. Each option has its pros and cons as indented bullets | The user asked for this change. A table does not show correctly on a mobile screen |
| 2026-10-13 | Hitl asks the cards in priority order, up to 4 cards of 1 feature together. Up to 2 specifiers. The specifier uses effort `xhigh`, infers more from the ADRs and the system design, and asks guideline questions. For document PRs, an exact non-blocking fix merges with no new check | The user asked for these changes to make the team faster. Today, code work stopped for about 3 hours because specs waited for user answers |
| 2026-10-13 | Changed the HITL card (section 7): a header with the role and a plain title, plain words with no IDs, options with no numbers, ✅ and ❌ on the Pros and Cons lines, and `[recommended]` with no separate recommendation line | The user asked for this change. The cards must be easy to read with no knowledge of the IDs |
| 2026-10-13 | Added rule 11: a story is `ready` only if its spec is merged, its feature is not blocked, and no open `ASK` blocks it. The orchestrator assigns a ready story to an implementer at once. The specifier and each PR that sets a feature to `done` set the story state by this rule | The user asked for this change. 9 stories showed `ready` but were blocked or had no spec |
| 2026-10-14 | Added an Effort column to section 3. Orchestrator: `claude-opus-5-5`, `medium`. Hitl: `claude-sonnet-5-5`, `medium`. Planner: `claude-opus-5-5`, `medium`. Projecter: `claude-sonnet-5-5`, `medium`. Architect: `claude-opus-5-5`, `high`. Implementer: `claude-sonnet-5-5`, `high`. Verifier: `claude-opus-5-5`, `high`. Chores: `claude-sonnet-5-5`, `medium`. Retro: `claude-opus-5-5`, `medium`. Specifier: `claude-opus-5-5`, `xhigh`. Section 8 uses these values and restarts a session when its model or effort changes | The user asked for this change |
| 2026-10-14 | Removed the ADR count from the ADR folder row in section 1. The row points to `docs/adr/README.md` | The count (10) was out of date: `main` has 18 accepted ADRs. A count goes out of date again |
| 2026-10-15 | Rule 11: added the exception for a user `ANSWER` (FEATURE-18). Added rule 12: squash merge only, and a hash check of the author and the committer after each merge. The "Extra context files" row points to `docs/handoff.md` in the old folder `jag-private`. Section 4 points to the rule 11 exception. The repository rules point to the "Extra context files" row | Verifier comments on PR #94 and PR #95. A rebase merge wrote the account address as committer in `3ead9ae`. Since FEATURE-19 step B10, the main folder is a new clone and has no `docs/handoff.md` |
| 2026-10-15 | Rule 11: FEATURE-18 used the exception until FEATURE-19 was `done` | Verifier comment on PR #100. FEATURE-19 is `done` (`d023dd1`). STORY-48 to STORY-51 are now `ready` by the base rule |
| 2026-10-15 | The verifier checks ADR adherence on each PR (`adr:reviewing-adr-adherence`): a contradiction of an accepted ADR, or a change that needs an ADR. The architect decides each finding. Hitl asks the user. The architect writes the ADR after the `ANSWER`. Added the "ADR finding flow" to section 4. The retro fast path (section 6) gets a verifier check and a squash merge. The verifier posts a `VERDICT` comment and does not use `gh pr review` | The user asked for this change. The review must catch ADR problems, and the user decides ADR changes. Verifier comment on PR #103: section 6 and the verifier line did not agree with how the team works |
| 2026-10-15 | Changed the name of the verifier role to reviewer (it reviews PRs). Added a new verifier role: it runs the product from the PR branch (a terminal for a CLI, Playwright for a web app), checks the spec criteria, and sends test scenarios. Added the tester role: it writes and keeps the e2e tests, with no mocks and no unit tests. Added the Build row to section 1. Section 8 restarts a session when its role name changes, and stops a session of the old name before `RELOAD`. The verifier uses its own worktree and checks the last code after an ADR `ANSWER`. The tester sends design questions through the orchestrator. Defined a test PR | The user asked for this change. A review of the code does not show that the functions work for the user |
| 2026-10-15 | Reviewer: for a release readiness check, a reviewer session does the security review and writes `docs/release-readiness.md` in a docs PR. A different reviewer session reviews that PR | FEATURE-22 spec decision 18 (PR #112). Reviewer comment on PR #112: the reviewer role did not open PRs |
| 2026-10-15 | Added the method "Ask the Rule" to section 4: the specifier asks for the rule, not for the value; at most 2 rule cards, then at most 2 more, at most 4 cards in a batch; each rule card has a `This answer decides:` block; the specifier infers each detail from the rule with the recommended option and a source that a reviewer can open; a change to a PRD decision or an ADR goes as a `revisit:` line to the planner or the architect. The planner writes 1 PRD decision page for each rule. The architect accepts the ADR of a rule with the `ANSWER`. The reviewer opens the source of each inferred row. Section 7 gets the decides block | The user asked for this change. The FEATURE-23 spec sent 7 cards in 2 rounds, and the 5 detail cards all followed from the first rule answer. With this method, FEATURE-23 sends 2 cards in 1 batch |
