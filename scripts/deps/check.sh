#!/usr/bin/env bash
# The dependency checks of jag (FEATURE-22 spec, STORY-63).
#
#   1. cargo-deny: the advisories, the licenses, the bans, and the sources
#      (the policy is deny.toml).
#   2. npm audit of the npm packages.
#   3. release.py check-npm-deps: the npm packages have no third-party
#      dependency.
#
# Run it from the root of the repository. It needs cargo, npm, curl, tar, and
# sha256sum. It reads no secret.
set -euo pipefail

CARGO_DENY_VERSION=0.20.2
CARGO_DENY_ARCHIVE="cargo-deny-${CARGO_DENY_VERSION}-x86_64-unknown-linux-musl"
CARGO_DENY_SHA256=9f12ed4c49936e09b48bf862b595cde2fe64fcbd9d74dfacac6131ca824c8d5f

if [ ! -f deny.toml ] || [ ! -f npm/jag/package.json ]; then
  echo "deps: run this script from the root of the repository." >&2
  exit 2
fi

work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

echo "deps: download cargo-deny $CARGO_DENY_VERSION"
mkdir "$work/cargo-deny"
curl -sSfL -o "$work/cargo-deny/$CARGO_DENY_ARCHIVE.tar.gz" \
  "https://github.com/EmbarkStudios/cargo-deny/releases/download/$CARGO_DENY_VERSION/$CARGO_DENY_ARCHIVE.tar.gz"
(cd "$work/cargo-deny" && echo "$CARGO_DENY_SHA256  $CARGO_DENY_ARCHIVE.tar.gz" | sha256sum -c -)
tar -xzf "$work/cargo-deny/$CARGO_DENY_ARCHIVE.tar.gz" -C "$work/cargo-deny" "$CARGO_DENY_ARCHIVE/cargo-deny"

echo "deps: cargo deny check"
"$work/cargo-deny/$CARGO_DENY_ARCHIVE/cargo-deny" --locked check advisories licenses bans sources

# The packages have no lock file in the repository, so npm audit needs a new
# one. It is made in a new folder, so the repository does not change.
echo "deps: npm audit"
mkdir "$work/npm"
cp npm/jag/package.json "$work/npm/package.json"
(
  cd "$work/npm"
  npm install --package-lock-only --ignore-scripts --no-audit --no-fund
  npm audit
)

echo "deps: check-npm-deps"
python3 -I scripts/release/release.py check-npm-deps npm/jag/package.json

echo "deps: all checks passed."
