#!/bin/bash
set -euo pipefail

PARACHAIN_DIR=/tmp/litentry-parachain

cd "$PARACHAIN_DIR"
make clean-docker-rococo || true
docker container stop litentry-parachain-standalone || true
rm -rf "$PARACHAIN_DIR"