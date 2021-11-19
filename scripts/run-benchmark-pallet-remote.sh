#!/usr/bin/env bash

set -eo pipefail

# this script will be run on the remote host
# to calculate the weights

function usage() {
  echo "Usage: $0 pallet-names"
  echo "       pallet-names can be either * or a comma listed pallet names"
  echo "e.g.:  $0 *"
  echo "       $0 frame-system,pallet-proxy,pallet-collective"
}

[ $# -ne 1 ] && (usage; exit 1)

# pull docker image
docker pull litentry/litentry-parachain:runtime-benchmarksß

# clone the repo
TMPDIR=$(mktemp -d /tmp/XXXXXX)
cd "$TMPDIR"
git clone https://github.com/litentry/litentry-parachain
cd litentry-parachain

# copy binary out
docker cp $(docker create --rm litentry/litentry-parachain:runtime-benchmarks):/usr/local/bin/litentry-collator .
chmod a+x litentry-collator

# poopulate PALLETS
PALLETS=
caes "$1" in
  '*')  PALLETS=$(grep '::{Pallet' runtime/src/lib.rs | sed 's/::{Pallet.*//;s/::<.*//;s/.*: *//' | uniq | paste -s -d" " -) ;;
  *)    PALLETS=$(echo "$1" | tr ',' ' ') ;;
esac
PALLETS=${PALLETS//-/_}

echo "Pallets:"
echo "$PALLETS"

for p in $PALLETS; do
  echo "bencharking $p ..."
  ./litentry-collator benchmark \
      --chain=dev \
      --execution=wasm  \
      --db-cache=20 \
      --wasm-execution=compiled \
      --pallet="$p" \
      --extrinsic=* \
      --heap-pages=4096 \
      --steps=20 \
      --repeat=50 \
      --output=./runtime/src/weights/"$p".rs
done