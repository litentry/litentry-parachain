## Description

ts-tests of tee-worker

## Environment

-   [ ] TODO add docs re nvm, corepack, etc.

[nvm](https://github.com/nvm-sh/nvm) should be installed

## Installation

```
cd tee-worker/ts-tests
corepack yarn
```

## Type Generated

Update parachain metadata: `corepack yarn workspace parachain-api update-metadata` (requires the parachain is running)

Update sidechain metadata: `corepack yarn workspace sidechain-api update-metadata` (requires the worker is running)

Generate parachain type: `corepack yarn workspace parachain-api build`

Generate sidechain type: `corepack yarn workspace sidechain-api build`

## Local

[Start parachain && worker](https://github.com/litentry/litentry-parachain/blob/dev/README.md)

## Usage

Standard identity test: `corepack yarn test-identity:local`

Standard vc test: `corepack yarn test-vc:local`

Batch identity test: `corepack yarn test-batch:local`

Bulk identity test: `corepack yarn test-bulk-identity:local`

Bulk vc test: `corepack yarn test-bulk-vc:local`

Direct invocation identity test: `corepack yarn test-identity-direct-invocation:local`

Di examples: `corepack yarn workspace integration-tests di-examples`
