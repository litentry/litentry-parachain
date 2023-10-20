#!/bin/bash

# Copyright 2020-2023 Trust Computing GmbH.

set -euo pipefail

function usage() {
    echo "Usage: $0 <Options>"
    echo "updating metadata"
}

[ $# -ne 1 ] && (usage; exit 1)
TEST=$1

cd /client-api
pnpm install
pnpm run update-metadata
git status
git diff
# pnpm run build

echo "update metadata"
