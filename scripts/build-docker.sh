#!/usr/bin/env bash
set -eo pipefail

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR"

TAG="$1"
ARGS="$2"

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

echo "TAG: $TAG"
echo "ARGS: $ARGS"

GITUSER=litentry
GITREPO=litentry-parachain

# Build the image
echo "------------------------------------------------------------"
echo "Building ${GITUSER}/${GITREPO}:${TAG} docker image ..."
docker build --rm --no-cache --pull -f ./docker/Dockerfile \
    --build-arg BUILD_ARGS="$ARGS" \
    -t ${GITUSER}/${GITREPO}:${TAG} .

# Tag it with latest if no tag parameter was provided
[ -z "$1" ] && docker tag ${GITUSER}/${GITREPO}:${TAG} ${GITUSER}/${GITREPO}:latest

# Show the list of available images for this repo
echo "Image is ready"
echo "------------------------------------------------------------"
docker images | grep ${GITREPO}
