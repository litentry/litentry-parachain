#!/usr/bin/env bash
set -eo pipefail

function usage() {
  echo "Usage:   $0 litentry|litmus [srtool image]"
  echo "default: $0 litentry|litmus paritytech/srtool"
}

[ $# -lt 1 ] && (usage; exit 1)

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR"

CHAIN=$1
SRTOOL_IMAGE=${2:-paritytech/srtool}

case "$1" in
  litentry|litmus) ;;
  *) usage; exit 1 ;;
esac

echo "build $CHAIN-parachain-runtime using $SRTOOL_IMAGE"

# install / update the srtool-cli
cargo install --git https://github.com/chevdor/srtool-cli

# build the runtime
srtool -i "$SRTOOL_IMAGE" build -p $CHAIN-parachain-runtime -r runtime/$CHAIN

echo "============================"
echo "Done, the wasms are under runtime/target/srtool/release/wbuild/litentry-parachain-runtime/"
