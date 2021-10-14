#!/usr/bin/env bash

set -eo pipefail

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR/ts-tests"

echo "NODE_ENV=ci" > .env
yarn && yarn test 2>&1 | tee "/tmp/parachain_ci_test.log"
