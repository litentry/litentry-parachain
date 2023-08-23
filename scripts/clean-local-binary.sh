#!/usr/bin/env bash

# no `set -e` here as we allow commands to fail in this script

TMPDIR=${TMPDIR:-"/tmp/parachain_dev"}

# for f in $(ls $TMPDIR/*.pid 2>/dev/null); do
#   echo "Killing $f ..."
#   kill -9 $(cat "$f")
# done

# use killall here:
# the previously written PID could change if any process restarted
killall polkadot
killall litentry-collator

rm -rf "$TMPDIR"

echo "cleaned up."