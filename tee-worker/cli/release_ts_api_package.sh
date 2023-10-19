#!/bin/bash

# Copyright 2020-2023 Trust Computing GmbH.

set -euo pipefail

function usage() {
    echo "Usage: $0 <Options>"
    echo "publishing"
}

[ $# -ne 1 ] && (usage; exit 1)
TEST=$1

cd /client-api
pnpm install
pnpm run build

echo "building api package"
