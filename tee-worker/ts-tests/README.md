## Description

ts-tests of tee-worker

## Environment setup

-   Install [nvm](https://github.com/nvm-sh/nvm)
-   Inside the repository, run `nvm use` to set the correct Node version.
    -   If the version is not installed, run `nvm install`.

## Installation

```
cd tee-worker/ts-tests
nvm use
corepack yarn
```

## Type Generated

Update parachain metadata: `corepack yarn workspace parachain-api update-metadata` (requires the parachain is running)

Update sidechain metadata: `corepack yarn workspace sidechain-api update-metadata` (requires the worker is running)

Generate parachain type: `corepack yarn workspace parachain-api build`

Generate sidechain type: `corepack yarn workspace sidechain-api build`

Alternatively, you can run `corepack yarn update-build` to do all things above in one go.

## Local

[Start parachain && worker](https://github.com/litentry/litentry-parachain/blob/dev/README.md)

## Usage

II identity test: `corepack yarn test-identity:local`

II vc test: `corepack yarn test-vc:local`

II batch identity test: `corepack yarn test-batch:local`

Bulk II identity test: `corepack yarn test-bulk-identity:local`

Bulk II vc test: `corepack yarn test-bulk-vc:local`

Direct invocation substrate identity test: `corepack yarn test-substrate-ii-identity:local`

Direct invocation evm identity test: `corepack yarn test-evm-ii-identity:local`

EVM II examples: `corepack yarn workspace integration-tests evm-ii-examples`

Substrate II examples: `corepack yarn workspace integration-tests substrate-ii-examples`
