#!/bin/bash

set -eo pipefail

ROOTDIR=$(git rev-parse --show-toplevel)
UPSTREAM_URL_PREFIX="https://github.com/integritee-network"

function cleanup() {
   rm -rf "$1"
   echo "cleaned up $1"
}

function usage() {
	echo "Usage:"
    echo "For pallets: $0 p pallets-new-commit"
    echo "For worker : $0 w worker-new-commit"
}

# This function generates 9 patches for the diffs between commit-A and commit-B
# of the upstream repo, where
# commit-A: the commit recorded in ./<TARGET_DIR>/upstream_commit
# commit-B: the HEAD commit of upstream master or a given commit
#
# Patches will be generated under ./<TARGET_DIR>/
function generate_worker_patch() {
	local TARGET="worker"
	local TARGET_DIR="$ROOTDIR/$TARGET"
    echo "TARGET: $1"
    echo "TARGET_DIR: $TARGET_DIR"

	if [[ $TARGET == "worker" ]]
	then
		TARGET_DIR="$ROOTDIR/tee-worker"
	fi

	local UPSTREAM_URL="$UPSTREAM_URL_PREFIX/$TARGET"
    echo "UPSTREAM_URL: $UPSTREAM_URL"

	cd "$TARGET_DIR"

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
	echo "cloning ""$UPSTREAM_URL"" to $tmp_dir"
	git clone -q "$UPSTREAM_URL" repo
	cd repo
	[ "" != "$NEW_COMMIT" ] && git checkout "$NEW_COMMIT"
	echo "generating patch ..."
	git diff "$OLD_COMMIT" HEAD > "$TARGET_DIR/upstream.patch"
	git rev-parse --short HEAD > "$TARGET_DIR/upstream_commit"
	 
    # Clean up TMP DIR
	cleanup $tmp_dir

	echo
}

# This function generates a patch for the diffs between commit-A and commit-B
# of pallets repo
# -> ./<TARGET_DIR>/pallets_xxx.patch
# -> ./<TARGET_DIR>/primitives_xxx.patch
function generate_pallets_patch() {
	local TARGET='pallets'
	local TARGET_DIR="$ROOTDIR/$TARGET"
    echo "TARGET: $TARGET"
    echo "TARGET_DIR: $TARGET_DIR"

	local UPSTREAM_URL="$UPSTREAM_URL_PREFIX/$TARGET"
    echo "UPSTREAM_URL: $UPSTREAM_URL"

	cd "$TARGET_DIR"

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
	echo "cloning ""$UPSTREAM_URL"" to $tmp_dir"
	git clone -q "$UPSTREAM_URL" repo
	cd repo
	[ "" != "$NEW_COMMIT" ] && git checkout "$NEW_COMMIT"
	echo ">>> generating patch ..."

	local pallets=("parentchain" "sidechain" "teeracle" "teerex" "test-utils")
    for p in "${pallets[@]}"; do
        echo "generating $p.patch"
		git diff "$OLD_COMMIT" HEAD -- "$p" > "$TARGET_DIR/pallets_$p.patch"
    done

	# primitives
    local primitives=("sidechain" "teeracle" "teerex" "common")
    for p in "${primitives[@]}"; do
        echo "generating primitives_$p.patch"
		git diff "$OLD_COMMIT" HEAD -- primitives/"$p" > "$TARGET_DIR/primitives_$p.patch"
    done

	echo ">>> generating patch done."

	git rev-parse --short HEAD > "$TARGET_DIR/upstream_commit"
	 
    # Clean up TMP DIR
    cleanup $tmp_dir

	echo
}

function apply_pallets_tips() {
	echo "======================================================================="
	echo "upstream_commit(s) are updated."
	echo "upstream.patch(s) are generated."
	echo "To apply it, RUN FROM $ROOTDIR:"
    echo "  pallets   > git apply -p1 -3 --directory=pallets $ROOTDIR/pallets/pallets_xxx.patch"
	echo "  primitives> git apply -p1 -3 $ROOTDIR/pallets/primitives_xxx.patch"

    echo ""
	echo "after that, please:"
	echo "- pay special attention: "
    echo "  * ALL changes/conflicts from pallets/upstream.patch should ONLY apply into:"
    echo "    - pallets/(parentchain, sidechain, teeracle, teerex, test-utils)"
    echo "    - primitives/(common, sidechain, teeracle, teerex)"
}

function apply_woker_tips() {
	echo "======================================================================="
	echo "upstream_commit(s) are updated."
	echo "upstream.patch(s) are generated."
	echo "To apply it, RUN FROM $ROOTDIR:"
    echo "  git am -3 --exclude=tee-worker/Cargo.lock --exclude=tee-worker/enclave-runtime/Cargo.lock --directory=tee-worker < tee-worker/upstream.patch"

	echo ""
	echo "after that, please:"
	echo "- pay special attention: "
    echo "  * ALL changes/conflicts from tee-worker/upstream.patch patch should ONLY apply into:"
    echo "    - tee-worker"

    echo "- resolve any conflicts"
	echo "- optionally update Cargo.lock file"
    echo "- apply the changes to $ROOTDIR/.github/workflows/tee-worker-ci.yml"
}

OPT="$1"
case "$OPT" in
    p)
        has_pallets=true ;;
    w)
        has_worker=true ;;
    *)
        usage; exit 1 ;;
esac

if [ -z "$2" ]; then
    usage; exit 1
fi
NEW_COMMIT=$2

HAS_PALLETS=${has_pallets:-false}
HAS_WORKER=${has_worker:-false}

if [ $HAS_PALLETS == "true" ] && [ $HAS_WORKER == "true" ]
then
    echo "***********************************************************************"
    echo "It is HIGHLY RECOMMENDED to apply patch and commit separately."
    echo "If trapped in git am session, don't panic. Just resolve any conflicts and commit as usual."
    echo "And abort the am session at the end: git am --abort"
    echo "***********************************************************************"

    exit 1
fi

if [[ $HAS_PALLETS == "true" ]]
then
	generate_pallets_patch "$@"
    apply_pallets_tips
fi

if [[ $HAS_WORKER == "true" ]]
then
    generate_worker_patch "$@"
    apply_woker_tips
fi