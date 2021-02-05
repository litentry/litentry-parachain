#!/bin/sh

# configure the relative paths of binary and chain spec
GIT_ROOT=`git rev-parse --show-toplevel`
POLKADOT_BIN=$GIT_ROOT/polkadot/target/release/polkadot
ROCOCO_CHAINSPEC=$GIT_ROOT/polkadot/rococo-local-cfde-real-overseer-new.json

# run alice and bob as relay nodes
$POLKADOT_BIN --chain $ROCOCO_CHAINSPEC --alice --tmp &
$POLKADOT_BIN --chain $ROCOCO_CHAINSPEC --bob --tmp --port 30334 &
