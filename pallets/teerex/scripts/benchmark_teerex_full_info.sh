#!/bin/bash
set -eo pipefail

# This creates an extended weight file that contains:
# * `WeightInfo` trait declaration
# * `WeightInfo` implementation for an `IntegriteeRuntimeWeight` struct
# * `WeightInfo` implementation for `()` used in testing.
#
# The output file is intended to be used in the `pallet_teerex` internally for development only. It contains more
# information than needed for the actual node.

# use absolute paths to call this from wherever we want
SCRIPTS_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
PROJ_ROOT="$(dirname "$SCRIPTS_DIR")"
TEEREX_SRC="$PROJ_ROOT/src"

echo "PROJ_ROOT:    $SCRIPTS_DIR"
echo "SCRIPTS_DIR:  $PROJ_ROOT"
echo "TEEREX_SRC:   $TEEREX_SRC"

NODE_BINARY=${1:-../integritee-node/target/release/integritee-node}
CHAIN_SPEC=${2:-integritee-mainnet}

echo "Generating weights for pallet_teerex"
echo "node:   $NODE_BINARY"
echo "chain:  $CHAIN_SPEC"

./"$NODE_BINARY" \
  benchmark \
  --chain="$CHAIN_SPEC" \
  --steps=50 \
  --repeat=20 \
  --pallet=pallet_teerex \
  --extrinsic="*" \
  --execution=wasm \
  --wasm-execution=compiled \
  --heap-pages=4096 \
  --output="$TEEREX_SRC"/weights.rs \
  --template="$SCRIPTS_DIR"/frame-weight-template-full-info.hbs

