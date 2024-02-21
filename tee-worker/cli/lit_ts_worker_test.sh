#!/bin/bash

# Copyright 2020-2024 Trust Computing GmbH.

set -euo pipefail

function usage() {
    echo ""
    echo "This is a script for tee-worker worker ts-test. Pass test name as first argument"
    echo ""
}

[ $# -ne 1 ] && (usage; exit 1)
TEST=$1


BINARY_DIR="/usr/local/bin"
NODE_ENDPOINT="ws://litentry-node:9912"

echo "Using binary dir: $BINARY_DIR"
echo "Using node endpoint: $NODE_ENDPOINT"

cd /ts-tests
pnpm install
NODE_ENV=staging pnpm --filter worker run test $TEST
