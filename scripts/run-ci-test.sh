#!/usr/bin/env bash

set -eo pipefail

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR/ts-tests"

tmpdir="${1:-/tmp}"

echo "NODE_ENV=ci" > .env
yarn && yarn test 2>&1 | tee "$tmpdir/run_test.log"
