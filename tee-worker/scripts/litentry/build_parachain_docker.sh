#!/bin/bash
set -euo pipefail

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR"
# with '#' so that it filters out the pallet dependency
SHA=$(grep -F 'https://github.com/litentry/litentry-parachain.git?branch=tee-dev#' Cargo.lock | head -n1 | sed 's/.*#//;s/"$//')

PARACHAIN_DIR=/tmp/litentry-parachain
[ -d "$PARACHAIN_DIR" ] && rm -rf "$PARACHAIN_DIR"
git clone https://github.com/litentry/litentry-parachain "$PARACHAIN_DIR"
cd "$PARACHAIN_DIR"
git checkout tee-dev
git checkout "$SHA"

./scripts/build-docker.sh release tee-dev --features=tee-dev