#!/usr/bin/env bash
set -eo pipefail

function usage() {
  echo "Usage:   $0 [srtool image]"
  echo "default: $0 paritytech/srtool"
}

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR"

SRTOOL_IMAGE=paritytech/srtool

case "$1" in
  -h|help|--help) usage; exit ;;
  '') ;;
  *) SRTOOL_IMAGE="$1" ;;
esac

echo "using $SRTOOL_IMAGE"

# install / update the srtool-cli
cargo install --git https://github.com/chevdor/srtool-cli

# build the runtime
srtool -i "$SRTOOL_IMAGE" build -p litentry-parachain-runtime -r runtime

echo "============================"
echo "Done, the wasms are under runtime/target/srtool/release/wbuild/litentry-parachain-runtime/"
