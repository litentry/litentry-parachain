## Description

ts-tests of tee-worker

## Environment setup

-   Install [nvm](https://github.com/nvm-sh/nvm)
-   Inside the repository, run `nvm use` to set the correct Node version.
    -   If the version is not installed, run `nvm install`.

## Prerequisite

Before running the ts-tests, the client-api types generation needs to be completed.

See client-api [README.md](https://github.com/litentry/litentry-parachain/blob/dev/tee-worker/client-api/README.md)

## Installation

```
cd tee-worker/ts-tests
nvm use
corepack enable pnpm
pnpm install
```

## Local

[Start parachain && worker](https://github.com/litentry/litentry-parachain/blob/dev/README.md)

## Usage(ts-tests folder)

II identity test: `pnpm --filter integration-tests run test-ii-identity:local`

II vc test: `pnpm --filter integration-tests run test-ii-vc:local`

II batch identity test: `pnpm --filter integration-tests run test-ii-batch:local`

Direct invocation substrate identity test: `pnpm --filter integration-tests run test-di-substrate-identity:local`

Direct invocation evm identity test: `pnpm --filter integration-tests run test-di-evm-identity:local`

Direct invocation vc test: `pnpm --filter integration-tests run test-di-vc:local`