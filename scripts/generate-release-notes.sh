#!/usr/bin/env bash
set -eo pipefail

err_report() {
    echo "Error on line $1"
}

trap 'err_report $LINENO' ERR

function usage() {
    echo "Usage: $0 path-to-output diff-tag"
}

[ $# -gt 2 ] && (usage; exit 1)

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR"

REPO=https://github.com/litentry/litentry-parachain

# base image used to build the node binary
NODE_BUILD_BASE_IMAGE=$(grep FROM docker/Dockerfile | head -n1 | sed 's/^FROM //;s/ as.*//')

# somehow `docker inspect` doesn't pull our litentry-parachain image sometimes
docker pull "$NODE_BUILD_BASE_IMAGE"
docker pull "litentry/litentry-parachain:$RELEASE_TAG"

NODE_VERSION=$(grep version node/Cargo.toml | head -n1 | sed "s/'$//;s/.*'//")
NODE_BIN=litentry-collator
NODE_SHA1SUM=$(shasum litentry-collator/"$NODE_BIN" | awk '{print $1}')
if [ -f rust-toolchain.toml ]; then
  NODE_RUSTC_VERSION=$(rustc --version)
else
  NODE_RUSTC_VERSION=$(docker run --rm "$NODE_BUILD_BASE_IMAGE" rustup default nightly 2>&1 | grep " installed" | sed 's/.*installed - //')
fi

SRTOOL_DIGEST_FILE=litentry-parachain-runtime/litentry-parachain-srtool-digest.json

RUNTIME_VERSION=$(grep spec_version runtime/src/lib.rs | sed 's/.*version: //;s/,//')
RUNTIME_COMPRESSED_SIZE=$(cat "$SRTOOL_DIGEST_FILE" | jq .runtimes.compressed.size | sed 's/"//g')
RUNTIME_RUSTC_VERSION=$(cat "$SRTOOL_DIGEST_FILE" | jq .rustc | sed 's/"//g')
RUNTIME_COMPRESSED_SHA256=$(cat "$SRTOOL_DIGEST_FILE" | jq .runtimes.compressed.sha256 | sed 's/"//g')
RUNTIME_COMPRESSED_BLAKE2=$(cat "$SRTOOL_DIGEST_FILE" | jq .runtimes.compressed.blake2_256 | sed 's/"//g')
RUNTIME_COMPRESSED_SET_CODE_HASH=$(cat "$SRTOOL_DIGEST_FILE" | jq .runtimes.compressed.subwasm.proposal_hash | sed 's/"//g')
RUNTIME_COMPRESSED_AUTHORIZE_UPGRADE_HASH=$(cat "$SRTOOL_DIGEST_FILE" | jq .runtimes.compressed.subwasm.parachain_authorize_upgrade_hash | sed 's/"//g')

SUBSTRATE_DEP=$(grep sp-core node/Cargo.toml | sed 's/.*branch = "//;s/".*//')
CUMULUS_DEP=$(grep cumulus-client-cli node/Cargo.toml | sed 's/.*branch = "//;s/".*//')
POLKADOT_DEP=$(grep polkadot-cli node/Cargo.toml | sed 's/.*branch = "//;s/".*//')

TAB="$(printf '\t')"

# use <CODE> to decorate around the stuff and then replace it with `
# so that it's not executed as commands inside heredoc
cat << EOF > "$1"

## Client

<CODEBLOCK>
version                      : $NODE_VERSION
name                         : $NODE_BIN
rustc                        : $NODE_RUSTC_VERSION
sha1sum                      : $NODE_SHA1SUM
docker image                 : litentry/litentry-parachain:$RELEASE_TAG
<CODEBLOCK>

## Runtime

<CODEBLOCK>
version                      : $RUNTIME_VERSION
size                         : $RUNTIME_COMPRESSED_SIZE
rustc                        : $RUNTIME_RUSTC_VERSION
sha256                       : $RUNTIME_COMPRESSED_SHA256
blake2-256                   : $RUNTIME_COMPRESSED_BLAKE2
proposal (setCode)           : $RUNTIME_COMPRESSED_SET_CODE_HASH
proposal (authorizeUpgrade)  : $RUNTIME_COMPRESSED_AUTHORIZE_UPGRADE_HASH
<CODEBLOCK>

## Dependencies

<CODEBLOCK>
Substrate                    : $SUBSTRATE_DEP
Polkadot                     : $POLKADOT_DEP
Cumulus                      : $CUMULUS_DEP
<CODEBLOCK>

EOF

if [ "$GENESIS_RELEASE" = "true" ]; then
  GENESIS_STATE_HASH=$(shasum litentry-collator/litentry-genesis-state | awk '{print $1}')
  GENESIS_WASM_HASH=$(shasum litentry-collator/litentry-genesis-wasm | awk '{print $1}')

  # double check that exported wasm matches what's written in chain-spec
  # intentionally use 'generate-prod' as chain type
  docker run --rm "litentry/litentry-parachain:$RELEASE_TAG" build-spec --chain=generate-prod --raw | \
  grep -F '"0x3a636f6465"' | sed 's/.*"0x3a636f6465": "//;s/",$//' | tr -d '\n' > /tmp/built-wasm

  if cmp /tmp/built-wasm litentry-collator/litentry-genesis-wasm; then
    echo "genesis-wasm equal, all good."
    rm -f /tmp/built-wasm
  else
    echo "genesis-wasm unequal"
    exit 1
  fi
  cat << EOF >> "$1"
## Genesis artefacts

<CODEBLOCK>
sha1sum of genesis state  : $GENESIS_STATE_HASH
sha1sum of genesis wasm   : $GENESIS_WASM_HASH
<CODEBLOCK>

EOF
fi

# restore ``` in markdown doc
# use -i.bak for compatibility for MacOS and Linux
sed -i.bak 's/<CODEBLOCK>/```/g' "$1"
rm -f "$1.bak"

# if we have a diff-tag, list the changes inbetween
DIFF_TAG="$2"

if [ -z "$DIFF_TAG" ]; then
  echo "Nothing to compare"
  exit 0
elif [ "$DIFF_TAG" = "$RELEASE_TAG" ]; then
  echo "Skip compare to itself"
  exit 0
else
  cat << EOF >> "$1"
## Changes

Raw diff: [$DIFF_TAG...$RELEASE_TAG](https://github.com/litentry/litentry-parachain/compare/$DIFF_TAG...$RELEASE_TAG)

Details:

EOF

  git log --no-merges --abbrev-commit --pretty="format:%h|%s" $DIFF_TAG..$RELEASE_TAG | while read -r f; do
    commit=$(echo "$f" | cut -d'|' -f1)
    desc=$(echo "$f" | cut -d'|' -f2)
    echo -e "- [\`$commit\`]($REPO/commit/$commit) $desc" >> "$1"
  done
fi
