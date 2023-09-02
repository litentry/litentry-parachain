#!/bin/bash
set -euo pipefail

ROOTDIR=$(git rev-parse --show-toplevel)
DESTDIR="$ROOTDIR/tee-worker/docker/litentry"

# generate files
cd "$ROOTDIR"
make generate-docker-compose-rococo

if [ $(stat -c %s docker/generated-rococo/rococo-local.json) -ne 5040588 ]; then
    echo "unexpected rococo-local.json size"
    exit 1
fi

# copy files over to `DESTDIR`
mkdir -p "$DESTDIR"
cp docker/generated-rococo/* "$DESTDIR/"