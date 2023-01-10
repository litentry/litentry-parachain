#!/bin/bash

set -eo pipefail

cleanup() {
  rm -rf "$1"
  echo "cleaned up $1"
}

# This script generates a patch for the diffs between commit-A and commit-B
# of the upstream repo (https://github.com/integritee-network/worker), where
# commit-A: the commit recorded in tee-worker/upstream_commit
# commit-B: the HEAD commit of upstream master or a given commit
#
# The patch will be generated under tee-worker/upstream.patch

UPSTREAM="https://github.com/integritee-network/worker"
ROOTDIR0=$(git rev-parse --show-toplevel)
ROOTDIR="$ROOTDIR0/tee-worker"
cd "$ROOTDIR"

if [ -f upstream_commit ]; then
  OLD_COMMIT=$(head -1 upstream_commit)
else
  echo "Can't find upstream_commit file, quit"
  exit 1
fi

if [ "$(git remote get-url upstream 2>/dev/null)" != "$UPSTREAM" ]; then
  echo "please set your upstream to $UPSTREAM"
  echo "e.g.: git remote add upstream $UPSTREAM"
  exit 1
else
  git fetch -q upstream
fi

TMPDIR=$(mktemp -d)
trap 'cleanup "$TMPDIR"' ERR EXIT INT

cd "$TMPDIR"
echo "cloning $UPSTREAM ..."
git clone -q "$UPSTREAM" worker
cd worker
[ ! -z "$1" ] && git checkout "$1"
echo "generating patch ..."
git diff $OLD_COMMIT HEAD > "$ROOTDIR/upstream.patch"
git rev-parse --short HEAD > "$ROOTDIR/upstream_commit"

echo "======================================================================="
echo "upstream_commit is updated."
echo "be sure to fetch the upstream to update the hashes of files."
echo ""
echo "upstream.patch is generated, to apply it, RUN FROM $ROOTDIR0:"
echo "  git am -3 --exclude=tee-worker/Cargo.lock --exclude=tee-worker/enclave-runtime/Cargo.lock --directory=tee-worker < tee-worker/upstream.patch"
echo ""
echo "after that, please:"
echo "- resolve any conflicts"
echo "- optionally update both Cargo.lock files"
echo "- apply the changes to <root-dir>/.github/workflows/tee-worker-ci.yml"
echo "======================================================================="
echo "If trapped in git am session, don't panic. Just resolve any conflicts and commit as usual."
echo "And abort the am session at the end: git am --abort"
