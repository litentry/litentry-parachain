#!/usr/bin/env bash
set -e

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT


TAG_COMMIT=`git rev-list --tags --max-count=1`
HEAD_COMMIT=`git rev-parse HEAD`
echo "TAG: $TAG_COMMIT"
echo "HEAD: $HEAD_COMMIT"

VERSION=$HEAD_COMMIT
if [ "$TAG_COMMIT" == "$HEAD_COMMIT" ]; then
    VERSION=`git describe --tags $TAG_COMMIT`
fi
GITUSER=litentry
GITREPO=litentry-parachain

# Build the image
echo "Building ${GITUSER}/${GITREPO}:latest docker image, hang on!"
time docker build -f ./docker/Dockerfile --build-arg RUSTC_WRAPPER= --build-arg PROFILE=release -t ${GITUSER}/${GITREPO}:latest .

# Show the list of available images for this repo
echo "Image is ready"
docker images | grep ${GITREPO}

echo -e "\nIf you just built version ${VERSION}, you may want to update your tag:"
echo " $ docker tag ${GITUSER}/${GITREPO}:$VERSION ${GITUSER}/${GITREPO}:${VERSION}"

popd
