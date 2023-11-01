#!/bin/bash

# Copyright 2020-2023 Trust Computing GmbH.

while getopts ":p:A:u:V:C:T:" opt; do
    case $opt in
        p)
            NPORT=$OPTARG
            ;;
        A)
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
        T)
            INTEGRATION_TEST=$OPTARG
            ;;
    esac
done

# Using default port if none given as arguments.
NPORT=${NPORT:-9944}
NODEURL=${NODEURL:-"ws://127.0.0.1"}

WORKER1PORT=${WORKER1PORT:-2000}
WORKER1URL=${WORKER1URL:-"wss://127.0.0.1"}

CLIENT_BIN=${CLIENT_BIN:-"./../bin/litentry-cli"}

CLIENT="${CLIENT_BIN} -p ${NPORT} -P ${WORKER1PORT} -u ${NODEURL} -U ${WORKER1URL}"

echo "Using client binary $CLIENT_BIN"
echo "Using node uri $NODEURL:$NPORT"
echo "Using trusted-worker uri $WORKER1URL:$WORKER1PORT"
echo "Start testing $INTEGRATION_TEST"
echo ""

apt-get install sudo
sudo apt-get install wget
sudo wget -qO /usr/local/bin/websocat https://github.com/vi/websocat/releases/latest/download/websocat.x86_64-unknown-linux-musl
sudo chmod a+x /usr/local/bin/websocat
websocat --version

cd /client-api/parachain-api
echo '{"id":1,"jsonrpc":"2.0","method":"state_getMetadata","params":[]}' | /usr/local/bin/websocat -n1 -k -B 99999999 $NODEURL:$NPORT > prepare-build/litentry-parachain-metadata.json
echo "update parachain metadata"

cd  /client-api/sidechain-api
${CLIENT} print-sgx-metadata-raw
${CLIENT} print-sgx-metadata-raw > prepare-build/litentry-sidechain-metadata.json
echo "update sidechain metadata"

cd /client-api
pnpm install
pnpm run build



cd /ts-tests

echo "Testing $INTEGRATION_TEST"
pnpm install
pnpm --filter integration-tests run $INTEGRATION_TEST:staging