#!/bin/bash

# Copyright 2020-2024 Trust Computing GmbH.

set -euo pipefail

while getopts ":p:A:u:W:V:C:" opt; do
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
        W)
            NODEHTTPURL=$OPTARG
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
NODEURL=${NODEURL:-"ws://localhost"}
NODEHTTPURL=${NODEHTTPURL:-"http://localhost"}
WORKER1PORT=${WORKER1PORT:-2011}
WORKER1URL=${WORKER1URL:-"ws://bitacross-worker-1"}

CLIENT_BIN=${CLIENT_BIN:-"/usr/local/bin/bitacross-cli"}

CLIENT="${CLIENT_BIN} -p ${NPORT} -P ${WORKER1PORT} -u ${NODEURL} -U ${WORKER1URL}"

function usage() {
    echo ""
    echo "This is a script for bitacross-worker integration ts tests. Pass test name as first argument"
    echo ""
}

[ $# -ne 1 ] && (usage; exit 1)
TEST=$1

echo "Using client binary $CLIENT_BIN"
echo "Using node uri $NODEURL:$NPORT"
echo "Using trusted-worker uri $WORKER1URL:$WORKER1PORT"
echo "Using node http uri $NODEHTTPURL:$NPORT"
echo ""

cd /ts-tests
pnpm install

NODE_ENV=staging pnpm --filter integration-tests run test $TEST
