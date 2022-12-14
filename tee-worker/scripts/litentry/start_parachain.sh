#!/bin/bash
set -euo pipefail

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR"

sed -i.bak "s;litentry-parachain:latest;litentry-parachain:tee-dev;" docker/rococo-parachain-launch-config.yml
make launch-docker-rococo