#!/usr/bin/env bash

function usage() {
  echo "Usage: $0 litentry|litmus"
}

[ $# -ne 1 ] && (usage; exit 1)

CHAIN=$1

# wait interval in seconds to check the block import and production of parachain
WAIT_INTERVAL_SECONDS=10

# rounds to wait in `check_block`, in total it's 12 * 10 = 2min
WAIT_ROUNDS=12

# restart interval in seconds for restarting relaychains sequentially
RESTART_INTERVAL_SECONDS=20

# if the parachain ever produces the first block
HAS_BLOCK=false

function print_divider() {
  echo "------------------------------------------------------------"
}

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR/docker/generated-$CHAIN/"

docker-compose up -d --build

# sleep for a while to make sure `docker-compose` is ready
# otherwise `docker-compose logs` could print empty output
sleep 10

parachain_service=$(docker-compose ps --services --filter 'status=running' | grep -F 'parachain-')
relaychain_service="$(docker-compose ps --services --filter 'status=running' | grep -F 'relaychain-')"

print_divider

function check_block() {
  for i in $(seq 1 $WAIT_ROUNDS); do
    sleep $WAIT_INTERVAL_SECONDS
    if docker-compose logs "$parachain_service" | grep -F '[Parachain]' | grep -Fq "best: #1"; then
      echo "parachain produced #1, all good. Quit now"
      exit 0
    fi
  done
}

echo "waiting for parachain to import blocks ..."

while : ; do
  if docker-compose logs "$parachain_service" | grep -F '[Parachain]' | grep -Fq "Imported #1"; then
    echo "parachain imported #1"
    break
  else
    sleep $WAIT_INTERVAL_SECONDS
  fi
done

print_divider
echo "checking parachain block production ..."
check_block

echo "no parachain blocks detected, restart relaychains ..."
for i in $relaychain_service; do
  sleep $RESTART_INTERVAL_SECONDS
  docker-compose restart $i
done

print_divider
echo "relaychain restarted"
echo "checking parachain block production again ..."

check_block
echo "still no blocks detected, you might want to check it manually. Quit now"