#!/bin/bash

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

LOG_FOLDER="./../log"

echo "Using client binary $CLIENT_BIN"
echo "Using node uri $NODEURL:$NPORT"
echo "Using trusted-worker uri $WORKER1URL:$WORKER1PORT"
echo ""

ACC=//Bob
TIMEOUT=100000 # 100 seconds

CLIENT="$CLIENT_BIN -p $NPORT -P $WORKER1PORT -u $NODEURL -U $WORKER1URL"
echo "CLIENT is $CLIENT"

echo "* Query on-chain enclave registry:"
$CLIENT list-workers
echo ""

if [ "$READMRENCLAVE" = "file" ]
then
    read MRENCLAVE <<< $(cat ~/mrenclave.b58)
    echo "Reading MRENCLAVE from file: ${MRENCLAVE}"
else
    # This will always take the first MRENCLAVE found in the registry !!
    read MRENCLAVE <<< $($CLIENT list-workers | awk '/  MRENCLAVE: / { print $2; exit }')
    echo "Reading MRENCLAVE from worker list: ${MRENCLAVE}"
fi
[[ -z $MRENCLAVE ]] && { echo "MRENCLAVE is empty. cannot continue" ; exit 1; }


# indirect call that will be sent to the parachain, it will be synchronously handled
sleep 10
echo "* Set $ACC 's heartbeat timeout to $TIMEOUT"
${CLIENT} set-heartbeat-timeout-storage "$ACC" "$TIMEOUT"
echo ""

if grep -qr "Received SetHeartbeatTimeoutStorage" $LOG_FOLDER; then
    echo "SetHeartbeatTimeoutStorage test passed"
else
    echo "No SetHeartbeatTimeoutStorage event found"
    exit 1
fi
