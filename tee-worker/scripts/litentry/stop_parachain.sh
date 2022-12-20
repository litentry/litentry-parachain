#!/bin/bash
set -euo pipefail

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR"
make clean-docker-rococo || true
