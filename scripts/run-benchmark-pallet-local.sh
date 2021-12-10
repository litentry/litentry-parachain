#!/usr/bin/env bash

set -eo pipefail

function usage() {
    echo "Usage: $0 pallet-name"
}

[ $# -ne 1 ] && (usage; exit 1)

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR"

echo "running benchmark for $1"

filename=${1//-/_}

./target/release/litentry-collator benchmark \
    --chain=dev \
    --execution=wasm  \
    --db-cache=20 \
    --wasm-execution=compiled \
    --pallet="$1" \
    --extrinsic=* \
    --heap-pages=4096 \
    --steps=20 \
    --repeat=50 \
    --header=./LICENCE_HEADER \
    --output=./runtime/src/weights/"$filename".rs