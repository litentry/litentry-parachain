#!/bin/bash

# Also need to set the port

cd ../scripts/ts-utils/
yarn install
yarn tsc
yarn node setup-enclave.js  $ENCLAVE_ACCOUNT $NEW_MRENCLAVE $SCHEDULE_UPDATE_BLOCK