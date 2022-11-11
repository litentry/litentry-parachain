#!/usr/bin/env bash
set -eo pipefail

# This is the dependency preparation for compiling the docker image

sudo apt-get update && \
sudo apt-get install -y --no-install-recommends \
cmake pkg-config libssl-dev git gcc build-essential git clang libclang-dev libudev-dev curl llvm protobuf-compiler \

echo "will build docker images"