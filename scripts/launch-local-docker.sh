#!/usr/bin/env bash

function usage() {
  echo "Usage: $0 litentry|litmus"
}

[ $# -ne 1 ] && (usage; exit 1)

CHAIN=$1

# interval and rounds to wait to check the block production and finalization of parachain
WAIT_INTERVAL_SECONDS=10
WAIT_ROUNDS=30

# if the parachain has produced the first block
BLOCK_PRODUCED=false

function print_divider() {
  echo "------------------------------------------------------------"
}

TMPDIR=/tmp/parachain_dev
[ -d "$TMPDIR" ] || mkdir -p "$TMPDIR"

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR/docker/generated-$CHAIN/"

docker-compose up -d --build

# sleep for a while to make sure `docker-compose` is ready
# otherwise `docker-compose logs` could print empty output
sleep 10

parachain_service=$(docker-compose ps --services --filter 'status=running' | grep -F 'parachain-')

print_divider

[ -d "${ROOTDIR}/scripts/bridge/" ] || mkdir -p "${ROOTDIR}/scripts/bridge/"
# Build the image
echo "Building litentry/chainbridge:latest docker image ..."
docker build --no-cache -f ${ROOTDIR}/docker/bridge.dockerfile -t litentry/chainbridge:latest .

print_divider

docker run -d --rm --name chainbridge litentry/chainbridge bash -c 'ls /go/ChainBridge/build && sleep 10'
docker cp chainbridge:/go/ChainBridge/build/chainbridge ${TMPDIR}/
echo "copy binary:chainbridge to ${TMPDIR}"

print_divider

docker rm -f geth &>/dev/null
docker run -d --rm --entrypoint 'sh' --name 'geth' -v ${ROOTDIR}/scripts/geth:/data/ -p 8546:8546 -p 8545:8545 ethereum/client-go:latest /data/run_geth.sh docker /data
echo "runing geth...(container name: geth)"

print_divider

echo "waiting for parachain to produce blocks ..."

for i in $(seq 1 $WAIT_ROUNDS); do
  sleep $WAIT_INTERVAL_SECONDS
  if docker-compose logs "$parachain_service" 2>&1 | grep -F '0 peers' 2>/dev/null | grep -Fq "best: #1" 2>/dev/null; then
    echo "parachain produced #1"
    BLOCK_PRODUCED=true
    break
  fi
done

if [ "$BLOCK_PRODUCED" = "false" ]; then
  echo "no block production detected, you might want to check it manually. Quit now"
  exit 1
fi

print_divider

echo "waiting for parachain to finalize blocks ..."

for i in $(seq 1 $WAIT_ROUNDS); do
  sleep $WAIT_INTERVAL_SECONDS
  if docker-compose logs "$parachain_service" 2>&1 | grep -F '0 peers' 2>/dev/null | grep -Fq "finalized #1" 2>/dev/null; then
    echo "parachain finalized #1, all good. Quit now"
    exit 0
  fi
done

echo "no block finalization detected, you might want to check it manually. Quit now"
exit 1