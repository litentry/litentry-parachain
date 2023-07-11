#!/bin/bash

# Copyright 2020-2023 Litentry Technologies GmbH.

set -euo pipefail

function usage() {
    echo "Usage: $0 <Options>"
    echo ""
    echo "This is a script for tee-worker ts-test. Current available Options:"
    echo "  test-identity: "
    echo "  test-vc: "
    echo "  test-resuming-worker: "
    echo "  test-bulk-identity: "
    echo "  test-bulk-vc: "
    echo ""
    echo "Please try to extend the above list when adding new ts-test."
}

[ $# -ne 1 ] && (usage; exit 1)
TEST=$1

echo "workspace integration-tests $TEST:staging"

cd /ts-tests/integration-tests

echo "workspace integration-tests $TEST:staging"

yarn install
yarn run $TEST:staging
