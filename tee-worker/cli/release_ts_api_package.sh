#!/bin/bash

# Copyright 2020-2023 Trust Computing GmbH.

set -euo pipefail
while getopts ":m:p:P:t:u:V:C:" opt; do
    case $opt in
        t)
            TEST=$OPTARG
            ;;
        m)
            READMRENCLAVE=$OPTARG
            ;;
        p)
            NPORT=$OPTARG
            ;;
        P)
            WORKER1PORT=$OPTARG
            ;;
        u)
            NODEURL=$OPTARG
            ;;
        V)
            WORKER1URL=$OPTARG
            ;;
        C)
            CLIENT_BIN=$OPTARG
            ;;
    esac
done

# Using default port if none given as arguments.
NPORT=${NPORT:-9944}
NODEURL=${NODEURL:-"ws://127.0.0.1"}

WORKER1PORT=${WORKER1PORT:-2000}
WORKER1URL=${WORKER1URL:-"wss://127.0.0.1"}

CLIENT_BIN=${CLIENT_BIN:-"./../bin/litentry-cli"}

echo "Using client binary ${CLIENT_BIN}"
echo "Using node uri ${NODEURL}:${NPORT}"
echo "Using trusted-worker uri ${WORKER1URL}:${WORKER1PORT}"
echo ""
function usage() {
    echo "Usage: $0 <Options>"
    echo "updating metadata"
}

[ $# -ne 1 ] && (usage; exit 1)
apt-get install sudo
sudo apt-get install wget
sudo wget -qO /usr/local/bin/websocat https://github.com/vi/websocat/releases/latest/download/websocat.x86_64-unknown-linux-musl
sudo chmod a+x /usr/local/bin/websocat
websocat --version


cd /client-api
pnpm install

cd /client-api/parachain-api
echo '{"id":1,"jsonrpc":"2.0","method":"state_getMetadata","params":[]}' | /usr/local/bin/websocat -n1 -k -B 99999999 ws://litentry-node:9912
# echo '{"id":1,"jsonrpc":"2.0","method":"state_getMetadata","params":[]}' | /usr/local/bin/websocat -n1 -k -B 99999999 ws://litentry-node:9912 > prepare-build/litentry-parachain-metadata.json

echo "update parachain metadata"

cd  /client-api/sidechain-api
echo '{"id":1,"jsonrpc":"2.0","method":"state_getMetadata","params":[]}' | /usr/local/bin/websocat -n1 -k -B 99999999 wss://litentry-worker-1:2011
# echo '{"id":1,"jsonrpc":"2.0","method":"state_getMetadata","params":[]}' | /usr/local/bin/websocat -n1 -k -B 99999999 wss://litentry-worker-1:2011 > prepare-build/litentry-sidechain-metadata.json
echo "update sidechain metadata"

pnpm run build

