#!/usr/bin/env bash
set -eo pipefail

err_report() {
  echo "Error on line $1"
}

trap 'err_report $LINENO' ERR

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR"

# generate the full release tag based on the code versions
# for shorter name and better readability:
# - pc: parachain client
# - pr: parachain runtime
# - ic: identity worker client
# - ir: identity worker runtime
# - bc: bitacross worker client
# - br: bitacross worker runtime

pc=$(grep version node/Cargo.toml | head -n1 | sed "s/'$//;s/.*'//")
pr=$(grep spec_version runtime/litentry/src/lib.rs | sed 's/.*version: //;s/,//')
ic=$(grep version tee-worker/service/Cargo.toml | head -n1 | sed "s/'$//;s/.*'//")
ir=$(grep spec_version tee-worker/app-libs/sgx-runtime/src/lib.rs | sed 's/.*version: //;s/,//')
bc=$(grep version bitacross-worker/service/Cargo.toml | head -n1 | sed "s/'$//;s/.*'//")
br=$(grep spec_version bitacross-worker/app-libs/sgx-runtime/src/lib.rs | sed 's/.*version: //;s/,//')
echo "p${pc}-${pr}-i${ic}-${ir}-b${bc}-${br}"