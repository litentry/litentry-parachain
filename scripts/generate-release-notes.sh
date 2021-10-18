#!/usr/bin/env bash
set -eo pipefail

err_report() {
    echo "Error on line $1"
}

trap 'err_report $LINENO' ERR

function usage() {
    echo "Usage: $0 path-to-output"
}

[ $# -ne 1 ] && (usage; exit 1)

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR"

# somehow `docker inspect` doesn't pull our litentry-parachain image sometimes
docker pull paritytech/ci-linux:production
docker pull "litentry/litentry-parachain:$RELEASE_TAG"

NODE_HASH=$(sha1sum litentry-collator/litentry-collator | awk '{print $1}')
NODE_RUSTC_VERSION=$(docker run --rm paritytech/ci-linux:production rustc --version)
NODE_BUILD_DOCKER_IMAGE_DIGEST=$(docker inspect paritytech/ci-linux:production  | grep paritytech/ci-linux@sha256 | sed 's/ *"//;s/"//')
NODE_BINARY_DOCKER_IMAGE_DIGEST=$(docker inspect "litentry/litentry-parachain:$RELEASE_TAG"  | grep litentry/litentry-parachain@sha256 | sed 's/ *"//;s/"//')
NODE_VERSION=$(grep version node/Cargo.toml | head -n1 | sed "s/'$//;s/.*'//")

SRTOOL_DIGEST_FILE=litentry-parachain-runtime/litentry-parachain-srtool-digest.json

RUNTIME_VERSION=$(grep spec_version runtime/src/lib.rs | sed 's/.*version: //;s/,//')

RUNTIME_COMPACT_SHA256=$(cat "$SRTOOL_DIGEST_FILE" | jq .runtimes.compact.sha256 | sed 's/"//g')
RUNTIME_COMPACT_PROPOSAL_HASH=$(cat "$SRTOOL_DIGEST_FILE" | jq .runtimes.compact.subwasm.proposal_hash | sed 's/"//g')
RUNTIME_COMPACT_PARACHAIN_UPGRADE_HASH=$(cat "$SRTOOL_DIGEST_FILE" | jq .runtimes.compact.subwasm.parachain_authorize_upgrade_hash | sed 's/"//g')

RUNTIME_COMPRESSED_SHA256=$(cat "$SRTOOL_DIGEST_FILE" | jq .runtimes.compressed.sha256 | sed 's/"//g')
RUNTIME_COMPRESSED_PROPOSAL_HASH=$(cat "$SRTOOL_DIGEST_FILE" | jq .runtimes.compressed.subwasm.proposal_hash | sed 's/"//g')
RUNTIME_COMPRESSED_PARACHAIN_UPGRADE_HASH=$(cat "$SRTOOL_DIGEST_FILE" | jq .runtimes.compressed.subwasm.parachain_authorize_upgrade_hash | sed 's/"//g')

# use <CODE> to decorate around the stuff and then replace it with `
# so that it's not executed as commands inside heredoc
cat << EOF > "$1"

# Release notes for litentry-parachain $RELEASE_TAG

## Client

version: **$NODE_VERSION**

### binary
- name: <CODE>litentry-collator<CODE>
- sha1sum hash: <CODE>$NODE_HASH<CODE>
- compiled with <CODE>$NODE_BUILD_DOCKER_IMAGE_DIGEST<CODE>
- rustc version: <CODE>$NODE_RUSTC_VERSION<CODE>

### docker image
- name: <CODE>litentry/litentry-parachain:$RELEASE_TAG<CODE>
- repo digest hash: <CODE>$NODE_BINARY_DOCKER_IMAGE_DIGEST<CODE>

## Runtime

version: **$RUNTIME_VERSION**

### compact
- sha256: <CODE>$RUNTIME_COMPACT_SHA256<CODE>
- proposal_hash: <CODE>$RUNTIME_COMPACT_PROPOSAL_HASH<CODE>
- parachain_authorize_upgrade_hash: <CODE>$RUNTIME_COMPACT_PARACHAIN_UPGRADE_HASH<CODE>

### compact-compressed
- sha256: <CODE>$RUNTIME_COMPRESSED_SHA256<CODE>
- proposal_hash: <CODE>$RUNTIME_COMPRESSED_PROPOSAL_HASH<CODE>
- parachain_authorize_upgrade_hash: <CODE>$RUNTIME_COMPRESSED_PARACHAIN_UPGRADE_HASH<CODE>

## Changes

EOF

sed -i 's/<CODE>/`/g' "$1"