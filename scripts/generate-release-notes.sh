#!/usr/bin/env bash
set -eo pipefail

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR"

NODE_HASH=$(sha1sum litentry-collator/litentry-collator)
NODE_RUSTC_VERSION=$(docker run --rm paritytech/ci-linux:production rustc --version)
NODE_BUILD_DOCKER_IMAGE_DIGEST=$(docker inspect paritytech/ci-linux:production  | grep paritytech/ci-linux@sha256 | sed 's/ *"//;s/"//')
NODE_BINARY_DOCKER_IMAGE_DIGEST=$(docker inspect litentry/litentry-parachain:$RELEASE_TAG  | grep litentry/litentry-parachain@sha256 | sed 's/ *"//;s/"//')
NODE_VERSION=$(grep version node/Cargo.toml | head -n1 | sed "s/'$//;s/.*'//")

SRTOOL_DIGEST_FILE=litentry-parachain-runtime/litentry-parachain-srtool-digest.json

RUNTIME_VERSION=$(grep spec_version runtime/src/lib.rs | sed 's/.*version: //;s/,//')

RUNTIME_COMPACT_SHA256=$(cat "$SRTOOL_DIGEST_FILE" | jq .runtimes.compact.sha256 | sed 's/"//g')
RUNTIME_COMPACT_PROPOSAL_HASH=$(cat "$SRTOOL_DIGEST_FILE" | jq .runtimes.compact.subwasm.proposal_hash | sed 's/"//g')
RUNTIME_COMPACT_PARACHAIN_UPGRADE_HASH=$(cat "$SRTOOL_DIGEST_FILE" | jq .runtimes.compact.subwasm.parachain_authorize_upgrade_hash | sed 's/"//g')

RUNTIME_COMPRESSED_SHA256=$(cat "$SRTOOL_DIGEST_FILE" | jq .runtimes.compressed.sha256 | sed 's/"//g')
RUNTIME_COMPRESSED_PROPOSAL_HASH=$(cat "$SRTOOL_DIGEST_FILE" | jq .runtimes.compressed.subwasm.proposal_hash | sed 's/"//g')
RUNTIME_COMPRESSED_PARACHAIN_UPGRADE_HASH=$(cat "$SRTOOL_DIGEST_FILE" | jq .runtimes.compressed.subwasm.parachain_authorize_upgrade_hash | sed 's/"//g')

cat << EOF

# Release notes for litentry-parachain $RELEASE_TAG

## Client

version: **$NODE_VERSION**

### binary
name: `litentry-collator`
sha1sum hash: `$NODE_HASH`
compiled with `$NODE_BUILD_DOCKER_IMAGE_DIGEST`
rustc version: `$NODE_RUSTC_VERSION`

### docker image
name: `litentry/litentry-parachain:$RELEASE_TAG`
repo digest hash: `$NODE_BINARY_DOCKER_IMAGE_DIGEST`

## Runtime

version: **$RUNTIME_VERSION**

### compact
- sha256: `$RUNTIME_COMPACT_SHA256`
- proposal_hash: `$RUNTIME_COMPACT_PROPOSAL_HASH`
- parachain_authorize_upgrade_hash: `$RUNTIME_COMPACT_PARACHAIN_UPGRADE_HASH`

### compact-compressed
- sha256: `$RUNTIME_COMPRESSED_SHA256`
- proposal_hash: `$RUNTIME_COMPRESSED_PROPOSAL_HASH`
- parachain_authorize_upgrade_hash: `$RUNTIME_COMPRESSED_PARACHAIN_UPGRADE_HASH`

## Changes

EOF