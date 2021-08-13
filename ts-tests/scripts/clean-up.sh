#!/bin/sh

echo "killing litentry-token-server ..."
killall litentry-token-server
echo "killing polkadot ..."
killall polkadot
echo "killing litentry-collator ..."
killall litentry-collator

echo

if [ "$2" -eq 0 ]; then
    echo "Removing $1 ..."
    rm -rf "$1"
else
    echo "Please check logs in $1"
fi
