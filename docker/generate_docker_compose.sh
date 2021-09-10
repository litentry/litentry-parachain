#!/bin/sh

basedir=$(dirname "$0")
cd "$basedir"

if ! docker image inspect litentry/litentry-parachain:latest &>/dev/null; then
  echo "please build litentry/litentry-parachain:latest first"
  exit 1
fi

if ! parachain-launch --version &>/dev/null; then
  echo "please install parachain-launch first:"
  echo "e.g."
  echo "yarn global add @open-web3/parachain-launch"
  exit 1
fi

parachain-launch generate --config=2relay-1para-launch-config.yml --output=generated --yes

cat << EOF

Done, please check files under $basedir/generated/

To start the network, run
cd generated
docker-compose up -d --build
EOF
