#!/bin/bash
set -euo pipefail

PARACHAIN_DIR=/tmp/litentry-parachain

cd "$PARACHAIN_DIR"
make clean-docker-rococo || true
rm -rf "$PARACHAIN_DIR"