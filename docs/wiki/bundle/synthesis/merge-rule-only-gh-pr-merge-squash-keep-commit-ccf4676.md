---
type: Synthesis
title: 'Merge rule: only gh pr merge --squash, keep commit ccf4676'
description: 'Human answer of 2026-11-03: keep the wrong author on ccf4676, allow only gh pr merge --squash, and keep the author check.'
status: draft
generated:
  by: claude-code/opus-5.5
  at: '2026-11-03T14:39:59Z'
---

# Merge rule: only gh pr merge --squash, keep commit ccf4676

Human answer of 2026-11-03: keep the wrong author on ccf4676, allow only gh pr merge --squash, and keep the author check.

## Question

What to do about a wrong author address on the public `main` branch? It is on the merge commit `ccf4676` of PR #279.

## Facts

- GitHub GraphQL was rate limited. The merger squash merged with the REST API (`PUT /pulls/<n>/merge`).
- That merge wrote an author address that is not the GitHub noreply address of `78aa3f5` and the earlier commits.
- The committer was correct.
- The author check of `docs/adlc.md` section 1 found the error after the merge.

## Answer

Keep the commit and fix the merge rule.

- The history of `main` does not change. This is the same choice as on 2026-10-15 (commit `9634a17`, FEATURE-19 spec decision 13).
- The rules permit only `gh pr merge --squash`. The merger waits when GitHub limits the rate.
- The author check after each merge stays.
- The user checks later that the GitHub setting "Keep my email addresses private" is on. The user thinks about the setting "Block command line pushes that expose my email".

## Update 2026-11-03 (D-006): the merge command still wrote the wrong author

Facts:

- After the answer above, the merger used `gh pr merge --squash`. The merge commit `ce76f88` again got an author address that is not the noreply address.
- All commits of the PR had the noreply address.
- So the merge method is not the cause. GitHub takes the squash author address from an account setting that changed on 2026-11-03.

Answer (human `404pilo`, through hitl, 2026-11-03, veracity 90): set the author address in the merge command. Decision: D-006. It extends D-002.

- The merger runs `gh pr merge <n> --squash --author-email "$(git log -1 --format=%ae 78aa3f5)"`. It reads the address from git and never prints it.
- If such a merge fails, the merger stops and asks. It never merges without the flag.
- The author check after each merge stays.
- The user turns on "Keep my email addresses private" again. Merges do not wait for this.
- `ce76f88` is kept, as `ccf4676` is.

Card from decision-judge: level middle, confidence medium, hard to reverse, hidden, band strict. This page holds no email address.

## Update 2026-11-03: the address source

Question: which address goes on the commits of the user? (card "Which email address goes on your commits")

Answer (human `404pilo`, through hitl, 2026-11-03, veracity 90): "Your chosen address for all new work".

- The chosen address goes on all new commits and merges.
- It is kept in the repository-local git config of the main folder (`user.email`). It is never printed and never in a repository file.
- The checks before each push and after each merge compare with it.
- Earlier commits keep their addresses.
- It replaces the address source of D-006 (the read from `78aa3f5`). The rest of D-006 stays: the `--author-email` flag, the stop when a merge fails, and the author check.

This page holds no email address.

## Provenance

- Decided by: human `404pilo`, through hitl, on 2026-11-03 (veracity 90).
- Card from decision-judge: level middle, confidence medium, hard to reverse, hidden, band strict.
- This page holds no email address.
