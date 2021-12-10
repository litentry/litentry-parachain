#!/usr/bin/env bash

set -eo pipefail

# this script will be run on the remote host
# to calculate the weights

function usage() {
  echo "Usage: $0 branch-or-tag pallet-names"
  echo "       branch-or-tag will be used to checkout codes"
  echo "       pallet-names can be either * or a comma listed pallet names"
  echo "e.g.:  $0 dev *"
  echo "       $0 dev frame-system,pallet-proxy,pallet-collective"
}

[ $# -ne 2 ] && (usage; exit 1)

# pull docker image
docker pull litentry/litentry-parachain:runtime-benchmarks

# clone the repo
TMPDIR=/tmp
cd "$TMPDIR"
[ -d litentry-parachain ] && rm -rf litentry-parachain
git clone https://github.com/litentry/litentry-parachain
cd litentry-parachain
git checkout "$1"

# copy binary out
docker cp $(docker create --rm litentry/litentry-parachain:runtime-benchmarks):/usr/local/bin/litentry-collator .
chmod a+x litentry-collator

# poopulate PALLETS
PALLETS=
case "$2" in
  '*')  PALLETS=$(grep add_benchmark! runtime/src/lib.rs | tr ',' ' ' | awk '{print $3}' | paste -s -d' ' -) ;;
  *)    PALLETS=$(echo "$2" | tr ',' ' ') ;;
esac
PALLETS=${PALLETS//-/_}

echo "Pallets:"
echo "$PALLETS"

for p in $PALLETS; do
  echo "benchmarking $p ..."
  # filter out the flooding warnings from pallet_scheduler:
  # Warning: There are more items queued in the Scheduler than expected from the runtime configuration.
  #          An update might be needed
  RUST_LOG=runtime::scheduler=error ./litentry-collator benchmark \
      --chain=dev \
      --execution=wasm  \
      --db-cache=20 \
      --wasm-execution=compiled \
      --pallet="$p" \
      --extrinsic=* \
      --heap-pages=4096 \
      --steps=20 \
      --repeat=50 \
      --header=./LICENCE_HEADER \
      --output=./runtime/src/weights/"$p".rs
done