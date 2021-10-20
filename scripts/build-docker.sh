#!/usr/bin/env bash
set -eo pipefail

ROOTDIR=$(git rev-parse --show-toplevel)
cd "$ROOTDIR"

VERSION="$1"
FEATURES="$2"

if [ -z "$VERSION" ]; then
    TAG_COMMIT=`git rev-list --tags --max-count=1`
    HEAD_COMMIT=`git rev-parse HEAD`
    echo "TAG  commit: $TAG_COMMIT"
    echo "HEAD commit: $HEAD_COMMIT"

    if [ "$TAG_COMMIT" == "$HEAD_COMMIT" ]; then
        VERSION=`git describe --tags $TAG_COMMIT`
    else
        VERSION=`git rev-parse --short HEAD`
    fi
fi

echo "VERSION: $VERSION"
echo "FEATURES: $FEATURES"

GITUSER=litentry
GITREPO=litentry-parachain

# Build the image
echo "Building ${GITUSER}/${GITREPO}:${VERSION} docker image, hang on!"
docker build --rm --no-cache --pull -f ./docker/Dockerfile \
    --build-arg PROFILE=release \
    --build-arg FEATURES="$FEATURES" \
    -t ${GITUSER}/${GITREPO}:${VERSION} .

# Tag it with latest if no arg was provided
[ -z "$1" ] && docker tag ${GITUSER}/${GITREPO}:${VERSION} ${GITUSER}/${GITREPO}:latest

# Show the list of available images for this repo
echo "Image is ready"
docker images | grep ${GITREPO}
