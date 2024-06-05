#!/bin/bash

# Copyright 2020-2024 Trust Computing GmbH.

while getopts ":p:A:B:u:W:V:C:" opt; do
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
    esac
done

# Using default port if none given as arguments.
NPORT=${NPORT:-9944}
NODEURL=${NODEURL:-"ws://127.0.0.1"}

WORKER1PORT=${WORKER1PORT:-2000}
WORKER1URL=${WORKER1URL:-"wss://127.0.0.1"}

CLIENT_BIN=${CLIENT_BIN:-"./../bin/litentry-cli"}

echo "Using client binary $CLIENT_BIN"
echo "Using node uri $NODEURL:$NPORT"
echo "Using trusted-worker uri $WORKER1URL:$WORKER1PORT"
echo ""

CLIENT="$CLIENT_BIN -p $NPORT -P $WORKER1PORT -u $NODEURL -U $WORKER1URL"
echo "CLIENT is: $CLIENT"


FIRST_NEW_ACCOUNT=$(${CLIENT} new-account)
echo "New Account created: ${FIRST_NEW_ACCOUNT}" 

echo "Linking identity to Bob"
OUTPUT=$(${CLIENT} link-identity //Bob did:litentry:substrate:${FIRST_NEW_ACCOUNT}) || { echo "Link identity command failed"; exit 1; }
echo "Finished Linking identity to Bob"
sleep 30

echo "Capturing IDGraph Hash of Bob" 
INITIAL_ID_GRAPH_HASH=$(${CLIENT} id-graph-hash did:litentry:substrate:0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48) || { echo "Failed to get ID Graph hash"; exit 1; }
echo "Initial ID Graph Hash of Bob: ${INITIAL_ID_GRAPH_HASH}"

SECOND_NEW_ACCOUNT=$(${CLIENT} new-account)
echo "New Account created: ${SECOND_NEW_ACCOUNT}" 

echo "Linking new identity to Bob with Eve as delegate signer"
OUTPUT=$(${CLIENT} link-identity //Bob "did:litentry:substrate:${SECOND_NEW_ACCOUNT}" -d //Eve) || { echo "Link identity command failed"; exit 1; }
echo "Finished Linking identity to Bob"
sleep 30

echo "Capturing IDGraph Hash of Bob" 
FINAL_ID_GRAPH_HASH=$(${CLIENT} id-graph-hash did:litentry:substrate:0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48) || { echo "Failed to get ID Graph hash"; exit 1; }
echo "Final ID Graph Hash of Bob: ${FINAL_ID_GRAPH_HASH}"

if [ "$INITIAL_ID_GRAPH_HASH" != "$FINAL_ID_GRAPH_HASH" ]; then
    exit 1
else
    echo "Failed Parentchain extrinsic not executed"
fi
