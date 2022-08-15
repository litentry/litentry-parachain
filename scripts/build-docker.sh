#!/usr/bin/env bash
set -eo pipefail

function usage() {
  echo "Usage:   $0 release|production [docker-tag] [build-args]"
}

[ $# -lt 1 ] && (usage; exit 1)

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR"

PROFILE="$1"
TAG="$2"
ARGS="$3"

NOCACHE_FLAG=

case "$PROFILE" in
    release)
        ;;
    production)
        NOCACHE_FLAG="--no-cache" ;;
    *)
        usage; exit 1 ;;
esac

if [ -z "$TAG" ]; then
    TAG_COMMIT=`git rev-list --tags --max-count=1`
    HEAD_COMMIT=`git rev-parse HEAD`
    echo "TAG  commit: $TAG_COMMIT"
    echo "HEAD commit: $HEAD_COMMIT"

    if [ "$TAG_COMMIT" == "$HEAD_COMMIT" ]; then
        TAG=`git describe --tags $TAG_COMMIT`
    else
        TAG=`git rev-parse --short HEAD`
    fi
fi

echo "PROFILE: $PROFILE"
echo "TAG: $TAG"
echo "ARGS: $ARGS"

GITUSER=litentry
GITREPO=litentry-parachain

# Build the image
echo "------------------------------------------------------------"
echo "Building ${GITUSER}/${GITREPO}:${TAG} docker image ..."
docker build ${NOCACHE_FLAG} --pull -f ./docker/Dockerfile \
    --build-arg PROFILE="$PROFILE" \
    --build-arg BUILD_ARGS="$ARGS" \
    -t ${GITUSER}/${GITREPO}:${TAG} .

# Tag it with latest if no tag parameter was provided
[ -z "$2" ] && docker tag ${GITUSER}/${GITREPO}:${TAG} ${GITUSER}/${GITREPO}:latest

# Show the list of available images for this repo
echo "Image is ready"
echo "------------------------------------------------------------"
docker images | grep ${GITREPO}
