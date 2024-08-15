# ⛔️ Deprecated

This package is deprecated and unmaintained. Please use [@litentry/parachain-api](https://www.npmjs.com/package/@litentry/parachain-api) and [@litentry/sidechain-api](https://www.npmjs.com/package/@litentry/sidechain-api) instead

# chain-types

This library contains the Litentry Network API types and types definitions.

These types were auto generated using [Polkadot.js Type Generation](https://polkadot.js.org/docs/api/examples/promise/typegen/)

## How to use it

1. Install the package from NPM

   ```
   npm install @litentry/chain-types
   ```

2. Extend and decorate the API's types with:

   ```ts
   import { identityManagement } from '@litentry/chain-types';

   const api = await ApiPromise.create({
     provider: wsProvider,
     types: {
       ...identityManagement.types,
     },
   });
   ```

3. Import type definitions as needed:

   ```ts
   import type { LitentryIdentity } from '@litentry/chain-types';

   function myFunction(identity: LitentryIdentity) {
     // ...
   }
   ```

### Versions

This package is distributed under two main tags: `next` and `latest`.

Versions in the pattern of `x.x.x-next.x` feature the most recent code version to work with `tee-dev`. E.g., `1.0.0-next.0`. Once stable and once the Litentry Protocol is upgraded, the version will be tagged as `latest` and should be used against `tee-prod`. E.g., `1.0.0`. You can find all versions on https://www.npmjs.com/package/@litentry/chain-types?activeTab=versions

## Development

### Quick start

1. Install dependencies

   ```
   pnpm install
   ```

2. Spin up an local NPM registry

   ```
   pnpm nx local-registry
   ```

3. Publish locally

   Follow the steps of [Publish new versions](#publish-new-versions). The step 1 can be skipped.

   As long as the local registry is up, any publishing will happen locally.

4. Run test and lint checks

   ```
   pnpm nx run chain-types:lint

   pnpm nx run chain-types:test
   ```

### Publish new versions

1. Bump the version on package.json to for instance `1.0.0`.

2. Update the latest documentation

   ```
   pnpm nx run chain-types:generate-doc
   ```

3. Build the project

   ```
   pnpm nx run chain-types:build
   ```

4. Publish the distribution files

   ```
   pnpm nx run chain-types:publish --ver 1.0.0 --tag latest
   ```

### How to regenerate types

By default, the following commands will ran against the `tee-dev` Parachain Endpoint

```sh
pnpm nx run chain-types:download-meta
pnpm nx run chain-types:generate-defs
pnpm nx run chain-types:generate-meta
```

### When to regenerate types

Regenerating types can be done at any time. It will pull metadata from the Parachain and create the corresponding interfaces, types and API metadata.

We may want to regenerate when any of these happen:

- The Parachain pallet changed.
- The [Parachain TypeScript test type definitions](https://github.com/litentry/litentry-parachain/blob/dev/tee-worker/ts-tests/type-definitions.ts) changed.
- We added new type definitions in `libs/chain-types/src/lib/interfaces`.
- `@polkadot/*` dependencies got updated.
