#!/bin/bash

# to make sure the script aborts when (sub-)function exits abnormally
set -e

# Demonstrates how to shield tokens from the parentchain into the sidechain.
#
# setup:
# run all on localhost:
#   litentry-node purge-chain --dev
#   litentry-node --dev -lruntime=debug
#   rm light_client_db.bin
#   export RUST_LOG=litentry_worker=info,ita_stf=debug
#   litentry-worker init_shard
#   litentry-worker shielding-key
#   litentry-worker signing-key
#   litentry-worker run
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

CLIENT_BIN=${CLIENT_BIN:-"./../bin/litentry-cli"}

echo "Using client binary ${CLIENT_BIN}"
${CLIENT_BIN} --version
echo "Using node uri ${LITENTRY_RPC_URL}:${LITENTRY_RPC_PORT}"
echo "Using trusted-worker uri ${WORKER_1_URL}:${WORKER_1_PORT}"
echo ""

# the parentchain token is 12 decimal
UNIT=$(( 10 ** 12 ))
FEE_TOLERANCE=$((10 ** 11))

# we have to make these amounts greater than ED, see
# https://github.com/litentry/litentry-parachain/issues/1162
AMOUNT_SHIELD=$(( 6 * UNIT ))
AMOUNT_TRANSFER=$(( 2 * UNIT ))
AMOUNT_UNSHIELD=$(( 1 * UNIT ))

CLIENT="${CLIENT_BIN} -p ${LITENTRY_RPC_PORT} -P ${WORKER_1_PORT} -u ${LITENTRY_RPC_URL} -U ${WORKER_1_URL}"

# interval and max rounds to wait to check the given account balance in sidechain
WAIT_INTERVAL_SECONDS=10
WAIT_ROUNDS=20

# Do a live query and assert the given account's balance is equal to expected
# usage:
#   assert_account_balance <mrenclave> <account> <expected-balance>
function assert_account_balance()
{
    for i in $(seq 1 $WAIT_ROUNDS); do
        state=$(${CLIENT} trusted --mrenclave "$1" balance "$2")
        if (( $3 >= state ? $3 - state < FEE_TOLERANCE : state - $3 < FEE_TOLERANCE)); then
            return
        else
            sleep $WAIT_INTERVAL_SECONDS
        fi
    done
    echo
    echo "Assert $2 failed, expected = $3, actual = $state, tolerance = $FEE_TOLERANCE"
    exit 1
}


# Do a live query and assert the given account's nonce is equal to expected
# usage:
#   assert_account_nonce <mrenclave> <account> <expected-nonce>
function assert_account_nonce()
{
    for i in $(seq 1 $WAIT_ROUNDS); do
        state=$(${CLIENT} trusted --mrenclave "$1" nonce "$2")
        echo $state
        if [ $state -eq "$3" ]; then
            return
        else
            sleep $WAIT_INTERVAL_SECONDS
        fi
    done
    echo
    echo "Assert $2 failed, expected = $3, actual = $state"
    exit 1
}

# Do a live query and assert the given account's state is equal to expected
# usage:
#   assert_account_state <mrenclave> <account-pub-key> <jq-filter> <expected-state>
function assert_account_state()
{
    state=$(${CLIENT} trusted --mrenclave "$1" get-storage System Account "$2" | jq "$3")
    if [ -z "$state" ]; then
        echo "Query Account $2 $3 failed"
        exit 1
    fi

    if [ $state -eq "$4" ]; then
        return
    fi
    echo
    echo "Assert $2 $3 failed, expected = $4, actual = $state"
    exit 1
}

echo "* Query on-chain enclave registry:"
${CLIENT} list-workers
echo ""

if [ "$READ_MRENCLAVE" = "file" ]
then
    read MRENCLAVE <<< $(cat ~/mrenclave.b58)
    echo "Reading MRENCLAVE from file: ${MRENCLAVE}"
else
    # this will always take the first MRENCLAVE found in the registry !!
    read MRENCLAVE <<< $($CLIENT list-workers | awk '/  MRENCLAVE: / { print $2; exit }')
    echo "Reading MRENCLAVE from worker list: ${MRENCLAVE}"
fi
[[ -z $MRENCLAVE ]] && { echo "MRENCLAVE is empty. cannot continue" ; exit 1; }

echo "* Create a new incognito account for Alice"
ICGACCOUNTALICE=//AliceIncognito
echo "  Alice's incognito account = ${ICGACCOUNTALICE}"
echo ""

# Asssert the initial balance of Alice incognito
# We create different (new) accounts for Bob incognito, hence his initial balance is always 0
BALANCE_INCOGNITO_ALICE=0
case $TEST in
    first)
        assert_account_balance ${MRENCLAVE} ${ICGACCOUNTALICE} 0
        ICGACCOUNTBOB=//BobIncognitoFirst ;;
    second)
        assert_account_balance ${MRENCLAVE} ${ICGACCOUNTALICE} $(( AMOUNT_SHIELD - AMOUNT_TRANSFER - AMOUNT_UNSHIELD ))
        BALANCE_INCOGNITO_ALICE=$(( AMOUNT_SHIELD - AMOUNT_TRANSFER - AMOUNT_UNSHIELD ))
        ICGACCOUNTBOB=//BobIncognitoSecond ;;
    *)
        echo "unsupported test mode"
        exit 1 ;;
