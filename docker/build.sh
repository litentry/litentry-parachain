#!/usr/bin/env bash
set -e
pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT

TAG_COMMIT=`git rev-list --tags --max-count=1`
HEAD_COMMIT=`git rev-parse HEAD`
echo "TAG  commit: $TAG_COMMIT"
echo "HEAD commit: $HEAD_COMMIT"

if [ "$TAG_COMMIT" == "$HEAD_COMMIT" ]; then
    VERSION=`git describe --tags $TAG_COMMIT`
else
    VERSION=`git rev-parse --short HEAD`
fi

echo "VERSION: $VERSION"

GITUSER=litentry
GITREPO=litentry-parachain

# Build the image
echo "Building ${GITUSER}/${GITREPO}:latest docker image, hang on!"
time docker build --rm -f ./docker/Dockerfile --build-arg PROFILE=release -t ${GITUSER}/${GITREPO}:latest .

# Tag it with VERSION
docker tag ${GITUSER}/${GITREPO}:latest ${GITUSER}/${GITREPO}:${VERSION}

# Show the list of available images for this repo
echo "Image is ready"
docker images | grep ${GITREPO}

popd
