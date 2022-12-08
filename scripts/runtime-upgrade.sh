#!/bin/bash
# set -eo we don't allow any command failed in this script.
set -eo pipefail

# the script is used to simulation runtime upgrade,See issue_378 for details
# https://github.com/litentry/litentry-parachain/issues/378

# pre-knowledge:
# Get the latest snapshot of the blockchain and export it. Start the chain locally with the obtained snapshot,
# and then run the runtime-upgrade program to check the consistency of the status before and after the upgrade.

# This script should do:
# 1.new runtime.wasm is already?
# 2.upload runtime.wasm
# 3.update successful

function usage() {
  echo
  echo "Usage:   $0 litentry|litmus|rococo  [release_tag] will download runtime.wasm && runtime upgrade"
  echo "         both are of Linux verion"
}

CHAIN_TYPE=${1:-rococo}
RELEASE_TAG=${2}

function print_divider() {
  echo "------------------------------------------------------------"
}

function download_new_wasm() {
  echo "will download $CHAIN_TYPE runtime.wasm please wait a later ~~"

  url="https://github.com/litentry/litentry-parachain/releases/download/$RELEASE_TAG/$CHAIN_TYPE-parachain-runtime.compact.compressed.wasm"

  echo "$url"
  cd "$(pwd)/docker"
  wget -q "$url"
  mv $CHAIN_TYPE-parachain-runtime.compact.compressed.wasm runtime.compact.compressed.wasm
  echo "right download successful!"
  cd ..
}

download_new_wasm

sleep 10
#2. upload runtime.wasm  reference ts-test  register-parachain.ts
echo "simulation runtime upgrade now ..."
cd "$(pwd)/ts-tests"
echo "NODE_ENV=ci" >.env
yarn
yarn runtime-upgrade 2>&1 | tee "$TMPDIR/runtime-upgrade.log"

print_divider

#3.succeccful
echo "simulation runtime upgrade successful!"
