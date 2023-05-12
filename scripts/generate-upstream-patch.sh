#!/bin/bash

set -eo pipefail

cleanup() {
	rm -rf "$1"
	echo "cleaned up $1"
}

print_help() {
	echo "Usage:"
	echo "$0 [-h] [-p <tag|branch|commit-hash>] [-w <tag|branch|commit-hash>]"
	echo ""
	echo "without any parameter, the script will generate upstream patch for both pallets and tee-worker"
	echo "	-h print this message"
	echo "	-p specify the tag|branch|commit-hash for upstream pallets"
	echo "	-w specify the tag|branch|commit-hash for upstream worker"
}

check_upstream() {
	local TARGET=$1
	local UPSTREAM_URL="$UPSTREAM_URL_PREFIX/$TARGET"
	if [ "$(git remote get-url upstream_$TARGET 2>/dev/null)" != "$UPSTREAM_URL" ]; then
		git remote add upstream_$TARGET $UPSTREAM_URL
	fi
}

# This function generates a patch for the diffs between commit-A and commit-B
# of the upstream repo, where
# commit-A: the commit recorded in ./<TARGET_DIR>/upstream_commit
# commit-B: the HEAD commit of upstream master or a given commit
#
# A patch will be generated under ./<TARGET_DIR>/upstream.patch
generate_upstream_patch() {
	local TARGET=$1
	local TARGET_DIR="$ROOTDIR/$TARGET"
	if [[ $TARGET == "worker" ]]
	then
		TARGET_DIR="$ROOTDIR/tee-worker"
	fi
	local UPSTREAM_URL="$UPSTREAM_URL_PREFIX/$TARGET"
	# If $2 exists, it would be the target commit

	cd $TARGET_DIR

	if [ -f upstream_commit ]; then
		OLD_COMMIT=$(head -1 upstream_commit)
	else
		echo "Can't find upstream_commit file in $TARGET_DIR, quit"
		exit 1
	fi

	echo "fetch upstream_$TARGET"
	git fetch -q "upstream_$TARGET"

	local tmp_dir
	tmp_dir=$(mktemp -d)
	cd "$tmp_dir"
	echo "cloning $UPSTREAM_URL to $tmp_dir"
	git clone -q $UPSTREAM_URL repo
	cd repo
	[ "" != "$2" ] && git checkout "$2"
	echo "generating patch ..."
	git diff $OLD_COMMIT HEAD > "$TARGET_DIR/upstream.patch"
	git rev-parse --short HEAD > "$TARGET_DIR/upstream_commit"
	cleanup $tmp_dir
	echo
}

while getopts ":p:w:h" opt; do
	case $opt in
		p)
			has_pallets=true
			pallets_commit=$OPTARG
			;;
		w)
			has_worker=true
			worker_commit=$OPTARG
			;;
		h)
			print_help
			exit 0
			;;
		*)
			echo "unknown args"
			exit 1
			;;
	esac
done

HAS_PALLETS=${has_pallets:-false}
PALLETS_COMMIT=${pallets_commit:-""}
HAS_WORKER=${has_worker:-false}
WORKER_COMMIT=${worker_commit:-""}

if [ $HAS_PALLETS == "false" ] && [ $HAS_WORKER == "false" ]
then
	HAS_PALLETS=true
	HAS_WORKER=true
	echo "will update both pallets and worker upstream"
	echo
fi

UPSTREAM_URL_PREFIX="https://github.com/integritee-network"
ROOTDIR=$(git rev-parse --show-toplevel)

if [ "$HAS_PALLETS" == "true" ]
then
	check_upstream "pallets"
fi

if [ "$HAS_WORKER" == "true" ]
then
	check_upstream "worker"
fi


if [ "$HAS_PALLETS" == "true" ] || [ "$HAS_WORKER" == "true" ]
then
	# From upstream pallets (https://github.com/integritee-network/pallets),
	# only 'teerex', 'teeracle', 'sidechain' and 'primitives' are taken in.
	if [ "$HAS_PALLETS" == "true" ]
	then
		generate_upstream_patch "pallets" $PALLETS_COMMIT
	fi
	if [ "$HAS_WORKER" == "true" ]
	then
		generate_upstream_patch "worker" $WORKER_COMMIT
	fi
	echo "======================================================================="
	echo "upstream_commit(s) are updated."
	echo "upstream.patch(s) are generated."
	echo "To apply it, RUN FROM $ROOTDIR:"
	if [ "$HAS_PALLETS" == "true" ]
	then
		echo "  git am -3 --directory=pallets < pallets/upstream.patch"
	fi
	if [ "$HAS_WORKER" == "true" ]
	then
		echo "  git am -3 --exclude=tee-worker/Cargo.lock --exclude=tee-worker/enclave-runtime/Cargo.lock --directory=tee-worker < tee-worker/upstream.patch"
	fi
	echo ""
	echo "after that, please:"
	echo "- pay special attention: "
	if [ "$HAS_PALLETS" == "true" ]
	then
		echo "  * ALL changes/conflicts from pallets/upstream.patch should ONLY apply into:"
		echo "    - pallets/(parentchain, sidechain, teeracle, teerex, test-utils)"
		echo "    - primitives/(common, sidechain, teeracle, teerex)"
	fi
	if [ "$HAS_WORKER" == "true" ]
	then
		echo "  * ALL changes/conflicts from tee-worker/upstream.patch patch should ONLY apply into:"
		echo "    - tee-worker"
	fi
	echo "- resolve any conflicts"
	echo "- optionally update Cargo.lock file"
	if [ "$HAS_WORKER" == "true" ]
	then
		echo "- apply the changes to $ROOTDIR/.github/workflows/tee-worker-ci.yml"
	fi
	echo "======================================================================="
	if [ "$HAS_PALLETS" == "true" ] && [ "$HAS_WORKER" == "true" ]
	then
		echo "***********************************************************************"
		echo "It is HIGHLY RECOMMENDED to apply patch and commit separately."
		echo "If trapped in git am session, don't panic. Just resolve any conflicts and commit as usual."
		echo "And abort the am session at the end: git am --abort"
		echo "***********************************************************************"
	fi
fi
