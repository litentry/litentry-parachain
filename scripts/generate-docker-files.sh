#!/usr/bin/env bash
set -eo pipefail

function usage() {
  echo "Usage: $0 litentry|litmus|rococo"
}

[ $# -ne 1 ] && (usage; exit 1)

function print_divider() {
  echo "------------------------------------------------------------"
}

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR/docker"

CHAIN=$1
CONFIG=$CHAIN-parachain-launch-config.yml
OUTDIR=generated-$CHAIN

print_divider

echo "installing parachain-launch ..."
corepack yarn install
print_divider

# pull the polkadot image to make sure we are using the latest
# litentry-parachain image is left as it is, since it could be freshly built
POLKADOT_IMAGE=$(grep 'parity/polkadot' "$CONFIG" | sed 's/.*image: //')
echo "pulling $POLKADOT_IMAGE ..."
docker pull -q "$POLKADOT_IMAGE"

print_divider

corepack yarn start --version
corepack yarn start generate --config="$CONFIG" --output="$OUTDIR" --yes

echo "Done, please check files under $ROOTDIR/docker/$OUTDIR/"
print_divider
