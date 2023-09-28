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
corepack enable pnpm
pnpm install
```

## Type Generated

Update parachain metadata: `pnpm --filter parachain-api run update-metadata` (requires the parachain is running)

Update sidechain metadata: `pnpm --filter sidechain-api run update-metadata` (requires the worker is running)

Generate parachain type: `pnpm --filter parachain-api run build`

Generate sidechain type: `pnpm --filter sidechain-api run build`

Alternatively, you can run `pnpm --run update-build` to do all things above in one go.

## Local

[Start parachain && worker](https://github.com/litentry/litentry-parachain/blob/dev/README.md)

## Usage

II identity test: `pnpm --filter integration-tests run test-ii-identity:local`

II vc test: `pnpm --filter integration-tests run test-ii-vc:local`

II batch identity test: `pnpm --filter integration-tests run test-ii-batch:local`

Direct invocation substrate identity test: `pnpm --filter integration-tests run test-di-substrate-identity:local`

Direct invocation evm identity test: `pnpm --filter integration-tests run test-di-evm-identity:local`

Direct invocation vc test: `pnpm --filter integration-tests run test-di-vc:local`
