## Description

Client-api of tee-worker

## Environment setup

-   Install [nvm](https://github.com/nvm-sh/nvm)
-   Inside the repository, run `nvm use` to set the correct Node version.
    -   If the version is not installed, run `nvm install`.

## Installation

```cd tee-worker/ts-tests
cd tee-worker/client-api
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

## Publish

1. Modify the `version` in the `package.json` files of both the parachain-api and sidechain-api, like:

`"version": "1.0.0-{tag}"`

Please refer to the [version rules](https://docs.npmjs.com/about-semantic-versioning) and ensure that it is unique and hasn't been published on the GitHub package registry before.

2. Merge into the target branch.

3. Run the [Release Ts API Package](https://github.com/litentry/litentry-parachain/actions/workflows/release-ts-api-package.yml) action.