#!/usr/bin/env bash
# Download the pinned ripgrep for this machine, check it against the pin in
# tests/parity/rg.sha256, and extract rg (ADR-0021).
#
# Usage: scripts/parity/install-rg.sh [DIR]
#
# DIR is the folder for the download and the extracted files. The default is a
# new temporary folder. The script prints the path of the rg file on stdout and
# nothing else. All other messages go to stderr.
#
# CI sets the variable for the tests like this:
#   echo "JAG_PARITY_RG=$(scripts/parity/install-rg.sh "$RUNNER_TEMP/rg")" >> "$GITHUB_ENV"
set -euo pipefail

here=$(cd "$(dirname "$0")" && pwd)
pin="$here/../../tests/parity/rg.sha256"

case "$(uname -s)-$(uname -m)" in
  Linux-x86_64) target=x86_64-unknown-linux-musl ;;
  Linux-aarch64 | Linux-arm64) target=aarch64-unknown-linux-musl ;;
  Darwin-arm64) target=aarch64-apple-darwin ;;
  Darwin-x86_64) target=x86_64-apple-darwin ;;
  *)
    echo "install-rg: no pinned ripgrep for $(uname -s) $(uname -m)." >&2
    exit 1
    ;;
esac

line=$(grep -F -e "-$target.tar.gz" "$pin" || true)
if [ "$(printf '%s\n' "$line" | grep -c .)" != 1 ]; then
  echo "install-rg: the pin file must have one line for $target." >&2
  exit 1
fi
archive=${line##* }
version=${archive#ripgrep-}
version=${version%%-*}
folder=${archive%.tar.gz}

dir=${1:-$(mktemp -d)}
mkdir -p "$dir"
dir=$(cd "$dir" && pwd)

url="https://github.com/BurntSushi/ripgrep/releases/download/$version/$archive"
echo "install-rg: download $url" >&2
curl --fail --silent --show-error --location --retry 3 --output "$dir/$archive" "$url"

# The pin in the repository is the check. The .sha256 file of the release
# comes from the same place as the archive, so it is not used.
printf '%s\n' "$line" > "$dir/pin.sha256"
if command -v sha256sum > /dev/null 2>&1; then
  (cd "$dir" && sha256sum -c pin.sha256) >&2
else
  (cd "$dir" && shasum -a 256 -c pin.sha256) >&2
fi

tar -xzf "$dir/$archive" -C "$dir" "$folder/rg"
chmod +x "$dir/$folder/rg"
echo "install-rg: $("$dir/$folder/rg" --version | head -n 1)" >&2
printf '%s\n' "$dir/$folder/rg"
