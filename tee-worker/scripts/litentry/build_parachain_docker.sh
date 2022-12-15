#!/bin/bash
set -euo pipefail

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR"
./scripts/build-docker.sh release tee-dev --features=tee-dev