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

echo "installing parachain-launch ..."
yarn add @open-web3/parachain-launch
print_divider

# pull the polkadot image to make sure we are using the latest
# litentry-parachain image is left as it is, since it could be freshly built
POLKADOT_IMAGE=$(grep 'parity/polkadot' "parachain-launch-config-$CHAIN_TYPE.yml" | sed 's/.*image: //')
echo "pulling $POLKADOT_IMAGE ..."
docker pull -q "$POLKADOT_IMAGE"

print_divider

"$PARCHAIN_LAUNCH_BIN" generate --config="parachain-launch-config-$CHAIN_TYPE.yml" --output="generated-$CHAIN_TYPE" --yes

echo "Done, please check files under $ROOTDIR/docker/generated-$CHAIN_TYPE/"
print_divider
