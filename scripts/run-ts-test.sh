#!/usr/bin/env bash

set -eo pipefail

bridge=false
if [ -n "$1" ]; then
    bridge=true
fi

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR/ts-tests"

TMPDIR=/tmp/parachain_dev
[ -d "$TMPDIR" ] || mkdir -p "$TMPDIR"

[ -f .env ] || echo "NODE_ENV=ci" >.env
yarn
yarn test 2>&1 | tee "$TMPDIR/parachain_ci_test.log"
if $bridge; then
    yarn test-bridge 2>&1 | tee -a "$TMPDIR/parachain_ci_test.log"
fi
