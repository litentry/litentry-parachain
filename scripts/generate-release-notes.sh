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

type=$2

export DOCKER_TAG=$(echo $RELEASE_TAG | cut -d'-' -f1 | sed 's/p/v/')

# helper functions to parse the type mask
is_client_release() {
  [ "${type:0:1}" = "1" ]
}

is_runtime_release() {
  [ "${type:1:1}" = "1" ]
}

is_worker_release() {
  [ "${type:2:1}" = "1" ]
}

is_enclave_release() {
  [ "${type:3:1}" = "1" ]
}

if is_client_release; then
  # base image used to build the node binary
  NODE_BUILD_BASE_IMAGE=$(grep FROM docker/Dockerfile | head -n1 | sed 's/^FROM //;s/ as.*//')

  # somehow `docker inspect` doesn't pull our litentry-parachain image sometimes
  docker pull "$NODE_BUILD_BASE_IMAGE"
  docker pull "litentry/litentry-parachain:$DOCKER_TAG"

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
if is_client_release; then
  echo "- [x] Parachain client" >> "$1"
else
  echo "- [ ] Parachain client" >> "$1"
fi
if is_runtime_release; then
  echo "- [x] Parachain runtime" >> "$1"
else
  echo "- [ ] Parachain runtime" >> "$1"
fi
if is_worker_release; then
  echo "- [x] TEE worker" >> "$1"
else
  echo "- [ ] TEE worker" >> "$1"
fi
if is_enclave_release; then
  echo "- [x] TEE enclave" >> "$1"
else
  echo "- [ ] TEE enclave" >> "$1"
fi
echo >> "$1"

# use <CODE> to decorate around the stuff and then replace it with `
# so that it's not executed as commands inside heredoc

cat << EOF >> "$1"
## Dependencies

<CODEBLOCK>
Substrate                    : $SUBSTRATE_DEP
Polkadot                     : $POLKADOT_DEP
Cumulus                      : $CUMULUS_DEP
<CODEBLOCK>

EOF

if is_client_release; then
  cat << EOF >> "$1"
## Parachain client

<CODEBLOCK>
version                      : $NODE_VERSION
name                         : $NODE_BIN
rustc                        : $NODE_RUSTC_VERSION
sha1sum                      : $NODE_SHA1SUM
docker image                 : litentry/litentry-parachain:$DOCKER_TAG
<CODEBLOCK>

EOF
fi

if is_runtime_release; then
  echo "## Parachain runtime" >> "$1"
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

if [ "$GENESIS_RELEASE" != "none" ]; then
  if [ "$2" = "runtime" ]; then
    echo "genesis release requires to build client"
    exit 1
  fi

  GENESIS_STATE_HASH=$(shasum litentry-collator/$GENESIS_RELEASE-genesis-state | awk '{print $1}')
  GENESIS_WASM_HASH=$(shasum litentry-collator/$GENESIS_RELEASE-genesis-wasm | awk '{print $1}')

  # double check that exported wasm matches what's written in chain-spec
  # intentionally use 'generate-prod' as chain type
  docker run --rm "litentry/litentry-parachain:$DOCKER_TAG" build-spec --chain=generate-$GENESIS_RELEASE --raw | \
  grep -F '"0x3a636f6465"' | sed 's/.*"0x3a636f6465": "//;s/",$//' | tr -d '\n' > /tmp/built-wasm

  if cmp /tmp/built-wasm litentry-collator/$GENESIS_RELEASE-genesis-wasm; then
    echo "genesis-wasm equal, all good."
    rm -f /tmp/built-wasm
  else
    echo "genesis-wasm unequal"
    exit 1
  fi
  cat << EOF >> "$1"
## Parachain genesis artefacts

<CODEBLOCK>
sha1sum of genesis state  : $GENESIS_STATE_HASH
sha1sum of genesis wasm   : $GENESIS_WASM_HASH
<CODEBLOCK>

EOF
fi

if is_worker_release; then
  WORKER_VERSION=$(grep version tee-worker/service/Cargo.toml | head -n1 | sed "s/'$//;s/.*'//")
  WORKER_BIN=$(grep name tee-worker/service/Cargo.toml | head -n1 | sed "s/'$//;s/.*'//")
  WORKER_RUSTC_VERSION=$(cd tee-worker && rustc --version)
  UPSTREAM_COMMIT=$(cat tee-worker/upstream_commit)
cat << EOF >> "$1"
## TEE worker

<CODEBLOCK>
version                      : $WORKER_VERSION
name                         : $WORKER_BIN
rustc                        : $WORKER_RUSTC_VERSION
sha1sum                      : $WORKER_SHA1SUM
upstream commit:             : $UPSTREAM_COMMIT
<CODEBLOCK>

EOF
fi

if is_enclave_release; then
  ENCLAVE_VERSION=$(grep spec_version tee-worker/app-libs/sgx-runtime/src/lib.rs | sed 's/.*version: //;s/,//')
cat << EOF >> "$1"
## TEE enclave

<CODEBLOCK>
version                      : $ENCLAVE_VERSION
sha1sum                      : $ENCLAVE_SHA1SUM
mrenclave                    : $MRENCLAVE
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

Raw diff: [$DIFF_TAG...$RELEASE_TAG]($REPO/compare/$DIFF_TAG...$RELEASE_TAG)

Details:

EOF

  labels=("C0-breaking" "C1-noteworthy")

  git log --no-merges --abbrev-commit --pretty="format:%h|%s%n" "$DIFF_TAG..$RELEASE_TAG" | grep -v "^$" | while read -r f; do
    commit=$(echo "$f" | cut -d'|' -f1)
    desc=$(echo "$f" | cut -d'|' -f2)
    output="- [\`$commit\`]($REPO/commit/$commit) $desc"
    
    for ((i=0; i<${#labels[@]}; i++)); do
      label=$(gh pr list --search "$commit" --label "${labels[i]}" --state merged)
      [ -n "$label" ] && output+=" $REPO/labels/${labels[i]}"
    done
    
    echo "$output" >> "$1"
  done
fi