esac

echo "* Create a new incognito account for Bob"
echo "  Bob's incognito account = ${ICGACCOUNTBOB}"
echo ""

echo "* Shield ${AMOUNT_SHIELD} tokens to Alice's incognito account"
${CLIENT} shield-funds //Alice ${ICGACCOUNTALICE} ${AMOUNT_SHIELD} ${MRENCLAVE}
echo ""

echo "* Wait and assert Alice's incognito account balance... "
assert_account_balance ${MRENCLAVE} ${ICGACCOUNTALICE} $(( BALANCE_INCOGNITO_ALICE + AMOUNT_SHIELD ))
echo "✔ ok"

echo "* Wait and assert Bob's incognito account balance... "
assert_account_balance ${MRENCLAVE} ${ICGACCOUNTBOB} 0
echo "✔ ok"
echo ""

echo "* Send ${AMOUNT_TRANSFER} funds from Alice's incognito account to Bob's incognito account"
$CLIENT trusted --mrenclave ${MRENCLAVE} transfer ${ICGACCOUNTALICE} ${ICGACCOUNTBOB} ${AMOUNT_TRANSFER}
echo ""

echo "* Wait and assert Alice's incognito account balance... "
assert_account_balance ${MRENCLAVE} ${ICGACCOUNTALICE} $(( BALANCE_INCOGNITO_ALICE + AMOUNT_SHIELD - AMOUNT_TRANSFER ))
echo "✔ ok"

echo "* Wait and assert Bob's incognito account balance... "
assert_account_balance ${MRENCLAVE} ${ICGACCOUNTBOB} ${AMOUNT_TRANSFER}
echo "✔ ok"
echo ""

echo "* Un-shield ${AMOUNT_UNSHIELD} tokens from Alice's incognito account to Ferie's L1 account"
${CLIENT} trusted --mrenclave ${MRENCLAVE} unshield-funds ${ICGACCOUNTALICE} //Ferdie ${AMOUNT_UNSHIELD}
echo ""

echo "* Wait and assert Alice's incognito account balance... "
assert_account_balance ${MRENCLAVE} ${ICGACCOUNTALICE} $(( BALANCE_INCOGNITO_ALICE + AMOUNT_SHIELD - AMOUNT_TRANSFER - AMOUNT_UNSHIELD ))
echo "✔ ok"

echo "* Wait and assert Bob's incognito account balance... "
assert_account_balance ${MRENCLAVE} ${ICGACCOUNTBOB} ${AMOUNT_TRANSFER}
echo "✔ ok"

# Test the nonce handling, using Bob's incognito account as the sender as Alice's
# balance needs to be verified in the second round while Bob is newly created each time

echo "* Create a new incognito account for Charlie"
ICGACCOUNTCHARLIE=$(${CLIENT} trusted --mrenclave ${MRENCLAVE} new-account)
echo "  Charlie's incognito account = ${ICGACCOUNTCHARLIE}"
echo ""

echo "* Assert Bob's incognito initial nonce..."
assert_account_nonce ${MRENCLAVE} ${ICGACCOUNTBOB} 0
echo "✔ ok"
echo ""

echo "* Send 3 consecutive 0.2 UNIT balance Transfer Bob -> Charlie"
for i in $(seq 1 3); do
    # use direct calls so they are submitted to the top pool synchronously
    $CLIENT trusted --direct --mrenclave ${MRENCLAVE} transfer ${ICGACCOUNTBOB} ${ICGACCOUNTCHARLIE} $(( AMOUNT_TRANSFER / 10 ))
done
echo ""

echo "* Assert Bob's incognito current nonce..."
assert_account_nonce ${MRENCLAVE} ${ICGACCOUNTBOB} 3
echo "✔ ok"
echo ""

echo "* Send a 2 UNIT balance Transfer Bob -> Charlie (that will fail)"
$CLIENT trusted --direct --mrenclave ${MRENCLAVE} transfer ${ICGACCOUNTBOB} ${ICGACCOUNTCHARLIE} ${AMOUNT_TRANSFER} || true
echo ""

echo "* Assert Bob's incognito nonce..."
# the nonce should be increased nontheless, even for the failed tx
assert_account_nonce ${MRENCLAVE} ${ICGACCOUNTBOB} 4
echo "✔ ok"
echo ""

echo "* Send another 0.2 UNIT balance Transfer Bob -> Charlie"
$CLIENT trusted --direct --mrenclave ${MRENCLAVE} transfer ${ICGACCOUNTBOB} ${ICGACCOUNTCHARLIE} $(( AMOUNT_TRANSFER / 10 ))
echo ""

echo "* Assert Bob's incognito nonce..."
assert_account_nonce ${MRENCLAVE} ${ICGACCOUNTBOB} 5
echo "✔ ok"
echo ""

echo "* Wait and assert Bob's incognito account balance... "
# in total 4 balance transfer should go through => 1.2 UNIT remaining
assert_account_balance ${MRENCLAVE} ${ICGACCOUNTBOB} $(( AMOUNT_TRANSFER * 6 / 10 ))
echo "✔ ok"

echo ""
echo "-----------------------"
echo "✔ The $TEST test passed!"
echo "-----------------------"
echo ""
