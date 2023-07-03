#!/bin/bash

# Also need to set the port

cd $ROOTDIR/scripts/ts-utils/
yarn install
npx ts-node setup-enclave.ts  $ENCLAVE_ACCOUNT $NEW_MRENCLAVE $SCHEDULE_UPDATE_BLOCK