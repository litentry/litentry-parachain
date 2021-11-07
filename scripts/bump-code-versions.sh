#!/usr/bin/env bash
set -eo pipefail

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR"

function usage() {
  echo "Usage:   $0 from to"
  echo "   eg:   $0 0.9.11 0.9.12"
}

[ $# -ne 2 ] && (usage; exit 1)

echo "bumping codes from v$1 to v$2 ..."

for f in 'node/Cargo.toml' 'runtime/Cargo.toml' 'primitives/Cargo.toml'; do
  sed -i '' "s/polkadot-v$1/polkadot-v$2/g" "$f"
  sed -i '' "s/release-v$1/release-v$2/g" "$f"
done

echo "done"
