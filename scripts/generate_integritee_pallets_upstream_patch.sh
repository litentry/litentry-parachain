#!/bin/bash

set -eo pipefail

cleanup() {
  rm -rf "$1"
  echo "cleaned up $1"
}

# This function generates a patch for the diffs between commit-A and commit-B
# of the upstream repo, where
# commit-A: the commit recorded in ./<TARGET_DIR>/upstream_commit
# commit-B: the HEAD commit of upstream master or a given commit
#
# A patch will be generated under ./<TARGET_DIR>/upstream.patch
generate_upstream_patch() {
	local TARGET_DIR=$1
	local UPSTREAM_NAME=$2
	local UPSTREAM_URL=$3
	# If $4 exists, it would be the target commit

	cd $TARGET_DIR

	if [ -f upstream_commit ]; then
		OLD_COMMIT=$(head -1 upstream_commit)
	else
		echo "Can't find upstream_commit file in $TARGET_DIR, quit"
		exit 1
	fi

	local tmp_dir=$(mktemp -d)
	cd "$tmp_dir"
	echo "cloning $UPSTREAM_URL ..."
	git clone -q $UPSTREAM_URL repo
	cd repo
	[ "" != "$4" ] && git checkout "$4"
	echo "generating patch ..."
	git diff $OLD_COMMIT HEAD > "$TARGET_DIR/$UPSTREAM_NAME.patch"
	git rev-parse --short HEAD > "$TARGET_DIR/upstream_commit"
	cleanup $tmp_dir
}

while getopts ":p:w:" opt; do
	case $opt in
		p)
			has_pallets=true
			pallets_commit=$OPTARG
			;;
		w)
			HAS_WORKER=true
			WORKER_COMMIT=$OPTARG
			;;
	esac
done

HAS_PALLETS=${has_pallets:-false}
PALLETS_COMMIT=${pallets_commit:-""}
HAS_WORKER=${has_worker:-false}
WORKER_COMMIT=${worker_commit:-""}

if [ $HAS_PALLETS == "fals" ] && [ $HAS_WORKER == "false" ]
	$HAS_PALLETS="true"
	$HAS_WORKER="true"
fi

UPSTREAM_PALLETS_URL="https://github.com/integritee-network/pallets"
UPSTREAM_WORKER_URL="https://github.com/integritee-network/worker"

ROOTDIR=$(git rev-parse --show-toplevel)
PALLETS_DIR="$ROOTDIR/pallets"
WORKER_DIR="$ROOTDIR/tee-worker"

if [ $HAS_PALLETS == "true" ] || [ $HAS_WORKER == "true" ]
then
	# From upstream pallets (https://github.com/integritee-network/pallets),
	# only 'teerex', 'teeracle', 'sidechain' and 'primitives' are taken in.
	if [ $HAS_PALLETS == "true"]; then
		generate_upstream_patch $PALLETS_DIR "upstream_pallets" $UPSTREAM_PALLETS_URL $PALLETS_COMMIT
	fi
	if [ $HAS_WORKER == "true"]; then
		generate_upstream_patch $WORKER_DIR "upstream_worker" $UPSTREAM_WORKER_URL $WORKER_COMMIT
	fi
	echo "======================================================================="
	echo "upstream_commit(s) are updated."
	echo "be sure to fetch the upstream to update the hashes of files."
	echo ""
	echo "upstream.patch(s) are generated, to apply it, RUN FROM $ROOTDIR:"
	if [ $HAS_PALLETS == "true"]; then
		echo "  git am -3 --directory=pallets < pallets/upstream.patch"
	fi
	if [ $HAS_WORKER == "true"]; then
		echo "  git am -3 --exclude=tee-worker/Cargo.lock --exclude=tee-worker/enclave-runtime/Cargo.lock --directory=tee-worker < tee-worker/upstream.patch"
	fi
	echo ""
	echo "after that, please:"
	echo "- pay special attention: "
	if [ $HAS_PALLETS == "true"]; then
		echo "  * ALL changes/conflicts from pallets/upstream.patch should ONLY apply into:"
		echo "    - pallets/(sidechain, teeracle, teerex, test-utils)"
		echo "    - primitives/(common, sidechain, teeracle, teerex)"
	fi
	if [ $HAS_WORKER == "true"]; then
		echo "  * ALL changes/conflicts from tee-worker/upstream.patch patch should ONLY apply into:"
		echo "    - tee-worker"
	fi
	echo "- resolve any conflicts"
	echo "- optionally update Cargo.lock file"
	echo "======================================================================="
	echo "***********************************************************************"
	echo "It is HIGHLY RECOMMENDED to apply patch and commit separately."
	echo "If trapped in git am session, don't panic. Just resolve any conflicts and commit as usual."
	echo "And abort the am session at the end: git am --abort"
	echo "***********************************************************************"
fi
