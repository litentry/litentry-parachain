#!/bin/bash

# Copyright 2020-2023 Trust Computing GmbH.

set -euo pipefail

function usage() {
    echo "Usage: $0 <Options>"
    echo "updating metadata"
}

[ $# -ne 1 ] && (usage; exit 1)
whereis websocat

cd /client-api
pnpm install

cd /client-api/parachain-api

curl -s -H "Content-Type: application/json" -d '{"id": "1", "jsonrpc": "2.0", "method": "state_getMetadata", "params": []}' http://litentry-node:9912 > prepare-build/litentry-parachain-metadata.json
echo "update parachain metadata"

ls /usr/local/worker-bin

cd  /client-api/sidechain-api

/usr/local/worker-bin/litentry-cli print-sgx-metadata-raw

echo "update sidechain metadata"

# pnpm run build

