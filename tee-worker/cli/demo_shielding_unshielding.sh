#!/bin/bash

# to make sure the script aborts when (sub-)function exits abnormally
set -e

# Demonstrates how to shield tokens from the parentchain into the sidechain.
#
# setup:
# run all on localhost:
#   integritee-node purge-chain --dev
#   integritee-node --dev -lruntime=debug
#   rm light_client_db.bin
#   export RUST_LOG=integritee_service=info,ita_stf=debug
#   integritee-service init_shard
#   integritee-service shielding-key
#   integritee-service signing-key
#   integritee-service run
#
# then run this script

# usage:
#  demo_shielding_unshielding.sh -p <NODEPORT> -P <WORKERPORT> -t <TEST_BALANCE_RUN> -m file
#
# TEST_BALANCE_RUN is either "first" or "second"
# if -m file is set, the mrenclave will be read from file

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

CLIENT_BIN=${CLIENT_BIN:-"./../bin/integritee-cli"}

echo "Using client binary ${CLIENT_BIN}"
echo "Using node uri ${NODEURL}:${NPORT}"
echo "Using trusted-worker uri ${WORKER1URL}:${WORKER1PORT}"
echo ""

# the parachain LIT is 12 decimal
UNIT=$(( 10 ** 12 ))

# we have to make these amounts greater than ED, see
# https://github.com/litentry/litentry-parachain/issues/1162
AMOUNT_SHIELD=$(( 6 * UNIT ))
AMOUNT_TRANSFER=$(( 2 * UNIT ))
AMOUNT_UNSHIELD=$(( 1 * UNIT ))

CLIENT="${CLIENT_BIN} -p ${NPORT} -P ${WORKER1PORT} -u ${NODEURL} -U ${WORKER1URL}"

# interval and max rounds to wait to check the given account balance in sidechain
WAIT_INTERVAL_SECONDS=10
WAIT_ROUNDS=20

# Wait until the given account's balance is equal to expected,
# with timeout WAIT_INTERVAL_SECONDS * WAIT_ROUNDS
# usage:
#   wait_until_balance <mrenclave> <account> <expected-balance>
function wait_until_balance()
{
    for i in $(seq 1 $WAIT_ROUNDS); do
        sleep $WAIT_INTERVAL_SECONDS
        balance=$(${CLIENT} trusted --mrenclave "$1" balance "$2")
        if [ $balance -eq "$3" ]; then
            return
        else
            :
        fi
    done
    echo "Assert $2 balance failed, expected = $3, actual = $balance"
    exit 1
}

echo "* Query on-chain enclave registry:"
${CLIENT} list-workers
echo ""

if [ "$READMRENCLAVE" = "file" ]
then
    read MRENCLAVE <<< $(cat ~/mrenclave.b58)
    echo "Reading MRENCLAVE from file: ${MRENCLAVE}"
else
    # this will always take the first MRENCLAVE found in the registry !!
    read MRENCLAVE <<< $($CLIENT list-workers | awk '/  MRENCLAVE: / { print $2; exit }')
    echo "Reading MRENCLAVE from worker list: ${MRENCLAVE}"
fi
[[ -z $MRENCLAVE ]] && { echo "MRENCLAVE is empty. cannot continue" ; exit 1; }

echo "* Get balance of Alice's on-chain account"
${CLIENT} balance "//Alice"
echo ""

echo "* Get balance of Bob's on-chain account"
${CLIENT} balance "//Bob"
echo ""

echo "* Create a new incognito account for Alice"
ICGACCOUNTALICE=//AliceIncognito
echo "  Alice's incognito account = ${ICGACCOUNTALICE}"
echo ""

echo "* Create a new incognito account for Bob"
ICGACCOUNTBOB=$(${CLIENT} trusted --mrenclave ${MRENCLAVE} new-account)
echo "  Bob's incognito account = ${ICGACCOUNTBOB}"
echo ""

# Asssert the initial balance of Alice incognito
# The initial balance of Bob incognito should always be 0, as Bob is newly created
BALANCE_INCOGNITO_ALICE=0
case $TEST in
    first)
        wait_until_balance ${MRENCLAVE} ${ICGACCOUNTALICE} 0 ;;
    second)
        wait_until_balance ${MRENCLAVE} ${ICGACCOUNTALICE} $(( AMOUNT_SHIELD - AMOUNT_TRANSFER + AMOUNT_UNSHIELD ))
        BALANCE_INCOGNITO_ALICE=$(( AMOUNT_SHIELD - AMOUNT_TRANSFER + AMOUNT_UNSHIELD )) ;;
    *)
        echo "unsupported test mode"
        exit 1 ;;
esac

echo "* Shield ${AMOUNT_SHIELD} tokens to Alice's incognito account"
${CLIENT} shield-funds //Alice ${ICGACCOUNTALICE} ${AMOUNT_SHIELD} ${MRENCLAVE}
echo ""

echo "* Wait and assert Alice's incognito account balance"
wait_until_balance ${MRENCLAVE} ${ICGACCOUNTALICE} $(( BALANCE_INCOGNITO_ALICE + AMOUNT_SHIELD ))

echo "* Wait and assert Bob's incognito account balance"
wait_until_balance ${MRENCLAVE} ${ICGACCOUNTBOB} 0

echo "* Send ${AMOUNT_TRANSFER} funds from Alice's incognito account to Bob's incognito account"
$CLIENT trusted --mrenclave ${MRENCLAVE} transfer ${ICGACCOUNTALICE} ${ICGACCOUNTBOB} ${AMOUNT_TRANSFER}
echo ""

echo "* Wait and assert Alice's incognito account balance"
wait_until_balance ${MRENCLAVE} ${ICGACCOUNTALICE} $(( BALANCE_INCOGNITO_ALICE + AMOUNT_SHIELD - AMOUNT_TRANSFER ))

echo "* Wait and assert Bob's incognito account balance"
wait_until_balance ${MRENCLAVE} ${ICGACCOUNTBOB} ${AMOUNT_TRANSFER}

echo "* Un-shield ${AMOUNT_UNSHIELD} tokens from Alice's incognito account"
${CLIENT} trusted --mrenclave ${MRENCLAVE} --xt-signer //Alice unshield-funds ${ICGACCOUNTALICE} //Alice ${AMOUNT_UNSHIELD}
echo ""

echo "* Wait and assert Alice's incognito account balance"
wait_until_balance ${MRENCLAVE} ${ICGACCOUNTALICE} $(( BALANCE_INCOGNITO_ALICE + AMOUNT_SHIELD - AMOUNT_TRANSFER - AMOUNT_UNSHIELD ))

echo "* Wait and assert Bob's incognito account balance"
wait_until_balance ${MRENCLAVE} ${ICGACCOUNTBOB} ${AMOUNT_TRANSFER}

echo "The $TEST test passed!"