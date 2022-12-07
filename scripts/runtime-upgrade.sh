#!/bin/bash
# set -eo we don't allow any command failed in this script.
set -eo pipefail


# the script is used to simulation runtime upgrade,See issue_378 for details
# https://github.com/litentry/litentry-parachain/issues/378

# pre-knowledge:
# Get the latest snapshot of the blockchain and export it. Start the chain locally with the obtained snapshot,
# and then run the runtime-upgrade program to check the consistency of the status before and after the upgrade.
# If the upgrade is successful, the upgrade is successful.


# This script should do:
# 1.new runtime.wasm is already?
# 2.upload runtime.wasm
# 3.update successful

function usage() {
  echo
  echo "Usage:   $0 litentry|litmus|rococo will download runtime.wasm"
  echo "         both are of Linux verion"
}

CHAIN_TYPE=${1:-rococo}


case "$CHAIN_TYPE" in
  rococo)
    CHAIN_TYPE=rococo
  ;;
  litmus)
    CHAIN_TYPE=litmus
    ;;
  litentry)
    CHAIN_TYPE=litentry
    ;;
  *)
    echo "unsupported chain type"
    exit 1 ;;
esac

echo "$CHAIN_TYPE"

function print_divider() {
  echo "------------------------------------------------------------"
}

function download_new_wasm() {
    echo "will download $CHAIN_TYPE runtime.wasm please wait a later ~~"
    #https://github.com/litentry/litentry-parachain/releases/download/v0.9.13/rococo-parachain-runtime.compact.compressed.wasm
    url="https://github.com/litentry/litentry-parachain/releases/download/v0.9.13/$CHAIN_TYPE-parachain-runtime.compact.compressed.wasm"

    echo "$url"
    #NEW_WASM="$(pwd)/docker/$CHAIN_TYPE-parachain-runtime.compact.compressed.wasm"
    cd "$(pwd)/docker"
    wget -q "$url"
    echo "right is download successful!"

}

download_new_wasm


#2. upload runtime.wasm  reference ts-test  register-parachain.ts
echo "register parachain now ..."
cd "$ROOTDIR/ts-tests"
echo "NODE_ENV=ci" > .env
yarn
yarn register-parachain 2>&1 | tee "$TMPDIR/register-parachain.log"
print_divider

















