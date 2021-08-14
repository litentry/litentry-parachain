#!/bin/bash

set -o pipefail

TMP_DIR="${1:-/tmp}"
basedir=$(dirname "$0")

. $basedir/constants.sh $TMP_DIR

echo "Stop token server ..."
[ -f $TOKEN_SERVER_PIDFILE ] && kill -9 $(cat $TOKEN_SERVER_PIDFILE)

echo "Stop polkadot  ..."
[ -f $RELAY_ALICE_PIDFILE ]  && kill -9  $(cat $RELAY_ALICE_PIDFILE)
[ -f $RELAY_BOB_PIDFILE ]    && kill -9  $(cat $RELAY_BOB_PIDFILE)

echo "Stop litentry collator  ..."
[ -f $PARA_ALICE_PIDFILE ]   && kill -9  $(cat $PARA_ALICE_PIDFILE)
[ -f $PARA_BOB_PIDFILE ]     && kill -9  $(cat $PARA_BOB_PIDFILE)

exit 0
