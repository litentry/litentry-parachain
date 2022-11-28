#!/bin/bash
set -euo pipefail

ROOTDIR=$(git rev-parse --show-toplevel)
DESTDIR="$ROOTDIR/docker/litentry"
PARACHAIN_DIR=$(mktemp -d)
git clone https://github.com/litentry/litentry-parachain "$PARACHAIN_DIR"
cd "$PARACHAIN_DIR"
git checkout tee-dev

cp -f docker/rococo-parachain-launch-config.tee-dev.yml docker/rococo-parachain-launch-config.yml
# generate files
make generate-docker-compose-rococo
# copy files over to `DESTDIR`
mkdir -p "$DESTDIR"
cp docker/generated-rococo/* "$DESTDIR/"
# clean up
rm -rf "$PARACHAIN_DIR"