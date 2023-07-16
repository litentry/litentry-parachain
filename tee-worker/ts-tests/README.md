## Description

ts-tests of tee-worker

## Environment

node>=16, yarn v3

## Installation

`cd tee-worker/ts-tests`

`yarn`

## Type Generated

Update parachain metadata: `yarn workspace parachain-api update-metadata`

Update parachain metadata: `yarn workspace sidechain-api update-metadata`

Generate parachain type: `yarn workspace parachain-api build`

Generate sidechain type: `yarn workspace sidechain-api build`

## Local

[Start parachain && tee-worker](https://github.com/litentry/litentry-parachain/blob/dev/README.md)

`echo -e "NODE_ENV=local\nWORKER_END_POINT=wss://localhost:2000\nSUBSTRATE_END_POINT=ws://localhost:9944\nID_HUB_URL=http://localhost:3000" > ./integration-tests/.env.local`

## Usage

Standard identity test: `yarn test-identity:local`

Standard vc test: `yarn test-vc:local`

Batch identity test: `yarn test-batch:local`

Bulk identity test: `yarn test-bulk-identity:local`

Bulk vc test: `yarn test-bulk-vc:local`

Direct invocation identity test: `yarn test-identity-direct-invocation:local`
