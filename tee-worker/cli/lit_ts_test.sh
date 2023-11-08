#!/bin/bash

# Copyright 2020-2023 Trust Computing GmbH.

set -euo pipefail

function usage() {
    echo ""
    echo "This is a script for tee-worker ts-test. Preparing to test: $1"
    echo ""

}

[ $# -ne 1 ] && (usage; exit 1)
TEST=$1

cd /ts-tests
pnpm install
pnpm --filter integration-tests run $TEST:staging
