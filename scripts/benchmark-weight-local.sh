#!/usr/bin/env bash

set -eo pipefail

# This script benchmarks the runtime or pallet weight locally.
#
# When benchmarking pallet weight, only our own pallets are supported.
# Therefore substrate (or other github) pallets are not supported:
# they are benchmarked by the source anyway (e.g. SubstrateWeigt)
# The `litentry-collator` binary must be compiled with `runtime-benchmarks` feature.
#
# When benchmarking runtime weight, a third parameter is needed to
# define the runtime: litentry or litmus.

function usage() {
    echo "Usage: $0 litentry|litmus pallet-name runtime|pallet"
}

[ $# -ne 3 ] && (usage; exit 1)

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR"

PALLET=${2//-/_}

echo "running $1-$3 benchmark for $PALLET locally ..."

case "$3" in
    runtime)
        OUTPUT="--output=./runtime/$1/src/weights/${PALLET//:://}.rs"
        TEMPLATE=
        CHAIN="--chain=generate-$1"
        ;;
    pallet)
        OUTPUT="$(cargo pkgid -q $2 | sed 's/.*litentry-parachain/\./;s/#.*/\/src\/weights.rs/')"
        TEMPLATE="--template=./templates/benchmark/pallet-weight-template.hbs"
        CHAIN="--chain=$1-dev"
        if [[ $OUTPUT == *"github.com"* ]]; then
            echo "only local pallets can be benchmarked"
            exit 2
        else
            OUTPUT="--output=$OUTPUT"
        fi
        ;;
    *)
        usage
        exit 3
        ;;
esac

./target/release/litentry-collator benchmark \
    $CHAIN \
    --execution=wasm  \
    --db-cache=20 \
    --wasm-execution=compiled \
    --pallet="$PALLET" \
    --extrinsic=* \
    --heap-pages=4096 \
    --steps=20 \
    --repeat=50 \
    --header=./LICENSE_HEADER \
    $TEMPLATE \
    $OUTPUT

