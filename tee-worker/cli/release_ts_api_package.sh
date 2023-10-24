#!/bin/bash

# Copyright 2020-2023 Trust Computing GmbH.

set -euo pipefail

function usage() {
    echo "Usage: $0 <Options>"
    echo "updating metadata"
}

[ $# -ne 1 ] && (usage; exit 1)
apt-get install sudo
sudo apt-get install wget
sudo wget -qO /usr/local/bin/websocat https://github.com/vi/websocat/releases/latest/download/websocat.x86_64-unknown-linux-musl
sudo chmod a+x /usr/local/bin/websocat
whereis websocat
websocat --version

echo '{"id":1,"jsonrpc":"2.0","method":"state_getMetadata","params":[]}' | /usr/local/bin/websocat -n1 -k -B 99999999 wss://litentry-worker-1:2011

cd /client-api
pnpm install

cd /client-api/parachain-api

curl -s -H "Content-Type: application/json" -d '{"id": "1", "jsonrpc": "2.0", "method": "state_getMetadata", "params": []}' http://litentry-node:9912 > prepare-build/litentry-parachain-metadata.json
echo "update parachain metadata"

cd  /client-api/sidechain-api


echo "update sidechain metadata"

# pnpm run build

