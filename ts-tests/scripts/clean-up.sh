#!/bin/sh

basedir=$(dirname "$0")
cd "$basedir/../../docker/generated-dev"

docker images

echo "stop and remove docker containers..."
docker-compose rm -f -s -v

echo "remove docker volumes..."
docker volume ls | grep generated-dev | sed 's/local *//' | xargs docker volume rm

echo "remove dangling docker images if any..."
[ -z "$(docker images --filter=dangling=true -q)" ] || docker rmi -f $(docker images --filter=dangling=true -q)

echo "keep litentry/litentry-parachain:latest while removing other tags..."
docker rmi -f $(docker images litentry/litentry-parachain --format "{{.Repository}}:{{.Tag}}" | grep -v latest)

echo "remove generated images..."
docker rmi -f $(docker images --filter=reference='generated-dev*' --format "{{.Repository}}:{{.Tag}}")

echo "cleaned up."
