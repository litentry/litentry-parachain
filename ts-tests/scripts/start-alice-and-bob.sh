#!/bin/sh

# configure the relative paths of binary and chain spec
GIT_ROOT=`git rev-parse --show-toplevel`
LITENTRY_BIN=$GIT_ROOT/target/release/rococo-collator
POLKADOT_BIN=$GIT_ROOT/polkadot/target/release/polkadot
ROCOCO_CHAINSPEC=$GIT_ROOT/polkadot/rococo-local-cfde-real-overseer-new.json

# generate genesis head and wasm validation files
$LITENTRY_BIN export-genesis-state --parachain-id 1984 > para-1984-genesis
$LITENTRY_BIN export-genesis-wasm > para-1984-wasm

# run alice and bob as relay nodes
$POLKADOT_BIN --chain $ROCOCO_CHAINSPEC --alice --tmp --port 30333 &
$POLKADOT_BIN --chain $ROCOCO_CHAINSPEC --bob --tmp --port 30334 &

