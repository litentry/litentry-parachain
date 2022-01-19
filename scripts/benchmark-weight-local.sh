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
    echo "Usage: $0 pallet-name runtime|pallet [litentry|litmus]"
}

[ $# -lt 2 ] && (usage; exit 1)

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR"

PALLET=${1//-/_}

case "$2" in
    runtime)
        echo "running $3-$2 benchmark for $PALLET locally ..."
        OUTPUT="--output=./runtime/$3/src/weights/$PALLET.rs"
        TEMPLATE=
        CHAIN="--chain=generate-$3"
        ;;
    pallet)
        echo "running $2 benchmark for $PALLET locally ..."
        OUTPUT="$(cargo pkgid -q $1 | sed 's/.*litentry-parachain/\./;s/#.*/\/src\/weights.rs/')"
        TEMPLATE="--template=./templates/benchmark/pallet-weight-template.hbs"
        CHAIN="--chain=dev"
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

