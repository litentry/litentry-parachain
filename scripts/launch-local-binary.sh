#!/usr/bin/env bash

# This scripts starts a local network with 2 relaychain nodes + 1 parachain node.
# The binaries are passed as arguments for this script.
#
# mainly used on CI-runner, where:
# - The `polkadot` binary will be downloaded directly from official release.
# - The `litentry-collator` binary will be copied out from the litentry/litentry-parachain:latest image.
#
# To use this script locally, you might have to first compile the binaries that can run on your OS.

function usage() {
  echo
  echo "Usage:   $0 [path-to-polkadot-bin] [path-to-litentry-collator]"
  echo "Default: polkadot bin from the latest official release"
  echo "         litentry-collator bin from litentry/litentry-parachain:latest image"
  echo "         both are of Linux verion"
}

if [ "$#" -gt 2 ] ; then
  usage
  exit 1
fi

POLKADOT_BIN="$1"
PARACHAIN_BIN="$2"

TMPDIR=/tmp/parachain_dev
[ -d "$TMPDIR" ] || mkdir -p "$TMPDIR"
ROOTDIR=$(git rev-parse --show-toplevel)

cd "$ROOTDIR"
PARACHAIN_ID=$(grep DEFAULT_PARA_ID node/src/chain_spec.rs  | grep u32 | sed 's/.* = //;s/\;//')

function print_divider() {
  echo "------------------------------------------------------------"
}

print_divider

if [ -z "$POLKADOT_BIN" ]; then
  echo "no polkadot binary provided, download now ..."
  # TODO: find a way to get stable download link
  # https://api.github.com/repos/paritytech/polkadot/releases/latest is not reliable as 
  # polkadot could publish release which has no binary
  url="https://github.com/paritytech/polkadot/releases/download/v0.9.13/polkadot"
  POLKADOT_BIN="$TMPDIR/polkadot"
  wget -O "$POLKADOT_BIN" -q "$url"
  chmod a+x "$POLKADOT_BIN"
fi

if [ ! -s "$POLKADOT_BIN" ]; then
  echo "$POLKADOT_BIN is 0 bytes, download URL: $url"
  exit 1
fi

if ! "$POLKADOT_BIN" --version &> /dev/null; then
  echo "Cannot execute $POLKADOT_BIN, wrong executable?"
  usage
  exit 1
fi

if [ -z "$PARACHAIN_BIN" ]; then
  echo "no litentry-collator binary provided, build it now ..."
  make build-node
  PARACHAIN_BIN="$ROOTDIR/target/release/litentry-collator"
  chmod a+x "$PARACHAIN_BIN"
fi

if ! "$PARACHAIN_BIN" --version &> /dev/null; then
  echo "Cannot execute $PARACHAIN_BIN, wrong executable?"
  usage
  exit 1
fi

cd "$TMPDIR"

echo "starting dev network with binaries ..."

# generate chain spec
ROCOCO_CHAINSPEC=rococo-local-chain-spec.json
$POLKADOT_BIN build-spec --chain rococo-local --disable-default-bootnode --raw > $ROCOCO_CHAINSPEC

# generate genesis state and wasm for registration
$PARACHAIN_BIN export-genesis-state --chain dev > para-$PARACHAIN_ID-genesis
$PARACHAIN_BIN export-genesis-wasm --chain dev > para-$PARACHAIN_ID-wasm

# run alice and bob as relay nodes
$POLKADOT_BIN --chain $ROCOCO_CHAINSPEC --alice --tmp --port 30333 --ws-port 9944 --rpc-port 9933 &> "relay.alice.log" &
sleep 10

$POLKADOT_BIN --chain $ROCOCO_CHAINSPEC --bob --tmp --port 30334 --ws-port 9945  --rpc-port 9934 &> "relay.bob.log" &
sleep 10

# run a litentry-collator instance
$PARACHAIN_BIN --alice --collator --force-authoring --tmp --chain dev \
  --port 30335 --ws-port 9946 --rpc-port 9935 --execution wasm \
  -- \
  --execution wasm --chain $ROCOCO_CHAINSPEC --port 30332 --ws-port 9943 --rpc-port 9932 &> "para.alice.log" &
sleep 10

echo "register parachain now ..."
cd "$ROOTDIR/ts-tests"
echo "NODE_ENV=ci" > .env
yarn
yarn register-parachain 2>&1 | tee "$TMPDIR/register-parachain.log"
print_divider

echo "done. please check $TMPDIR for generated files if need"
print_divider
