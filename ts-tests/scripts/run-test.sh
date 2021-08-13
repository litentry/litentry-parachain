#!/bin/bash

# set -o pipefail

basedir=$(dirname "$0")
cd "$basedir"

cd ..

tmpdir=$(mktemp -d /tmp/tmp.XXXXXX)
tmpdir="${1:-$tmpdir}"
echo "tmpdir: $tmpdir"
echo "NODE_ENV=ci" > .env
# ./scripts/start-relay-and-para-chain.sh "$tmpdir"
# ./scripts/start-token-server.sh "$tmpdir"
# yarn && yarn register-parachain 2>&1 | tee "$tmpdir/register-parachain.log"
# yarn test 2>&1 | tee "$tmpdir/test.log"
# ./scripts/clean-up.sh "$tmpdir" $?
echo hello > "$tmpdir/world.txt"
exit 0
