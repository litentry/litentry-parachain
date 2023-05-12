#!/usr/bin/env bash

TMPDIR=/tmp/parachain_dev
[ -d "$TMPDIR" ] || mkdir -p "$TMPDIR"

ROOTDIR=$(git rev-parse --show-toplevel)

if [[ "$(docker image inspect litentry/chainbridge:latest 2>/dev/null)" == "[]" ]]; then
    echo "litentry/chainbridge:latest image not found..."
    ${ROOTDIR}/scripts/build-bridge-docker.sh
fi

echo "------------------------------------------------------------"

docker run -d --rm --name chainbridge litentry/chainbridge bash -c 'ls /go/bin/ && sleep 5'
docker cp chainbridge:/go/bin/chainbridge ${TMPDIR}/
echo "copy binary:chainbridge to ${TMPDIR}"

echo "------------------------------------------------------------"

docker rm -f geth &>/dev/null
docker run -d --rm --entrypoint 'sh' --name 'geth' \
    -u "$(id -u)":"$(id -g)" -v ${ROOTDIR}/scripts/geth:/data/ -p 8546:8546 -p 8545:8545 \
    ethereum/client-go:latest /data/run_geth.sh docker /data
echo "runing geth...(container name: geth)"
