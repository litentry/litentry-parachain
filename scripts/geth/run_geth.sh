#!/usr/bin/env bash
# Copyright 2020 ChainSafe Systems
# SPDX-License-Identifier: LGPL-3.0-only

TMPDIR=/tmp/parachain_dev
[ -d "$TMPDIR" ] || mkdir -p "$TMPDIR"

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR"

GETH_BIN="geth"
if ! geth version &> /dev/null; then
    echo "geth could not be found..download now"
    url="https://gethstore.blob.core.windows.net/builds/geth-linux-386-1.10.21-67109427.tar.gz"
    GETH_BIN="$TMPDIR/geth"
    wget -O "$TMPDIR/geth.tar.gz" -q "$url"
    tar -xf "$TMPDIR/geth.tar.gz" --strip-components 1 -C "$TMPDIR"
    chmod a+x "$GETH_BIN"
fi

DATADIR="${ROOTDIR}/scripts/geth/gethdata"

# Exit on failure
set -e

# Delete old chain data
rm -rf ${DATADIR}
# Init genesis
eval "${GETH_BIN} init --datadir ${DATADIR} ${ROOTDIR}/scripts/geth/genesis.json"
# Copy keystore
rm -rf ${DATADIR}/keystore
cp -r ${ROOTDIR}/scripts/geth/keystore ${DATADIR}
# Start geth with rpc, mining and unlocked accounts

start="${GETH_BIN} --datadir ${DATADIR} \
    --nodiscover \
    --unlock '0xff93B45308FD417dF303D6515aB04D9e89a750Ca','0x8e0a907331554AF72563Bd8D43051C2E64Be5d35','0x24962717f8fA5BA3b931bACaF9ac03924EB475a0','0x148FfB2074A9e59eD58142822b3eB3fcBffb0cd7','0x4CEEf6139f00F9F4535Ad19640Ff7A0137708485' \
    --password ${ROOTDIR}/scripts/geth/password.txt \
    --ws \
    --ws.port 8546 \
    --networkid 5 \
    --ws.origins='*' \
    --http \
    --http.port 8545 \
    --http.corsdomain='*' \
    --miner.gaslimit 8000000 \
    --allow-insecure-unlock \
    --mine
"

eval ${start}  2>&1 &> 'geth.log' &