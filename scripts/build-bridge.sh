#!/usr/bin/env bash
set -eo pipefail

ROOTDIR=$(git rev-parse --show-toplevel)

# Build the image
echo "Building litentry/chainbridge:latest docker image ..."
docker build --no-cache -f ${ROOTDIR}/docker/bridge.dockerfile -t litentry/chainbridge:latest .