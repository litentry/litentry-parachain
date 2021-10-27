#!/usr/bin/env bash

set -eo pipefail

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR/ts-tests"

TMPDIR=/tmp/parachain_dev
[ -d "$TMPDIR" ] || mkdir -p "$TMPDIR"

echo "NODE_ENV=ci" > .env
yarn
if [ "$1" != "docker" ]; then
  yarn register-parachain 2>&1 | tee "$TMPDIR/register-parachain.log"
fi
yarn test 2>&1 | tee "$TMPDIR/parachain_ci_test.log"
