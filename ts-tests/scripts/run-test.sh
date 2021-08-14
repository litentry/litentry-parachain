#!/bin/bash

basedir=$(dirname "$0")
cd "$basedir"

cd ..

tmpdir="${1:-/tmp}"

echo "NODE_ENV=ci" > .env
./scripts/start-relay-and-para-chain.sh "$tmpdir"
./scripts/start-token-server.sh "$tmpdir"
yarn && yarn register-parachain 2>&1 | tee "$tmpdir/register-parachain.log"
yarn test 2>&1 | tee "$tmpdir/test.log"
