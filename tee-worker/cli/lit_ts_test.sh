#!/bin/bash

# Copyright 2020-2023 Trust Computing GmbH.

set -euo pipefail

function usage() {
    echo "Usage: $0 <Options>"
    echo ""
    echo "This is a script for tee-worker ts-test. Current available Options:"
    echo "  test-ii-identity: "
    echo "  test-ii-vc: "
    echo "  test-resuming-worker: "
    echo ""
    echo "Please try to extend the above list when adding new ts-test."
}

[ $# -ne 1 ] && (usage; exit 1)
TEST=$1

cd /ts-tests

pnpm install
pnpm --filter integration-tests run $TEST:staging
