#!/bin/sh

TMP_DIR="/tmp"
TMP_DIR="${1:-$TMP_DIR}"
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

echo

if [[ $2 -eq 0 ]]; then
    echo "Removing $TMP_DIR ..."
    # rm -rf "$TMP_DIR"
else
    echo "Please check logs in $TMP_DIR"
fi
