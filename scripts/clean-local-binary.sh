#!/usr/bin/env bash

# no `set -e` here as we allow commands to fail in this script

TMPDIR=/tmp/parachain_dev

for f in $(ls $TMPDIR/*.pid 2>/dev/null); do
  echo "Killing $f ..."
  kill -9 $(cat "$f")
done

rm -rf "$TMPDIR"

echo "cleaned up."