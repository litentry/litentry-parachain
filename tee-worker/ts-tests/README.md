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

```
pnpm --filter integration-tests run test your-testfile.test.ts
```

II identity test: `pnpm --filter integration-tests run test ii_identity.test.ts`

II vc test: `pnpm --filter integration-tests run test ii_vc.test.ts`

II batch identity test: `pnpm --filter integration-tests run test ii_batch.test.ts`

Direct invocation substrate identity test: `pnpm --filter integration-tests run test di_substrate_identity.test.ts`

Direct invocation evm identity test: `pnpm --filter integration-tests run test di_evm_identity.test.ts`

Direct invocation vc test: `pnpm --filter integration-tests run test di_vc.test.ts`
