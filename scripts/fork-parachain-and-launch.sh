#!/bin/bash

set -eo pipefail

# this script:
# - scrapes the state of a given parachain using `fork-off-substrate`
# - save the state snapshot to a chain spec JSON
# - use this chain spec to launch a local parachain network

ROOTDIR=$(git rev-parse --show-toplevel)
TMPDIR=$(mktemp -d /tmp/XXXXXX)

FORK_OFF_SUBSTRATE_REPO="https://github.com/litentry/fork-off-substrate.git"

function print_divider() {
  echo "------------------------------------------------------------"
}

function usage() {
  print_divider
  echo "Usage: $0 [ws-rpc-endpoint] [rococo|litmus|litentry] [binary]"
  echo 
  echo "default:"
  echo "ws-rpc-endpoint:   ws://127.0.0.1:9944"
  echo "chain:             rococo"
  echo "binary:            the binary copied from litentry/litentry-parachain:latest"
  print_divider
}

[ $# -gt 3 ] && (usage; exit 1)

case "$1" in
  help|-h|--help)
    usage
    exit 1
    ;;
  *)
    ;;
esac

ENDPOINT="${1:-ws://127.0.0.1:9944}"
ORIG_CHAIN=${2:-rococo}
FORK_CHAIN=${ORIG_CHAIN}-dev

case "$ORIG_CHAIN" in
  litmus|rococo|litentry)
    ;;
  *)
    echo "unsupported chain type"
    exit 1 ;;
esac

echo "TMPDIR is $TMPDIR"
cd "$TMPDIR"
git clone "$FORK_OFF_SUBSTRATE_REPO"
cd fork-off-substrate
git checkout wss-fork
npm i

mkdir data && cd data

# copy the binary
if [ -z "$4" ]; then
  docker cp $(docker create --rm litentry/litentry-parachain:latest):/usr/local/bin/litentry-collator binary
else
  cp "$4" binary
fi
chmod a+x binary

# write .env file
cd ..
cat << EOF > .env
WS_RPC_ENDPOINT=$ENDPOINT
ALICE=1
ORIG_CHAIN=$ORIG_CHAIN
FORK_CHAIN=$FORK_CHAIN
EOF

npm start

if [ ! -f data/fork.json ]; then
  echo "cannot find data/fork.json, please check it manually"
  exit 2
fi

cp -f data/fork.json "$ROOTDIR/docker/"

cd "$ROOTDIR"
sed -i.bak "s;$FORK_CHAIN;fork.json;" docker/$ORIG_CHAIN-parachain-launch-config.yml

# start the network
make launch-docker-$ORIG_CHAIN
