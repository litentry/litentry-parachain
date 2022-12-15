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
  echo "Usage:   $0 litentry|litmus|rococo [path-to-polkadot-bin] [path-to-litentry-collator]"
  echo "Default: polkadot bin from the latest official release"
  echo "         litentry-collator bin from litentry/litentry-parachain:latest image"
  echo "         both are of Linux verion"
}

[ $# -lt 1 ] && (usage; exit 1)

CHAIN=$1
POLKADOT_BIN="$2"
PARACHAIN_BIN="$3"

TMPDIR=/tmp/parachain_dev
[ -d "$TMPDIR" ] || mkdir -p "$TMPDIR"
ROOTDIR=$(git rev-parse --show-toplevel)

cd "$ROOTDIR"
PARACHAIN_ID=$(grep DEFAULT_PARA_ID node/src/chain_specs/$CHAIN.rs  | grep u32 | sed 's/.* = //;s/\;//')
export PARACHAIN_ID

function print_divider() {
  echo "------------------------------------------------------------"
}

print_divider

if [ -z "$POLKADOT_BIN" ]; then
  echo "no polkadot binary provided, download now ..."
  # TODO: find a way to get stable download link
  # https://api.github.com/repos/paritytech/polkadot/releases/latest is not reliable as 
  # polkadot could publish release which has no binary
  url="https://github.com/paritytech/polkadot/releases/download/v0.9.32/polkadot"
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
$PARACHAIN_BIN export-genesis-state --chain $CHAIN-dev > genesis-state
$PARACHAIN_BIN export-genesis-wasm --chain $CHAIN-dev > genesis-wasm

# run alice and bob as relay nodes
$POLKADOT_BIN --chain $ROCOCO_CHAINSPEC --alice --tmp --port 30336 --ws-port 9946 --rpc-port 9936 &> "relay.alice.log" &
sleep 10

RELAY_ALICE_IDENTITY=$(grep 'Local node identity' relay.alice.log | sed 's/^.*: //')

$POLKADOT_BIN --chain $ROCOCO_CHAINSPEC --bob --tmp --port 30337 --ws-port 9947  --rpc-port 9937 \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$RELAY_ALICE_IDENTITY &> "relay.bob.log" &
sleep 10

# run a litentry-collator instance
$PARACHAIN_BIN --alice --collator --force-authoring --tmp --chain $CHAIN-dev \
  --port 30333 --ws-port 9944 --rpc-port 9933 --execution wasm \
  -- \
  --execution wasm --chain $ROCOCO_CHAINSPEC --port 30332 --ws-port 9943 --rpc-port 9932 \
  --bootnodes /ip4/127.0.0.1/tcp/30336/p2p/$RELAY_ALICE_IDENTITY &> "para.alice.log" &
sleep 10

echo "register parachain now ..."
cd "$ROOTDIR/ts-tests"
echo "NODE_ENV=ci" > .env
yarn
yarn register-parachain 2>&1 | tee "$TMPDIR/register-parachain.log"
print_divider

echo "done. please check $TMPDIR for generated files if need"

print_divider
