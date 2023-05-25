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
        *)
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

CLIENT="$CLIENT_BIN -p $NPORT -P $WORKER1PORT -u $NODEURL -U $WORKER1URL"
echo "CLIENT is $CLIENT"

echo "* Query on-chain enclave registry:"
WORKERS=$($CLIENT list-workers)
echo "WORKERS: "
echo "${WORKERS}"
echo ""

if [ "$READMRENCLAVE" = "file" ]
then
    read -r MRENCLAVE <<< "$(cat ~/mrenclave.b58)"
    echo "Reading MRENCLAVE from file: ${MRENCLAVE}"
else
    # This will always take the first MRENCLAVE found in the registry !!
    read -r MRENCLAVE <<< "$(echo "$WORKERS" | awk '/  MRENCLAVE: / { print $2; exit }')"
    echo "Reading MRENCLAVE from worker list: ${MRENCLAVE}"
fi
[[ -z $MRENCLAVE ]] && { echo "MRENCLAVE is empty. cannot continue" ; exit 1; }

ALICE=//Alice

echo "Get nonce of Alice: "
RESULT=$(${CLIENT} get-account-nonce ${ALICE} "${MRENCLAVE}")
read -r ALICE_NONCE_1 <<< "$(echo "$RESULT" | awk '/ nonce: / { print $3; exit }')"

echo ""
echo "* Create a new incognito account for Alice"
ICGACCOUNTALICE=//AliceIncognito
echo "  Alice's incognito account = ${ICGACCOUNTALICE}"
AMOUNTSHIELD=10000000000
echo ""

echo "* Issue ${AMOUNTSHIELD} tokens to Alice's incognito account"
${CLIENT} trusted --mrenclave "${MRENCLAVE}" --direct set-balance ${ICGACCOUNTALICE} ${AMOUNTSHIELD}
echo ""

echo "Get nonce of Alice after set-balance: "
RESULT=$(${CLIENT} get-account-nonce ${ALICE} "${MRENCLAVE}")
read -r ALICE_NONCE_2 <<< "$(echo "$RESULT" | awk '/ nonce: / { print $3; exit }')"
echo ""

EXPECTED_NONCE=$((ALICE_NONCE_1 + 1))
ACTUAL_NONCE=$ALICE_NONCE_2

if [  $EXPECTED_NONCE -eq "$ACTUAL_NONCE" ]; then
    echo "test passed"
    exit 0
else
    echo "KEY non-identical: expected: $EXPECTED_NONCE actual: $ACTUAL_NONCE"
    exit 1
fi
