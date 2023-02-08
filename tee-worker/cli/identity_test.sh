#!/bin/bash

set -o pipefail

cd /ts-tests
yarn install
yarn run test-identity:staging
