#!/bin/bash

set -eo pipefail

cleanup() {
  rm -rf "$1"
  echo "cleaned up $1"
}

# This script generates a patch for the diffs between commit-A and commit-B
# of the upstream repo (https://github.com/integritee-network/pallets), where
# commit-A: the commit recorded in ./pallets/upstream_commit
# commit-B: the HEAD commit of upstream master or a given commit
#
# From upstream repo (https://github.com/integritee-network/pallets),
# only 'teerex', 'teeracle', 'sidechain' and 'primitives' are taken in.

# The patch will be generated under ./pallets/upstream.patch

UPSTREAM="https://github.com/integritee-network/pallets"
ROOTDIR=$(git rev-parse --show-toplevel)
ROOTDIR="$ROOTDIR/pallets"
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
echo "upstream.patch is generated, to apply it, run:"
echo "  # git am -3 --include=test-utils --include=teerex --include=teeracle --include=sidechain --include=primitives --directory=pallets < upstream.patch"
echo "after that, please:"
echo "- resolve any conflicts"
echo "- optionally update Cargo.lock file"
echo "======================================================================="
echo "If trapped in git am session, don't panic. Just resolve any conflicts and commit as usual."
echo "And abort the am session at the end: git am --abort"
