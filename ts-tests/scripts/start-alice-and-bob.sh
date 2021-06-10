#!/bin/sh

# configure the relative paths of binary 
GIT_ROOT=`git rev-parse --show-toplevel`
LITENTRY_BIN=$GIT_ROOT/target/release/litentry-collator
POLKADOT_BIN=$GIT_ROOT/polkadot/target/release/polkadot

# generate chain spec
$POLKADOT_BIN build-spec --chain rococo-local --disable-default-bootnode --raw > rococo-local-cfde-real-overseer.json
ROCOCO_CHAINSPEC=./rococo-local-cfde-real-overseer.json

# generate genesis head and wasm validation files
$LITENTRY_BIN export-genesis-state --parachain-id 1984 > para-1984-genesis
$LITENTRY_BIN export-genesis-wasm > para-1984-wasm

# run alice and bob as relay nodes
$POLKADOT_BIN --chain $ROCOCO_CHAINSPEC --alice --tmp --port 30333 &
$POLKADOT_BIN --chain $ROCOCO_CHAINSPEC --bob --tmp --port 30334 &

# run a second litentry-collator instance 
$LITENTRY_BIN --collator --tmp --parachain-id 1984 --port 40334 --ws-port 9845 --alice -- --execution native --chain $ROCOCO_CHAINSPEC --port 30344 --ws-port 9978 &