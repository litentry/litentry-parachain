#!/usr/bin/env bash
set -eo pipefail

function print_divider() {
  echo "------------------------------------------------------------"
}

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR/docker"

PARCHAIN_LAUNCH_BIN="$ROOTDIR/docker/node_modules/.bin/parachain-launch"

CHAIN_TYPE=${1:-dev}

print_divider

if [ ! -f "$PARCHAIN_LAUNCH_BIN" ]; then
  echo "installing parachain-launch ..."
  yarn add @open-web3/parachain-launch
  print_divider
fi

"$PARCHAIN_LAUNCH_BIN" generate --config="parachain-launch-config-$CHAIN_TYPE.yml" --output="generated-$CHAIN_TYPE" --yes

echo "Done, please check files under $ROOTDIR/docker/generated-$CHAIN_TYPE/"
print_divider
