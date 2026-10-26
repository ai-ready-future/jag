# Private-data check

`check.sh` finds keys and private words in a repository. The workflow
`.github/workflows/private-data.yml` runs it on each pull request.

The list of private patterns is not in the repository. It is the GitHub
Actions secret `PRIVATE_PATTERNS`. It has one pattern on each line:

- Each pattern is a Go regular expression (RE2). The check ignores case.
- A line that starts with `#` is a comment. A blank line is skipped.
- The rule id in the output is `private-pattern-N`. N is the line number
  in the list.

## Run the check on your machine

1. Download gitleaks v8.30.1 and check its SHA-256 against the value in
   `.github/workflows/private-data.yml`.
2. Keep the pattern list in a private file outside the repository.
3. Run:

   ```
   $ PRIVATE_PATTERNS="$(cat /path/to/private-list)" \
     GITLEAKS=/path/to/gitleaks \
     REPO_DIR=. \
     scripts/private-data/check.sh
   ```

   To scan the commits of a branch, also set `BASE_SHA` and `HEAD_SHA`.
   `HEAD_SHA` must be the second parent of the commit of `REPO_DIR`. Set
   `PR_TITLE` and `PR_BODY` to scan the pull request text.

Exit code 0 means no finding. Exit code 1 means a finding. Exit code 2
means that the check cannot run. Do not share the output of a run that
used your private list.

## History mode

Set `HISTORY=main` to scan each commit of `main`. Set `HISTORY=all` to scan
each commit of each ref, also each `refs/pull/*` ref. Use a mirror clone
(`git clone --mirror`) to have each ref. The check reads the patch of each
commit, each commit message, the author and committer names and addresses,
each file name, and (for `all`) each ref name. It prints the commit (12
characters) and its place for each finding, never the value. It also lists
each binary file. A binary file is not a finding, but the clean step cannot
change it.

Set only one of `BASE_SHA`, `HISTORY`, and `TEXT_FILE`.

## Text mode

Set `TEXT_FILE` and `TEXT_INDEX`. Line N of `TEXT_INDEX` names the place of
line N of `TEXT_FILE`. For each finding, the check prints the place and the
rule id.

## The go-public scripts

The workflow `go-public.yml` is removed, because the move to the public
repository is done. The scripts stay. Run them by hand when you need them:

- `check.sh` in history mode scans the history of the repository.
- `issues.sh` scans each issue, its comments, the revisions of its text, the
  labels, and the milestones.
- `clean-history.sh` makes one clean history of `main`. Each match of a
  private pattern becomes `REDACTED`, each `.pyc` file goes out, and each
  `#N` in a commit message becomes `<old repo>#N`.

The steps of the move to a new public repository are in
`docs/superpowers/specs/2026-10-14-STORY-47-go-public-with-a-clean-history-design.md`.

## Fork pull requests

A run of the check for a pull request from a fork waits for the approval of
a maintainer (FEATURE-28, ADR-0027). The job `private-data` has the
environment `private-data-fork` for a fork run. That environment has a
required reviewer. All other runs use the environment `private-data-auto`,
which has no rule, so they start at once. A waiting run has no result, so the
required check stops the merge. Each new push and each edit of a fork pull
request makes a new run that needs a new approval.

The first step of the job fails a fork run when the variable
`PRIVATE_DATA_FORK_GATE` of its environment is not `on`. So a fork run in a
missing or wrong environment fails, and no scan runs.

### Who approves

Only the user, as `404pilo`, approves or rejects a waiting run, by hand on
GitHub: open the run page, choose "Review deployments", then "Approve and
deploy" or "Reject". No session approves or rejects a waiting run, also not
through the API (`pending_deployments`). The sessions use the token of the
same account, so GitHub cannot tell a session from the user. A session can
read the waiting state.

### Before you approve

Before you approve a waiting run, examine these items on the pull request
page:

1. The diff of all files. The change must have a clear purpose for jag.
2. The title, the body, and the commit messages. Look for text that has no
   purpose in the change, for example names, words, addresses, or strings
   that look like keys or guesses.
3. The history of the pull request. Many small pushes or edits that each
   change only a word or a short string are a probe, mainly after an earlier
   approved run failed.
4. The run. Approve only the newest waiting run. Reject the older waiting
   runs of the same pull request first.

When an item is not clear, reject the run and ask the author in a comment. A
rejected run fails the check and tells nothing about private data. Close a
pull request that probes, with no merge.

### The result is public after the approval

After an approval, the scan runs, and each person can see the result: a pass,
or a fail with the line `private-data: private data found`. The output names
no file and no line. But the pass or the fail is public. So each approved run
gives one public yes-or-no answer about the text of the pull request. The
approval does not hide that answer. It lets the maintainer decide which text
gets an answer, and how many answers a person gets.

## Run the tests

```
$ FILTER_REPO=/path/to/git-filter-repo GITLEAKS=/path/to/gitleaks scripts/private-data/test.sh
```

The tests use invented patterns and need no secret. Without `FILTER_REPO`
(git-filter-repo v2.47.0), the tests of the clean step do not run.
