#!/usr/bin/env bash

set -eo pipefail

function usage() {
    echo "Usage: $0 pallet-name"
}

[ $# -ne 1 ] && (usage; exit 1)

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR"

echo "running benchmark for $1"

./target/release/litentry-collator benchmark \
    --chain=./source/local.json \
    --execution=wasm  \
    --db-cache=20 \
    --wasm-execution=compiled \
    --pallet="$1" \
    --extrinsic=* \
    --heap-pages=4096 \
    --steps=20 \
    --repeat=50 \
    --output=./runtime/src/weights/"$1".rs \
    --template=./.maintain/frame-weight-template.hbs