#!/bin/sh

# configure the relative paths of binary
PARACHAIN_ID=2022
GIT_ROOT=`git rev-parse --show-toplevel`
LITENTRY_BIN=$GIT_ROOT/target/release/litentry-collator
POLKADOT_BIN=$GIT_ROOT/polkadot/target/release/polkadot

# generate chain spec
ROCOCO_CHAINSPEC=rococo-local-chain-spec.json
$POLKADOT_BIN build-spec --chain rococo-local --disable-default-bootnode --raw > $ROCOCO_CHAINSPEC

# generate genesis head and wasm validation files
$LITENTRY_BIN export-genesis-state --parachain-id $PARACHAIN_ID > para-$PARACHAIN_ID-genesis
$LITENTRY_BIN export-genesis-wasm > para-$PARACHAIN_ID-wasm

# run alice and bob as relay nodes
$POLKADOT_BIN --chain $ROCOCO_CHAINSPEC --alice --tmp --port 30333 --ws-port 9944 &> "$1/relay.alice.log" &
sleep 3
$POLKADOT_BIN --chain $ROCOCO_CHAINSPEC --bob --tmp --port 30334 --ws-port 9945  &> "$1/relay.bob.log" &
sleep 3

# run a second litentry-collator instance
$LITENTRY_BIN --collator --tmp --parachain-id $PARACHAIN_ID --port 40333 --ws-port 9954 --alice -- --execution native --chain $ROCOCO_CHAINSPEC --port 30344 --ws-port 9946 &> "$1/para.alice.log" &
sleep 3
$LITENTRY_BIN --collator --tmp --parachain-id $PARACHAIN_ID --port 40334 --ws-port 9955 --bob -- --execution native --chain $ROCOCO_CHAINSPEC --port 30344 --ws-port 9947 &> "$1/para.bob.log" &
