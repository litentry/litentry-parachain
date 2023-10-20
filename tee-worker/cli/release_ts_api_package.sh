#!/bin/bash

# Copyright 2020-2023 Trust Computing GmbH.

set -euo pipefail

function usage() {
    echo "Usage: $0 <Options>"
    echo "updating metadata"
}

[ $# -ne 1 ] && (usage; exit 1)
TEST=$1

cd /client-api
pnpm install

cd /client-api/parachain-api
curl -s -H \"Content-Type: application/json\" -d '{\"id\":\"1\", \"jsonrpc\":\"2.0\", \"method\": \"state_getMetadata\", \"params\":[]}' http://litentry-node:9912 > prepare-build/litentry-parachain-metadata.json
echo "update parachain metadata"

cd /
ls
/bin/litentry-cli print-sgx-metadata-raw

cd /client-api/sidechain-api
/bin/litentry-cli print-sgx-metadata-raw > prepare-build/litentry-sidechain-metadata.json

echo "update sidechain metadata"

git status
git diff
# pnpm run build

