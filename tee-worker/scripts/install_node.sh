#!/bin/bash

set -e

apt-get install -y curl
bash <(curl -fsSL https://deb.nodesource.com/setup_18.x)
apt-get update
apt-get install -y nodejs
npm install -g yarn

