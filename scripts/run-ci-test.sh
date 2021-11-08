#!/usr/bin/env bash

set -eo pipefail

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR/ts-tests"

TMPDIR=/tmp/parachain_dev
[ -d "$TMPDIR" ] || mkdir -p "$TMPDIR"

[ -f .env ] || echo "NODE_ENV=ci" > .env
yarn
yarn test 2>&1 | tee "$TMPDIR/parachain_ci_test.log"
