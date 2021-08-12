#!/bin/sh

GIT_ROOT=`git rev-parse --show-toplevel`
TOKEN_SERVER_BINARY=$GIT_ROOT/token-server/target/release/litentry-token-server

ETHERSCAN="RF71W4Z2RDA7XQD6EN19NGB66C2QD9UPHB"
INFURA="aa0a6af5f94549928307febe80612a2a"
BLOCKCHAIN=""

etherscan=$ETHERSCAN infura=$INFURA blockchain=$BLOCKCHAIN $TOKEN_SERVER_BINARY &> /tmp/litentry-token-server.log &
