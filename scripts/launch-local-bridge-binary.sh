#!/usr/bin/env bash

ROOTDIR=$(git rev-parse --show-toplevel)

#GOPATH=${HOME}/go go install github.com/Phala-Network/ChainBridge/cmd/chainbridge@main

${ROOTDIR}/scripts/geth/run_geth.sh

