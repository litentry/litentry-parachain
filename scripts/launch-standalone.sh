#!/usr/bin/env bash

# This scripts starts a standalone node without relaychain network locally, with the parachain runtime

set -eo pipefail

ROOTDIR=$(git rev-parse --show-toplevel)
PARACHAIN_BIN="$ROOTDIR/target/release/litentry-collator"

cd "$ROOTDIR"

if [ ! -f "$PARACHAIN_BIN" ]; then
  echo "no litentry-collator found, build it now ..."
  make build-node
fi

if ! "$PARACHAIN_BIN" --version &> /dev/null; then
  echo "Cannot execute $PARACHAIN_BIN, wrong executable?"
  exit 1
fi

echo "Starting litentry-collator in standalone mode ..."

$PARACHAIN_BIN --dev --unsafe-ws-external --unsafe-rpc-external
