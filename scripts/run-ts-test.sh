#!/usr/bin/env bash

set -eo pipefail

bridge=false

case "$1" in
    litentry|litmus|rococo) export PARACHAIN_TYPE=$1 ;;
    *) echo "usage: ./$0 litmus|litentry [bridge]"; exit 1 ;;
esac
    
if [ "$2" = "bridge" ]; then
    bridge=true
fi

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR/ts-tests"

TMPDIR=${TMPDIR:-"/tmp/parachain_dev"}

[ -d "$TMPDIR" ] || mkdir -p "$TMPDIR"

[ -f .env ] || echo "NODE_ENV=ci" >.env
corepack yarn
corepack yarn test-filter 2>&1 | tee "$TMPDIR/parachain_ci_test.log"
if $bridge; then
    corepack yarn test-bridge 2>&1 | tee -a "$TMPDIR/parachain_ci_test.log"
fi
