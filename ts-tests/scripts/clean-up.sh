#!/bin/sh

TMP_DIR="$1"

echo "killing litentry-token-server ..."
killall litentry-token-server
echo "killing polkadot ..."
killall polkadot
echo "killing litentry-collator ..."
killall litentry-collator

echo

if [ "$2" -eq 0 ]; then
    echo "Removing $TMP_DIR ..."
    rm -rf "$TMP_DIR"
else
    echo "Please check logs in $TMP_DIR"
fi
