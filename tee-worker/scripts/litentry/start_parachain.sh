#!/bin/bash
set -euo pipefail

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR"

function usage() {
    echo "Usage: $0 [standalone-mode]"
    echo
    echo "standalone-mode: true or false, default: true."
    echo "                 If the parachain should be started in standalone mode"
}

[ $# -gt 1 ] && (usage; exit 1)

STANDALONE=${1:-true}

if [ $STANDALONE = 'true' ]; then
    echo "starting parachain in standalone mode ..."
    docker run -d -p 9944:9944 -p 9933:9933 --rm --name litentry-parachain-standalone litentry/litentry-parachain:tee-dev \
        --dev --unsafe-ws-external --unsafe-rpc-external
    for i in $(seq 1 3); do
        sleep 10
        if docker container logs litentry-parachain-standalone 2>&1 | grep -Fq 'finalized #1' 2>/dev/null; then
            echo "parachain produced #1"
            echo "All good, quit now."
            BLOCK_PRODUCED=true
            break
        fi
    done

    if [ "$BLOCK_PRODUCED" = "false" ]; then
        echo "no block production detected, you might want to check it manually. Quit now"
        exit 1
    fi
else
    sed -i.bak "s;litentry-parachain:latest;litentry-parachain:tee-dev;" docker/rococo-parachain-launch-config.yml
    make launch-docker-rococo
fi