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

echo "Using client binary $CLIENT_BIN"
echo "Using node uri $NODEURL:$NPORT"
echo "Using trusted-worker uri $WORKER1URL:$WORKER1PORT"
echo ""

ACC=//Bob
KEY="22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12"

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
echo "* Set $ACC 's shielding key to $KEY"
${CLIENT} set-user-shielding-key "$ACC" "$KEY" ${MRENCLAVE}
echo ""

sleep 20
echo "* Get $ACC 's shielding key"
ACTUAL_KEY=$($CLIENT trusted --mrenclave $MRENCLAVE --direct user-shielding-key $ACC)
echo ""

if [ "$ACTUAL_KEY" = "$KEY" ]; then
    echo "KEY identical: $KEY"
    echo "test indirect call passed"
else
    echo "KEY non-identical: expected: $KEY actual: $ACTUAL_KEY"
    exit 1
fi

echo "------------------------------"
# direct call that will be asynchronously handled
KEY="8378193a4ce64180814bd60591d1054a04dbc4da02afde453799cd6888ee0c6c"
sleep 10
echo "* Set $ACC 's shielding key to $KEY"
${CLIENT} trusted --mrenclave $MRENCLAVE --direct set-user-shielding-key-preflight "$ACC" "$KEY"
echo ""

sleep 35
echo "* Get $ACC 's shielding key"
ACTUAL_KEY=$($CLIENT trusted --mrenclave $MRENCLAVE --direct user-shielding-key $ACC)
echo ""

if [ "$ACTUAL_KEY" = "$KEY" ]; then
    echo "KEY identical: $KEY"
    echo "test direct call passed"
else
    echo "KEY non-identical: expected: $KEY actual: $ACTUAL_KEY"
    exit 1
fi
