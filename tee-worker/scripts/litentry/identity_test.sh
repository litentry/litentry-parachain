#!/usr/bin/env bash

set -eo pipefail

root_dir=$(git rev-parse --show-toplevel)
root_dir="$root_dir/tee-worker"

#NODE PORT
node_port=9912
node_url=ws://integritee-node

worker_url=wss://tee-builder
worker_port=2000

CLIENT="./integritee-cli --node-url ${node_url} --node-port ${node_port} --worker-url ${worker_url} --trusted-worker-port  ${worker_port}"

cd "$root_dir/bin"
./integritee-service mrenclave | tee ~/mrenclave.b58
MRENCLAVE=$(cat ~/mrenclave.b58)

cd "$root_dir/tmp/worker1"

# node-js:  tweet_id: Buffer.from("1571829863862116352").toJSON().data.toString()
validation_data='{"Web2":{"Twitter":{"tweet_id":[49,53,55,49,56,50,57,56,54,51,56,54,50,49,49,54,51,53,50]}}}'

# node-js:  twitter_username: Buffer.from("litentry").toJSON().data.toString()
identity='{"web_type":{"Web2":"Twitter"},"handle":{"String":[108,105,116,101,110,116,114,121]}}'

echo "create_identity"
RUST_LOG=warn ${CLIENT} trusted --mrenclave ${MRENCLAVE} create-identity "//Alice" "$identity"

echo "set-challenge-code"
${CLIENT} trusted --mrenclave ${MRENCLAVE} set-challenge-code "//Alice" "$identity" 1134

echo "verify-identity-preflight"
RUST_LOG=info ${CLIENT} trusted --mrenclave ${MRENCLAVE} verify-identity-preflight "//Alice" "$identity" "$validation_data"
