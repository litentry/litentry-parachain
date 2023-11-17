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

if [ ! -z "$GH_TOKEN" ]; then
  auth_header="Authorization: token $GH_TOKEN"
fi

# 1. download runtime wasm
echo "Download runtime wasm from $1 ..."
case "$1" in
  https*)
    wget --header="$auth_header" --header="Accept: application/octet-stream" -d "$1" -O "$output_wasm" ;;
  *)
    cp -f "$1" "$output_wasm" ;;
esac

echo "Done"

if [ -f "$output_wasm" ] && [ -s "$output_wasm" ]; then
  ls -l "$output_wasm"
else
  echo "Cannot find $output_wasm or it has 0 bytes, quit"
  exit 0
fi

print_divider

# 2. check if the released runtime version is greater than the on-chain runtime version,
#    which should be now accessible via localhost:9933
# onchain_version=$(curl -s -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "state_getRuntimeVersion", "params": [] }' http://localhost:9933 | jq .result.specVersion)
# release_version=$(subwasm --json info "$output_wasm" | jq .core_version.specVersion)

# echo "Check runtime version ..."
# echo "On-chain: $onchain_version"
# echo "Release:  $release_version"

# if [ -n "$release_version" ] && \
#    [ -n "$onchain_version" ] && \
#    [ "$onchain_version" -ge "$release_version" ]; then
#   echo "Runtime version not increased, quit"
#   exit 0
# fi

# print_divider

# # 3. do runtime upgrade and verify
# echo "Do runtime upgrade and verify ..."
# cd "$ROOTDIR/ts-tests"
# echo "NODE_ENV=ci" > .env
# pnpm install && pnpm run test-runtime-upgrade 2>&1

# print_divider

# echo "Done"