#!/bin/bash

# Copyright 2020-2023 Trust Computing GmbH.

while getopts ":p:A:B:u:W:V:C:" opt; do
    case $opt in
        p)
            NPORT=$OPTARG
            ;;
        A)
            WORKER1PORT=$OPTARG
            ;;
        B)
            WORKER2PORT=$OPTARG
            ;;
        u)
            NODEURL=$OPTARG
            ;;
        V)
            WORKER1URL=$OPTARG
            ;;
        W)
            WORKER2URL=$OPTARG
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

ACC=//Bob
KEY="22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12"

CLIENT="$CLIENT_BIN -p $NPORT -P $WORKER1PORT -u $NODEURL -U $WORKER1URL"
echo "CLIENT is: $CLIENT"

echo "* Query on-chain enclave registry:"
WORKERS=$($CLIENT list-workers)
echo "WORKERS: "
echo "${WORKERS}"
echo ""

if [ "$READMRENCLAVE" = "file" ]
then
    read MRENCLAVE <<< $(cat ~/mrenclave.b58)
    echo "Reading MRENCLAVE from file: ${MRENCLAVE}"
else
    # This will always take the first MRENCLAVE found in the registry !!
    read MRENCLAVE <<< $(echo "$WORKERS" | awk '/  MRENCLAVE: / { print $2; exit }')
    echo "Reading MRENCLAVE from worker list: ${MRENCLAVE}"
fi
[[ -z $MRENCLAVE ]] && { echo "MRENCLAVE is empty. cannot continue" ; exit 1; }


ALICE=//Alice

ALICE_DID='did:litentry:substrate:0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48'
ALICE_IDENTITY='did:litentry:twitter:my_twitter'

BOB_DID='did:litentry:substrate:0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48'
BOB_IDENTITY='did:litentry:evm:0x0D9bFD1f18f5f4FD08247DC54aD3528909c4b3E9'
BOB_IDENTITY_NETWORKS='bsc,ethereum'

sleep 10
echo "* Create $ALICE_DID Identity"
${CLIENT} trusted link-identity "$ALICE_DID" "$ALICE_IDENTITY"

sleep 10
echo "* Create $BOB_DID Identity"
${CLIENT} trusted link-identity "$BOB_DID" "$BOB_IDENTITY" "$BOB_IDENTITY_NETWORKS"

sleep 20
echo "* Get IDGraph stats"
IDGRAPH_STATS=$($CLIENT trusted --mrenclave $MRENCLAVE --direct id-graph-stats $ALICE)
echo "${IDGRAPH_STATS}"
echo ""

read TOTALNUMBER <<< $(echo "$IDGRAPH_STATS" | awk '/Total number: / { print $3; exit }')
echo "TOTALNUMBER: ${TOTALNUMBER}"

if [ "$TOTALNUMBER" = "4" ]; then
    echo "test indirect call passed"
else
    echo "KEY non-identical: expected: "4" actual: $TOTALNUMBER"
    exit 1
fi