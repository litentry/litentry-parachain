#!/bin/bash

# Copyright 2020-2023 Trust Computing GmbH.

set -euo pipefail

/usr/local/bin/litentry-cli print-sgx-metadata-raw

cd /client-api
pnpm install
pnpm run update-build:ci
echo "Client-api build is complete"
echo ""