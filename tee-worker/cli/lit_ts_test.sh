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
    echo ""
    echo "Please try to extend the above list when adding new ts-test."
}

[ $# -ne 1 ] && (usage; exit 1)
TEST=$1

cd /ts-tests

apt-get install -y curl
bash <(curl -fsSL https://deb.nodesource.com/setup_18.x)
apt-get update
apt-get install -y nodejs
npm install -g yarn

yarn install
yarn run $TEST:staging
