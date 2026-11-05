#!/usr/bin/env bash
# private-data check: find out if GitHub has the merge ref of a pull request.
# GitHub makes no merge ref (refs/pull/<n>/merge) when the pull request
# conflicts with its base branch. The spec is
# docs/superpowers/specs/2026-11-04-STORY-80-name-the-merge-conflict-when-private-data-cannot-check-out-a-pr-design.md.
#
# Input (environment):
#   PR_NUMBER  required. The number of the pull request (a whole number).
#   REPO_URL   required. The URL of the repository (a folder path also works).
#   BASE_REF   required. The base branch of the pull request.
#   MERGE_REF_TRIES  optional. The most checks. Default: 6.
#   MERGE_REF_PAUSE  optional. The seconds to wait between two checks. Default: 10.
#
# Exit codes: 0 the merge ref exists, 1 it does not exist (a conflict),
# 2 the check cannot run.
#
# This script sends no token. It reads only the names and the commits of the
# refs. It fetches no commit and reads no file of the pull request. It never
# uses "set -x", and it prints no ref list and no commit.
set -euo pipefail

PR_NUMBER="${PR_NUMBER:-}"
REPO_URL="${REPO_URL:-}"
BASE_REF="${BASE_REF:-}"
MERGE_REF_TRIES="${MERGE_REF_TRIES:-6}"
MERGE_REF_PAUSE="${MERGE_REF_PAUSE:-10}"

fail() { # exit-code message
  local code=$1
  shift
  echo "$*" >&2
  exit "$code"
}

[[ $PR_NUMBER =~ ^[0-9]+$ ]] || fail 2 "private-data: PR_NUMBER must be a whole number"
[[ -n $REPO_URL ]] || fail 2 "private-data: REPO_URL is empty"
[[ -n $BASE_REF ]] || fail 2 "private-data: BASE_REF is empty"
[[ $MERGE_REF_TRIES =~ ^[1-9][0-9]*$ ]] || fail 2 "private-data: MERGE_REF_TRIES must be a whole number, 1 or more"
[[ $MERGE_REF_PAUSE =~ ^[0-9]+$ ]] || fail 2 "private-data: MERGE_REF_PAUSE must be a whole number"

# No question about a user name or a password: a private repository fails at once.
export GIT_TERMINAL_PROMPT=0

for ((try = 1; try <= MERGE_REF_TRIES; try++)); do
  # git exits 2 when no ref matches. Keep the code, so that "set -e" does not stop the script.
  rc=0
  git ls-remote --exit-code "$REPO_URL" "refs/pull/$PR_NUMBER/merge" >/dev/null 2>&1 || rc=$?
  case $rc in
    0)
      echo "private-data: the merge ref of PR #$PR_NUMBER exists"
      exit 0
      ;;
    2)
      if ((try < MERGE_REF_TRIES)); then
        echo "private-data: no merge ref for PR #$PR_NUMBER yet; next check in $MERGE_REF_PAUSE s"
        sleep "$MERGE_REF_PAUSE"
      fi
      ;;
    *)
      fail 2 "::error::private-data: cannot read the refs of the repository (git exit $rc). Run the job again."
      ;;
  esac
done

fail 1 "::error::private-data: PR #$PR_NUMBER conflicts with $BASE_REF: merge $BASE_REF into the branch"
