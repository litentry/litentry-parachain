#!/bin/bash
set -euo pipefail

ROOTDIR=$(git rev-parse --show-toplevel)
DESTDIR="$ROOTDIR/tee-worker/docker/litentry"

# generate files
cd "$ROOTDIR"
make generate-docker-compose-rococo

# copy files over to `DESTDIR`
mkdir -p "$DESTDIR"
cp docker/generated-rococo/* "$DESTDIR/"