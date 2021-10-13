#!/bin/bash

set -o pipefail

basedir=$(dirname "$0")
cd "$basedir/../ts-tests"

tmpdir="${1:-/tmp}"

echo "NODE_ENV=ci" > .env
yarn && yarn test 2>&1 | tee "$tmpdir/run_test.log"
