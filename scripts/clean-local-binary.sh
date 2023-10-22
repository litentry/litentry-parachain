#!/usr/bin/env bash

# no `set -e` here as we allow commands to fail in this script

LITENTRY_PARACHAIN_DIR=${LITENTRY_PARACHAIN_DIR:?}

# for f in $(ls $LITENTRY_PARACHAIN_DIR/*.pid 2>/dev/null); do
#   echo "Killing $f ..."
#   kill -9 $(cat "$f")
# done

# use killall here:
# the previously written PID could change if any process restarted
killall polkadot
killall litentry-collator

rm -rf "$LITENTRY_PARACHAIN_DIR"

echo "cleaned up."