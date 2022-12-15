#!/usr/bin/env bash
set -eo pipefail

err_report() {
  echo "Error on line $1"
}

trap 'err_report $LINENO' ERR

function usage() {
  echo "Usage: $0 path-to-output release-type [diff-tag]"
}

[ $# -ne 2 ] && [ $# -ne 3 ] && (usage; exit 1)

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR"

REPO=https://github.com/litentry/litentry-parachain

if [ "$2" != "runtime" ]; then
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
fi

SUBSTRATE_DEP=$(grep -F 'https://github.com/paritytech/substrate' node/Cargo.toml | head -n1 | sed 's/.*branch = "//;s/".*//')
POLKADOT_DEP=$(grep -F 'https://github.com/paritytech/polkadot' node/Cargo.toml | head -n1 | sed 's/.*branch = "//;s/".*//')
CUMULUS_DEP=$(grep -F 'https://github.com/paritytech/cumulus' node/Cargo.toml | head -n1 | sed 's/.*branch = "//;s/".*//')

echo > "$1"
echo "## This is a release for:" >> "$1"
if [ "$2" != "runtime" ]; then
  echo "- [x] Client" >> "$1"
else
  echo "- [ ] Client" >> "$1"
fi
if [ "$2" != "client" ]; then
  echo "- [x] Runtime" >> "$1"
else
  echo "- [ ] Runtime" >> "$1"
fi
echo >> "$1"

# use <CODE> to decorate around the stuff and then replace it with `
# so that it's not executed as commands inside heredoc

if [ "$2" != "runtime" ]; then
  cat << EOF >> "$1"
## Client

<CODEBLOCK>
version                      : $NODE_VERSION
name                         : $NODE_BIN
rustc                        : $NODE_RUSTC_VERSION
sha1sum                      : $NODE_SHA1SUM
docker image                 : litentry/litentry-parachain:$RELEASE_TAG
<CODEBLOCK>

EOF
fi

if [ "$2" != "client" ]; then
  echo "## Runtime" >> "$1"
  for CHAIN in litmus rococo litentry; do
    SRTOOL_DIGEST_FILE=$CHAIN-parachain-runtime/$CHAIN-parachain-srtool-digest.json
    RUNTIME_VERSION=$(grep spec_version runtime/$CHAIN/src/lib.rs | sed 's/.*version: //;s/,//')
    RUNTIME_COMPRESSED_SIZE=$(cat "$SRTOOL_DIGEST_FILE" | jq .runtimes.compressed.size | sed 's/"//g')
    RUNTIME_RUSTC_VERSION=$(cat "$SRTOOL_DIGEST_FILE" | jq .rustc | sed 's/"//g')
    RUNTIME_COMPRESSED_SHA256=$(cat "$SRTOOL_DIGEST_FILE" | jq .runtimes.compressed.sha256 | sed 's/"//g')
    RUNTIME_COMPRESSED_BLAKE2=$(cat "$SRTOOL_DIGEST_FILE" | jq .runtimes.compressed.blake2_256 | sed 's/"//g')
    RUNTIME_COMPRESSED_SET_CODE_HASH=$(cat "$SRTOOL_DIGEST_FILE" | jq .runtimes.compressed.subwasm.proposal_hash | sed 's/"//g')
    RUNTIME_COMPRESSED_AUTHORIZE_UPGRADE_HASH=$(cat "$SRTOOL_DIGEST_FILE" | jq .runtimes.compressed.subwasm.parachain_authorize_upgrade_hash | sed 's/"//g')
    cat << EOF >> "$1"
### $CHAIN

<CODEBLOCK>
version                      : $RUNTIME_VERSION
size                         : $RUNTIME_COMPRESSED_SIZE
rustc                        : $RUNTIME_RUSTC_VERSION
sha256                       : $RUNTIME_COMPRESSED_SHA256
blake2-256                   : $RUNTIME_COMPRESSED_BLAKE2
proposal (setCode)           : $RUNTIME_COMPRESSED_SET_CODE_HASH
proposal (authorizeUpgrade)  : $RUNTIME_COMPRESSED_AUTHORIZE_UPGRADE_HASH
<CODEBLOCK>

EOF
  done
fi

cat << EOF >> "$1"
## Dependencies

<CODEBLOCK>
Substrate                    : $SUBSTRATE_DEP
Polkadot                     : $POLKADOT_DEP
Cumulus                      : $CUMULUS_DEP
<CODEBLOCK>

EOF

if [ "$GENESIS_RELEASE" != "none" ]; then
  if [ "$2" = "runtime" ]; then
    echo "genesis release requires to build client"
    exit 1
  fi

  GENESIS_STATE_HASH=$(shasum litentry-collator/$GENESIS_RELEASE-genesis-state | awk '{print $1}')
  GENESIS_WASM_HASH=$(shasum litentry-collator/$GENESIS_RELEASE-genesis-wasm | awk '{print $1}')

  # double check that exported wasm matches what's written in chain-spec
  # intentionally use 'generate-prod' as chain type
  docker run --rm "litentry/litentry-parachain:$RELEASE_TAG" build-spec --chain=generate-$GENESIS_RELEASE --raw | \
  grep -F '"0x3a636f6465"' | sed 's/.*"0x3a636f6465": "//;s/",$//' | tr -d '\n' > /tmp/built-wasm

  if cmp /tmp/built-wasm litentry-collator/$GENESIS_RELEASE-genesis-wasm; then
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
DIFF_TAG="$3"

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

  # use %n as a workaround otherwise there's no newline after the whole body
  git log --no-merges --abbrev-commit --pretty="format:%h|%s%n" $DIFF_TAG..$RELEASE_TAG | grep -v "^$" | while read -r f; do
    commit=$(echo "$f" | cut -d'|' -f1)
    desc=$(echo "$f" | cut -d'|' -f2)
    echo -e "- [\`$commit\`]($REPO/commit/$commit) $desc" >> "$1"
  done
fi
