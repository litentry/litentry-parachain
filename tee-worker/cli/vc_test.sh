#!/bin/bash

set -o pipefail

cd /ts-tests
yarn install
yarn run test-vc:staging
