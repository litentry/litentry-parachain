#!/usr/bin/env bash
set -eo pipefail

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR/docker"

CHAIN_TYPE=${1:-dev}

if ! docker image inspect litentry/litentry-parachain:latest &>/dev/null; then
  echo "please build litentry/litentry-parachain:latest first"
  exit 1
fi

if ! parachain-launch --version &>/dev/null; then
  echo "please install parachain-launch first:"
  echo "e.g."
  echo "yarn global add @open-web3/parachain-launch"
  exit 1
fi

parachain-launch generate --config="parachain-launch-config-$CHAIN_TYPE.yml" --output="generated-$CHAIN_TYPE" --yes

cat << EOF
Done, please check files under $ROOTDIR/docker/generated-$CHAIN_TYPE/

To start the network, run:
make launch-local-docker
EOF
