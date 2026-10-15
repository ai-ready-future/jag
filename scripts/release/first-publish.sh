#!/usr/bin/env bash
# first-publish.sh: step U3 of the first release.
#
# It publishes an empty version 0.0.0 of the crate jag-grep and of the five
# npm packages, from the machine of the user. Each registry needs a first
# version before it accepts a trusted publisher. The spec is
# docs/superpowers/specs/2026-10-14-STORY-48-install-jag-from-github-releases-crates-io-or-npm-design.md,
# decision 20 and "The user steps before the first tag".
#
# The user runs this script. A session never runs it without --dry-run.
#
# Before you run it:
#   - U1 is done: the npm organization airf exists, and you own it.
#   - You are logged in to npm (npm login). npm asks for the 2FA code at each publish.
#   - You made a crates.io token for this step only. Set CARGO_REGISTRY_TOKEN,
#     or the script asks for the token. The script never writes the token.
#
# Usage: scripts/release/first-publish.sh [--dry-run]
#   --dry-run  Make the packages and ask each registry tool for a dry run.
#              Nothing is published, and no token is needed.
#
# The script works in a new temporary folder, and removes it at the end. It
# reads no key file.
set -euo pipefail

VERSION=0.0.0
CRATE=jag-grep
REPOSITORY=https://github.com/ai-ready-future/jag
EMPTY_TEXT="This version holds no program. Install a later version."

DRY_RUN=false
for arg in "$@"; do
  case $arg in
    --dry-run) DRY_RUN=true ;;
    -h | --help)
      sed -n '2,/^set -euo/p' "$0" | sed '$d' | sed 's/^# \{0,1\}//'
      exit 0
      ;;
    *)
      echo "first-publish: unknown option $arg" >&2
      exit 2
      ;;
  esac
done

SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
LICENSE_FILE="$SCRIPT_DIR/../../LICENSE"
if [[ ! -f $LICENSE_FILE ]]; then
  echo "first-publish: the file LICENSE is not at the root of the repository." >&2
  exit 1
fi
for tool in cargo npm; do
  if ! command -v "$tool" >/dev/null 2>&1; then
    echo "first-publish: $tool is not installed." >&2
    exit 1
  fi
done

WORK=$(mktemp -d)
chmod 700 "$WORK"
trap 'rm -rf "$WORK"' EXIT

# The crate: Cargo.toml, an empty src/lib.rs, LICENSE, and README.md.
make_crate() {
  local dir=$WORK/crate
  mkdir -p "$dir/src"
  : >"$dir/src/lib.rs"
  cp "$LICENSE_FILE" "$dir/LICENSE"
  printf '# %s\n\n%s\n' "$CRATE" "$EMPTY_TEXT" >"$dir/README.md"
  cat >"$dir/Cargo.toml" <<EOF
[package]
name = "$CRATE"
version = "$VERSION"
edition = "2021"
license = "MIT"
description = "Grep with a model that ranks the matches by how well they answer your question. This version holds no program."
repository = "$REPOSITORY"
readme = "README.md"
EOF
}

# One npm package: package.json, LICENSE, and README.md.
# Arguments: folder name, package name, os, cpu. An empty os means the main package.
make_npm() {
  local dir=$WORK/npm/$1 name=$2 os=${3:-} cpu=${4:-}
  mkdir -p "$dir"
  cp "$LICENSE_FILE" "$dir/LICENSE"
  printf '# %s\n\n%s\n' "$name" "$EMPTY_TEXT" >"$dir/README.md"
  {
    printf '{\n'
    printf '  "name": "%s",\n' "$name"
    printf '  "version": "%s",\n' "$VERSION"
    printf '  "description": "%s",\n' "$EMPTY_TEXT"
    printf '  "license": "MIT",\n'
    printf '  "repository": { "type": "git", "url": "git+%s.git" },\n' "$REPOSITORY"
    if [[ -n $os ]]; then
      printf '  "os": ["%s"],\n' "$os"
      printf '  "cpu": ["%s"],\n' "$cpu"
    fi
    printf '  "publishConfig": { "access": "public" }\n'
    printf '}\n'
  } >"$dir/package.json"
}

make_crate
make_npm jag-linux-x64 @airf/jag-linux-x64 linux x64
make_npm jag-linux-arm64 @airf/jag-linux-arm64 linux arm64
make_npm jag-darwin-arm64 @airf/jag-darwin-arm64 darwin arm64
make_npm jag-darwin-x64 @airf/jag-darwin-x64 darwin x64
make_npm jag @airf/jag

# A second run skips each version that a registry has already. So it can end
# a first publish that stopped part way.
crate_exists() {
  [[ $(curl -s -o /dev/null -w '%{http_code}' -A "jag first-publish ($REPOSITORY)" \
    "https://crates.io/api/v1/crates/$CRATE/$VERSION") == 200 ]]
}
npm_exists() {
  [[ $(npm view "$1@$VERSION" version 2>/dev/null) == "$VERSION" ]]
}

if [[ $DRY_RUN == true ]]; then
  echo "first-publish: dry run. Nothing is published."
else
  echo "first-publish: this publishes version $VERSION of $CRATE and of 5 npm packages."
  echo "first-publish: a publish cannot be undone."
  read -r -p "Type yes to continue: " answer
  if [[ $answer != yes ]]; then
    echo "first-publish: stopped. Nothing is published." >&2
    exit 1
  fi
fi

# The crate. The token is in the environment of the cargo command only.
if [[ $DRY_RUN == true ]]; then
  (cd "$WORK/crate" && cargo publish --dry-run)
elif crate_exists; then
  echo "first-publish: $CRATE $VERSION is on crates.io already. Skip."
else
  token=${CARGO_REGISTRY_TOKEN:-}
  if [[ -z $token ]]; then
    read -r -s -p "crates.io token for this step: " token
    echo
  fi
  if [[ -z $token ]]; then
    echo "first-publish: no crates.io token." >&2
    exit 1
  fi
  (cd "$WORK/crate" && CARGO_REGISTRY_TOKEN=$token cargo publish)
  unset token
fi

# The npm packages: the platform packages first, and the main package last.
for folder in jag-linux-x64 jag-linux-arm64 jag-darwin-arm64 jag-darwin-x64 jag; do
  if [[ $DRY_RUN == true ]]; then
    (cd "$WORK/npm/$folder" && npm publish --dry-run --access public)
  elif npm_exists "@airf/$folder"; then
    echo "first-publish: @airf/$folder $VERSION is on npm already. Skip."
  else
    (cd "$WORK/npm/$folder" && npm publish --access public)
  fi
done

echo "first-publish: done."
if [[ $DRY_RUN != true ]]; then
  echo "Next: step U4, the trusted publishers. See scripts/release/README.md."
fi
