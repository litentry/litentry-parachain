#!/usr/bin/env bash

# no `set -e` here as we allow commands to fail in this script

function usage() {
  echo "Usage: $0 litentry|litmus|rococo"
}

[ $# -ne 1 ] && (usage; exit 1)

TMPDIR=/tmp/parachain_dev
CHAIN=$1

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR/docker/generated-$CHAIN"

docker images

echo "stop and remove docker containers..."
docker compose rm -f -s -v
docker container stop geth || true

echo "remove docker volumes..."
docker volume ls | grep generated-$CHAIN | sed 's/local *//' | xargs docker volume rm

echo "remove gethdata/..."
rm -rf "$ROOTDIR/scripts/geth/gethdata"

echo "remove dangling docker images if any..."
IMG=$(docker images --filter=dangling=true -q)
[ -z "$IMG" ] || docker rmi -f $IMG

echo "keep litentry/litentry-parachain:latest while removing other tags..."
IMG=$(docker images litentry/litentry-parachain --format "{{.Repository}}:{{.Tag}}" | grep -v latest)
[ -z "$IMG" ] || docker rmi -f $IMG

echo "remove generated images..."
IMG=$(docker images --filter=reference="generated-$CHAIN*" --format "{{.Repository}}:{{.Tag}}")
[ -z "$IMG" ] || docker rmi -f $IMG

rm -rf "$TMPDIR"

echo "cleaned up."
