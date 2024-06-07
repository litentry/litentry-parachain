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

## Data-provider test

1. Start tee-worker with real endpoint and real code(Configure in `local-setup/env.dev.`).
2. Configure definitions in `ts-tests/integration-tests/common/credential-json/*.json`,like [vip3-membership-card-gold](https://github.com/litentry/litentry-parachain/blob/dev/tee-worker/ts-tests/integration-tests/common/credential-json/vip3.json#L3)
3. Execute test cases:
   1. Single test:  `pnpm run test-data-providers:local --id=your credential json id` 
   2. All credential tests:`pnpm run test-data-providers:local`（Run all the `credential-json/*.json` test cases, execute them in the order of [export](https://github.com/litentry/litentry-parachain/blob/dev/tee-worker/ts-tests/integration-tests/common/credential-json/index.ts#L21).）

​    