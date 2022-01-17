#!/usr/bin/env bash
set -eo pipefail

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR"

TAG=${1:-latest}
image="litentry/litentry-parachain:$TAG"

docker pull "$image"

docker run --rm "$image" build-spec --chain=generate-prod > node/res/chain_spec/prod-plain.json
docker run --rm "$image" build-spec --chain=generate-prod --raw > node/res/chain_spec/prod.json

echo "Done, please check node/res/chain_spec/"
ls -l node/res/chain_spec/
