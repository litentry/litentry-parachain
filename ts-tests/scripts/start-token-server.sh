#!/bin/sh
# export etherscan="RF71W4Z2RDA7XQD6EN19NGB66C2QD9UPHB"
# export infura="aa0a6af5f94549928307febe80612a2a"
# export blockchain=""
GIT_ROOT=`git rev-parse --show-toplevel`
TOKEN_SERVER_BINARY=$GIT_ROOT/token-server/target/release/litentry-token-server

etherscan="RF71W4Z2RDA7XQD6EN19NGB66C2QD9UPHB" infura="aa0a6af5f94549928307febe80612a2a" blockchain="" $TOKEN_SERVER_BINARY
