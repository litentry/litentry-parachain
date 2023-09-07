#!/usr/bin/env bash

TMPDIR=${TMPDIR:-"/tmp/parachain_dev"}
[ -d "$TMPDIR" ] || mkdir -p "$TMPDIR"

ROOTDIR=$(git rev-parse --show-toplevel)

GOPATH=${HOME}/go go install github.com/litentry/ChainBridge/cmd/chainbridge@dev

cp ${HOME}/go/bin/chainbridge $TMPDIR/chainbridge

${ROOTDIR}/scripts/geth/run_geth.sh

