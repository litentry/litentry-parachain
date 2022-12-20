#!/bin/bash
set -euo pipefail

function usage() {
    echo "Usage: $0 <litentry|litmus|rococo>"
}
[ $# -ne 1 ] && (usage; exit 1)

CHAIN=$1

# TODO: remove later
if [ "${CHAIN}" != 'rococo' ]; then
    echo "only support 'rococo' for the moment"
    usage; exit 1
fi

PARACHAIN_DIR=/tmp/litentry-parachain
[ -d "$PARACHAIN_DIR" ] && rm -rf "$PARACHAIN_DIR"
git clone https://github.com/litentry/litentry-parachain "$PARACHAIN_DIR"
cd "$PARACHAIN_DIR"
git checkout tee-dev

cp -f docker/${CHAIN}-parachain-launch-config.tee-dev.yml docker/${CHAIN}-parachain-launch-config.yml

make launch-docker-${CHAIN}
