#!/usr/bin/env bash

# This scripts starts a standalone node without relaychain network locally, with the parachain runtime

set -eo pipefail

LITENTRY_PARACHAIN_DIR=${LITENTRY_PARACHAIN_DIR:-"/tmp/parachain_dev"}
[ -d "$LITENTRY_PARACHAIN_DIR" ] || mkdir -p "$LITENTRY_PARACHAIN_DIR"

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

$PARACHAIN_BIN --dev --unsafe-ws-external --unsafe-rpc-external \
  --port "${CollatorPort:-30333}" --ws-port "${CollatorWSPort:-9944}" --rpc-port "${CollatorRPCPort:-9933}" \
  &> "$LITENTRY_PARACHAIN_DIR/para.alice.log" &

echo "Setting skip-schedule-enclave-check flag ..." 
cd "$ROOTDIR/ts-tests"
if [[ -z "${NODE_ENV}" ]]; then
    echo "NODE_ENV=ci" > .env
else
    echo "NODE_ENV=${NODE_ENV}" > .env
fi

sleep 10
