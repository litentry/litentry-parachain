#!/bin/bash

# Copyright 2020-2023 Litentry Technologies GmbH.

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

CLIENT_BIN=${CLIENT_BIN:-"./../bin/integritee-cli"}

echo "Using client binary $CLIENT_BIN"
echo "Using node uri $NODEURL:$NPORT"
echo "Using trusted-worker uri $WORKER1URL:$WORKER1PORT"
echo ""

ACC=//Bob
KEY="22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12"

CLIENT="$CLIENT_BIN -p $NPORT -P $WORKER1PORT -u $NODEURL -U $WORKER1URL"
echo "CLIENT is $CLIENT"

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
ALICE_KEY="8378193a4ce64180814bd60591d1054a04dbc4da02afde453799cd6888ee0c6c"

ALICE_IDENTITY='{"Substrate": {"network": "Polkadot", "address": [58, 215, 18, 154, 11, 223, 105, 185, 64, 123, 200, 233, 215, 156, 158, 88, 152, 249, 7, 217, 84, 219, 188, 4, 18, 50, 246, 243, 96, 91, 215, 11]}}'

BOB=//Bob
BOB_KEY="8378193a4ce64180814bd60591d1054a04dbc4da02afde453799cd6888ee0c6d"

BOB_IDENTITY='{"Substrate": {"network": "Polkadot", "address": [58, 215, 18, 154, 11, 223, 105, 185, 64, 123, 200, 233, 215, 156, 158, 88, 152, 249, 7, 217, 84, 219, 188, 4, 18, 50, 246, 243, 96, 91, 215, 11]}}'

sleep 10
echo "* Set $ALICE 's shielding key to $ALICE_KEY"
${CLIENT} set-user-shielding-key "$ALICE" "$ALICE_KEY" ${MRENCLAVE}
echo ""

sleep 10
echo "* Create $ALICE 's Identity"
${CLIENT} create-identity "$ALICE" "$ALICE_IDENTITY" ${MRENCLAVE}

sleep 10
echo "* Set $BOB 's shielding key to $BOB_KEY"
${CLIENT} set-user-shielding-key "$BOB" "$BOB_KEY" ${MRENCLAVE}
echo ""

sleep 10
echo "* Create $BOB 's Identity"
${CLIENT} create-identity "$BOB" "$BOB_IDENTITY" ${MRENCLAVE}

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