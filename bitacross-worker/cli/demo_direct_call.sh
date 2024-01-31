#!/bin/bash

# Executes a direct call on a worker and checks the balance afterwards.
#
# setup:
# run all on localhost:
#   litentry-node purge-chain --dev
#   litentry-node --tmp --dev -lruntime=debug
#   rm light_client_db.bin
#   export RUST_LOG=litentry_worker=info,ita_stf=debug
#   bitacross-worker init_shard
#   bitacross-worker shielding-key
#   bitacross-worker signing-key
#   bitacross-worker run
#
# then run this script

# usage:
#  demo_direct_call.sh -p <NODEPORT> -P <WORKERPORT> -t <TEST_BALANCE_RUN> -m file
#
# TEST_BALANCE_RUN is either "first" or "second"
# if -m file is set, the mrenclave will be read from file

while getopts ":m:p:P:t:u:V:C:" opt; do
    case $opt in
        t)
            TEST=$OPTARG
            ;;
        m)
            READ_MRENCLAVE=$OPTARG
            ;;
        p)
            LITENTRY_RPC_PORT=$OPTARG
            ;;
        P)
            WORKER_1_PORT=$OPTARG
            ;;
        u)
            LITENTRY_RPC_URL=$OPTARG
            ;;
        V)
            WORKER_1_URL=$OPTARG
            ;;
        C)
            CLIENT_BIN=$OPTARG
            ;;
        *)
            echo "invalid arg ${OPTARG}"
            exit 1
    esac
done

# Using default port if none given as arguments.
LITENTRY_RPC_PORT=${LITENTRY_RPC_PORT:-9944}
LITENTRY_RPC_URL=${LITENTRY_RPC_URL:-"ws://127.0.0.1"}

WORKER_1_PORT=${WORKER_1_PORT:-2000}
WORKER_1_URL=${WORKER_1_URL:-"wss://127.0.0.1"}

CLIENT_BIN=${CLIENT_BIN:-"./../bin/bitacross-cli"}

echo "Using client binary ${CLIENT_BIN}"
${CLIENT_BIN} --version
echo "Using node uri ${LITENTRY_RPC_URL}:${LITENTRY_RPC_PORT}"
echo "Using trusted-worker uri ${WORKER_1_URL}:${WORKER_1_PORT}"
echo ""


AMOUNTSHIELD=50000000000
AMOUNTTRANSFER=40000000000

CLIENT="${CLIENT_BIN} -p ${LITENTRY_RPC_PORT} -P ${WORKER_1_PORT} -u ${LITENTRY_RPC_URL} -U ${WORKER_1_URL}"
read -r MRENCLAVE <<< "$($CLIENT list-workers | awk '/  MRENCLAVE: / { print $2; exit }')"

echo ""
echo "* Create a new incognito account for Alice"
ICGACCOUNTALICE=//AliceIncognito
ICGACCOUNTALICE_PUBKEY=0x50503350955afe8a107d6f115dc253eb5d75a3fe37a90b373db26cc12e3c6661
echo "  Alice's incognito account = ${ICGACCOUNTALICE}"
echo ""

echo "* Create a new incognito account for Bob"
ICGACCOUNTBOB=//BobIncognito
ICGACCOUNTBOB_PUBKEY=0xc24c5b3969d8ec4ca8a655a98dcc136d5d4c29d1206ffe7721e80ebdfa1d0b77
echo "  Bob's incognito account = ${ICGACCOUNTBOB}"
echo ""

echo "* Issue ${AMOUNTSHIELD} tokens to Alice's incognito account"
${CLIENT} trusted --mrenclave ${MRENCLAVE} --direct set-balance ${ICGACCOUNTALICE} ${AMOUNTSHIELD}
echo ""

echo "Get balance of Alice's incognito account"
${CLIENT} trusted --mrenclave ${MRENCLAVE} balance ${ICGACCOUNTALICE}
echo ""

# Send funds from Alice to Bob's account.
echo "* Send ${AMOUNTTRANSFER} funds from Alice's incognito account to Bob's incognito account"
$CLIENT trusted --mrenclave ${MRENCLAVE} --direct transfer ${ICGACCOUNTALICE} ${ICGACCOUNTBOB} ${AMOUNTTRANSFER}
echo ""

# Prevent getter being executed too early and returning an outdated result, before the transfer was made.
echo "* Waiting 6 seconds"
sleep 6
echo ""

echo "* Get balance of Alice's incognito account"
# RESULT=$(${CLIENT} trusted --mrenclave ${MRENCLAVE} balance ${ICGACCOUNTALICE} | xargs)
RESULT=$(${CLIENT} trusted --mrenclave ${MRENCLAVE} get-storage System Account ${ICGACCOUNTALICE_PUBKEY} | jq ".data.free" | xargs)
echo $RESULT
echo ""

echo "* Bob's incognito account balance"
# RESULT=$(${CLIENT} trusted --mrenclave ${MRENCLAVE} balance ${ICGACCOUNTBOB} | xargs)
RESULT=$(${CLIENT} trusted --mrenclave ${MRENCLAVE} get-storage System Account ${ICGACCOUNTBOB_PUBKEY} | jq ".data.free" | xargs)
echo $RESULT
echo ""


# The following tests are for automated CI.
# They only work if you're running from fresh genesis.
case $TEST in
    first)
        if [ "40000000000" = "$RESULT" ]; then
            echo "test passed (1st time)"
            echo ""
            exit 0
        else
            echo "test ran through but balance is wrong. have you run the script from fresh genesis?"
            exit 1
        fi
        ;;
    second)
        if [ "80000000000" = "$RESULT" ]; then
            echo "test passed (2nd time)"
            echo ""
            exit 0
        else
            echo "test ran through but balance is wrong. is this really the second time you run this since genesis?"
            exit 1
        fi
        ;;
esac

exit 0
