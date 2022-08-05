#!/usr/bin/env bash

ROOTDIR=$(git rev-parse --show-toplevel)

GOPATH=${HOME}/go go install github.com/litentry/ChainBridge/cmd/chainbridge@dev

${ROOTDIR}/scripts/geth/run_geth.sh

