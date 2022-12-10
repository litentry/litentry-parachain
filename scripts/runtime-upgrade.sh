#!/bin/bash
# set -eo we don't allow any command failed in this script.
set -eo pipefail

ROOTDIR=$(git rev-parse --show-toplevel)

# the script is used to simulate runtime upgrade, see:
# https://github.com/litentry/litentry-parachain/issues/378

# The latest state of the blockchain is scraped and used to bootstrap a chain locally via fork-off-substrate,
# see ./scripts/fork-parachain-and-launch.sh
#
# After that, this script:
# 1. get the runtime wasm
# 2. do runtime upgrade using wasm from step 1
# 3. verify if the runtime upgrade is successful

output_wasm=/tmp/runtime.wasm

function usage() {
  echo
  echo "Usage: $0 wasm-path"
  echo "       wasm-path can be either local file path or https URL"
}

[ $# -ne 1 ] && (usage; exit 1)

function print_divider() {
  echo "------------------------------------------------------------"
}

print_divider

# download runtime wasm
echo "Get runtime wasm from $1"
case "$1" in
  https*)
    wget -q "$1" -O "$output_wasm" ;;
  *)
    cp -f "$1" "$output_wasm" ;;
esac

echo "Done"

if [ -f "$output_wasm" ]; then
  ls -l "$output_wasm"
else
  echo "Cannot find $output_wasm, quit"
  exit 1
fi

print_divider

# 2. do runtime upgrade and verify
echo "Do runtime upgrade and verify ..."
cd "$ROOTDIR/ts-tests"
echo "NODE_ENV=ci" > .env
yarn && yarn test-runtime-upgrade 2>&1

print_divider

echo "Done"