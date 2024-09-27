# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

<<<<<<< HEAD
=======
### Changed

-   Use `@litentry/parachain-api@0.9.20-03.next.0` and `@litentry/sidechain-api@0.9.20-03.next.0`

### Added

-   Support for `Email` Identity

>>>>>>> dev
## [4.2.0] - 2024-08-26

### Changed

<<<<<<< HEAD
- Use `@litentry/parachain-api@0.9.19-7`
=======
-   Use `@litentry/parachain-api@0.9.19-7`
>>>>>>> dev

## [4.1.0] - 2024-08-06

### Added

<<<<<<< HEAD
- Trusted call requests: add the `request.linkIdentityCallback` method.

### Changed

- Use `@litentry/parachain-api@0.9.18-11.2`
- `createLitentryIdentityType`: type can now be created by passing a raw value in hex or `Uint8Array`
- `request.requestBatchVc` now support and optional `signer`.

## [4.0.1] - 2024-07-19

- Use `@litentry/parachain-api@0.9.18-11` and `@litentry/sidechain-api@0.9.18-11` stable versions.

## [4.0.0] - 2024-07-15

- Migrate to `@litentry/parachain-api` and `@litentry/sidechain-api`.
- Distribute as ES Module
- Targets [parachain-release v0.9.18-10](https://github.com/litentry/litentry-parachain/releases/tag/v0.9.18-10)

## Added

- Export the type `IdGraph` and its type's struct name under `ID_GRAPH_STRUCT`.
- Challenge code now produces a prettified string for utf-8 signing for web3 identities when `options.prettify` is set to `true`.

## Changed

- Migrate to `@litentry/parachain-api` and `@litentry/sidechain-api` por chain types. Deprecates `@litentry/chain-types`.
- Support the new `RequestVcResultOrError` type definition.
- `KeyAesOutput` was renamed to `AesOutput`.
- renamed `global` to `globalThis`
- This library is now distributed as an ESModule

## Removed

- Drop `@litentry/chain-types` from dependencies.

### Fixed

- `request.getIdGraphHash` no longer throws when the user's id_graph is empty.
=======
-   Trusted call requests: add the `request.linkIdentityCallback` method.

### Changed

-   Use `@litentry/parachain-api@0.9.18-11.2`
-   `createLitentryIdentityType`: type can now be created by passing a raw value in hex or `Uint8Array`
-   `request.requestBatchVc` now support and optional `signer`.

## [4.0.1] - 2024-07-19

-   Use `@litentry/parachain-api@0.9.18-11` and `@litentry/sidechain-api@0.9.18-11` stable versions.

## [4.0.0] - 2024-07-15

-   Migrate to `@litentry/parachain-api` and `@litentry/sidechain-api`.
-   Distribute as ES Module
-   Targets [parachain-release v0.9.18-10](https://github.com/litentry/litentry-parachain/releases/tag/v0.9.18-10)

## Added

-   Export the type `IdGraph` and its type's struct name under `ID_GRAPH_STRUCT`.
-   Challenge code now produces a prettified string for utf-8 signing for web3 identities when `options.prettify` is set to `true`.

## Changed

-   Migrate to `@litentry/parachain-api` and `@litentry/sidechain-api` por chain types. Deprecates `@litentry/chain-types`.
-   Support the new `RequestVcResultOrError` type definition.
-   `KeyAesOutput` was renamed to `AesOutput`.
-   renamed `global` to `globalThis`
-   This library is now distributed as an ESModule

## Removed

-   Drop `@litentry/chain-types` from dependencies.

### Fixed

-   `request.getIdGraphHash` no longer throws when the user's id_graph is empty.
>>>>>>> dev

## [3.2.1] - 2024-06-10

### Added

<<<<<<< HEAD
- Adds a new dependency: `@litentry/chaindata`.
=======
-   Adds a new dependency: `@litentry/chaindata`.
>>>>>>> dev

## [3.1.2] - 2024-06-08

### Fixed

<<<<<<< HEAD
- Skip `StfError` validation for verifiable credentials requests. Rely on `RequestVcResultOrError` codec.
=======
-   Skip `StfError` validation for verifiable credentials requests. Rely on `RequestVcResultOrError` codec.
>>>>>>> dev

## [3.1.1] - 2024-06-07

### Fixed

<<<<<<< HEAD
- Fix error decoding for single assertions request in `request.requestBatchVc`.
=======
-   Fix error decoding for single assertions request in `request.requestBatchVc`.
>>>>>>> dev

## [3.1.0] - 2024-06-03

### Changed

<<<<<<< HEAD
- Upgrade `@polkadot/api*`, `@polkadot/rpc*`, `@polkadot/types*` to 10.9.1, and `@polkadot/util*` to `12.5.1`

### Removed

- Drop unused `@polkadot/keyring` dependency.

## [3.0.0] - 2024-06-03

- Introduce oAuth2 proofs support for Web2 identity validation

### Added

- Config: support `litentry-staging` for the env var `[NX_]PARACHAIN_NETWORK`.
- Config: support the new env var `[NX_]LITENTRY_NETWORK` for setting the network same as `[NX_]PARACHAIN_NETWORK` but higher precedence.
- Config: accept custom WS endpoints on `[NX_]LITENTRY_NETWORK` / `[NX_]PARACHAIN_NETWORK`.

### Changed

- Use `@litentry/chain-types@2.0.0`
- The type creator `createLitentryValidationDataType` now accepts building oAuth2 proofs for Discord and Twitter.

  ```ts
  // twitter
  const twitterOAuth2Proof = createLitentryValidationDataType(
    registry,
    {
      addressOrHandle: 'my_twitter_handle',
      type: 'Twitter',
    },
    {
      code: 'my_twitter_code',
      state: 'my_twitter_state',
      redirectUri: 'http://test-redirect-uri',
    }
  );

  // Discord
  const validationData = createLitentryValidationDataType(
    registry,
    {
      addressOrHandle: 'my_discord_handle',
      type: 'Discord',
    },
    {
      code: 'my_discord_code',
      redirectUri: 'http://test-redirect-uri',
    }
  );
  ```

  The legacy public message proofs are still supported.
=======
-   Upgrade `@polkadot/api*`, `@polkadot/rpc*`, `@polkadot/types*` to 10.9.1, and `@polkadot/util*` to `12.5.1`

### Removed

-   Drop unused `@polkadot/keyring` dependency.

## [3.0.0] - 2024-06-03

-   Introduce oAuth2 proofs support for Web2 identity validation

### Added

-   Config: support `litentry-staging` for the env var `[NX_]PARACHAIN_NETWORK`.
-   Config: support the new env var `[NX_]LITENTRY_NETWORK` for setting the network same as `[NX_]PARACHAIN_NETWORK` but higher precedence.
-   Config: accept custom WS endpoints on `[NX_]LITENTRY_NETWORK` / `[NX_]PARACHAIN_NETWORK`.

### Changed

-   Use `@litentry/chain-types@2.0.0`
-   The type creator `createLitentryValidationDataType` now accepts building oAuth2 proofs for Discord and Twitter.

    ```ts
    // twitter
    const twitterOAuth2Proof = createLitentryValidationDataType(
    	registry,
    	{
    		addressOrHandle: 'my_twitter_handle',
    		type: 'Twitter',
    	},
    	{
    		code: 'my_twitter_code',
    		state: 'my_twitter_state',
    		redirectUri: 'http://test-redirect-uri',
    	}
    );

    // Discord
    const validationData = createLitentryValidationDataType(
    	registry,
    	{
    		addressOrHandle: 'my_discord_handle',
    		type: 'Discord',
    	},
    	{
    		code: 'my_discord_code',
    		redirectUri: 'http://test-redirect-uri',
    	}
    );
    ```

    The legacy public message proofs are still supported.
>>>>>>> dev

## [2.0.1] - 2024-05-21

### Changed

<<<<<<< HEAD
- When no `PARACHAIN_NETWORK` or `NX_PARACHAIN_NETWORK` is specified, the library will default to the production (`tee-prod`) endpoint rather than to development (`tee-dev`).
=======
-   When no `PARACHAIN_NETWORK` or `NX_PARACHAIN_NETWORK` is specified, the library will default to the production (`tee-prod`) endpoint rather than to development (`tee-dev`).
>>>>>>> dev

## [2.0.0] - 2024-05-17

### Removed

<<<<<<< HEAD
- `createLitentryIdentityType` dropped the support deriving the identity type from the provided address. Now both `addressOrHandle` and `type` are required.

  ```ts
  import { createLitentryIdentityType } from '@litentry/enclave';

  // from
  createLitentryIdentityType(registry, {
    address: '5DNx1Kgis2u2SQq7EJrBdnV49PoZCxV3NqER4vV5VqjqZcat',
  });

  // To
  createLitentryIdentityType(registry, {
    addressOrHandle: '5DNx1Kgis2u2SQq7EJrBdnV49PoZCxV3NqER4vV5VqjqZcat',
    type: 'Substrate',
  });
  ```

  consequently, the following methods require a `LitentryIdentity` for the `who` parameter instead of a plain address string: `request.getIdGraph`, `request.linkIdentity`, `request.requestBatchVc`, `request.setIdentityNetworks`, and `request.createChallengeCode`.
=======
-   `createLitentryIdentityType` dropped the support deriving the identity type from the provided address. Now both `addressOrHandle` and `type` are required.

    ```ts
    import { createLitentryIdentityType } from '@litentry/enclave';

    // from
    createLitentryIdentityType(registry, {
    	address: '5DNx1Kgis2u2SQq7EJrBdnV49PoZCxV3NqER4vV5VqjqZcat',
    });

    // To
    createLitentryIdentityType(registry, {
    	addressOrHandle: '5DNx1Kgis2u2SQq7EJrBdnV49PoZCxV3NqER4vV5VqjqZcat',
    	type: 'Substrate',
    });
    ```

    consequently, the following methods require a `LitentryIdentity` for the `who` parameter instead of a plain address string: `request.getIdGraph`, `request.linkIdentity`, `request.requestBatchVc`, `request.setIdentityNetworks`, and `request.createChallengeCode`.
>>>>>>> dev

## [1.0.4] - 2024-05-16

Routinely update

## [1.0.3] - 2024-05-16

### Changed

<<<<<<< HEAD
- `@litentry/enclave` add support for Solana hex-encoded signatures. It hex string is not provided, it will default to base58 decoding.
=======
-   `@litentry/enclave` add support for Solana hex-encoded signatures. It hex string is not provided, it will default to base58 decoding.
>>>>>>> dev

## [1.0.2] - 2024-05-14

### Changed

<<<<<<< HEAD
- `@litentry/chain-types` is now marked as a peerDependency
=======
-   `@litentry/chain-types` is now marked as a peerDependency
>>>>>>> dev

## [1.0.1] - 2024-05-08

Routinely update

## [1.0.0] - 2024-04-24

<<<<<<< HEAD
- Initial public version

### Added

- Request methods that mutate the idGraph information will have a common response. The entire idGraph will no longer be returned but the information about the updated identity only.
- `request.getIdGraphHash` Request getter to get idGraph hash with no signature.
- `calculateIdGraphHash`: Helper method to calculate the hash of a given local idGraph.
- `request.requestBatchVC`: Request trusted call to request a batch of VCs.
- `Enclave.send` now supports a third argument to subscribe to the WS streamed responses.
- Payload signature is now beautify by default to look more human.
- Use a different key for encrypting the transmitted package to the Enclave.

### Removed

- `request.requestVc`. Superseded by `request.requestBatchVc`.
- `createEnclaveHttpProxyHandler`. The connection to the Enclave is now done directly via WebSockets.

### Changed

- Migrate from `teerex` to `teebag`.
- Enclave's nonce is now retrieved through the `author_getNextNonce` getter call.
- The connection to the Enclave is now done directly via WebSockets. Setting up an HTTP proxy is no longer necessary nor suggested.
- The payload size of all operations was reduced and fixed to a 32-bytes length.
=======
-   Initial public version

### Added

-   Request methods that mutate the idGraph information will have a common response. The entire idGraph will no longer be returned but the information about the updated identity only.
-   `request.getIdGraphHash` Request getter to get idGraph hash with no signature.
-   `calculateIdGraphHash`: Helper method to calculate the hash of a given local idGraph.
-   `request.requestBatchVC`: Request trusted call to request a batch of VCs.
-   `Enclave.send` now supports a third argument to subscribe to the WS streamed responses.
-   Payload signature is now beautify by default to look more human.
-   Use a different key for encrypting the transmitted package to the Enclave.

### Removed

-   `request.requestVc`. Superseded by `request.requestBatchVc`.
-   `createEnclaveHttpProxyHandler`. The connection to the Enclave is now done directly via WebSockets.

### Changed

-   Migrate from `teerex` to `teebag`.
-   Enclave's nonce is now retrieved through the `author_getNextNonce` getter call.
-   The connection to the Enclave is now done directly via WebSockets. Setting up an HTTP proxy is no longer necessary nor suggested.
-   The payload size of all operations was reduced and fixed to a 32-bytes length.
>>>>>>> dev

## 2023-12-05

Update to `Litentry-parachain p0.9.17-9170-w0.0.1-100`.

### Added

<<<<<<< HEAD
- `request.getIdGraph`: fetch the user's idGraph from the Enclave Sidechain. It requires user signature.

### Changed

- **Shielding key**: Users no longer need to set a shielding key on-chain. The data for network transportation is now protected by ephemeral shielding keys generated on the fly. Ephemeral shielding keys increase security and enhance the user experience.
- **Direct responses**: Operation responses are no longer gathered from the Parachain but from the Enclave itself.
- `request.linkIdentity`: The method now has a two level encryption: the information is encrypted with a different key that the one used for transportation.
- `request.linkIdentity`: The call argument `data.encryptionNonce` was removed.
- `request.linkIdentity`: The returned `send` callback now returns both the idGraph and the parsed sidechain response in a `WorkerRpcReturnValue` type.
- `request.createChallengeCode`: The call argument `args.shield` was removed. The Challenge code no longer needs encrypted information.
- `request.setIdentityNetworks`: The returned `send` callback now returns the transaction hash `txHash` and the parsed sidechain response in a `WorkerRpcReturnValue` type.
- `request.requestVc`: The returned `send` callback now returns the `vcIndex`, `vcHash` and the VC's contents on `vcPayload`. As well as the parsed sidechain response in a `WorkerRpcReturnValue` type.
- `enclave.getNonce` was moved as a requestor: `request.getEnclaveNonce`.
- `KeyAesOutput` type is no longer part of the Parachain-runtime metadata and thus it can't be found on `@polkadot/types/lookup`. Use `KeyAesOutput` instead from `@litentry/chain-types`
- `enclave.send`: Error thrown during Enclave operations include more information now.
- `createEnclaveHttpProxyHandler`: HTTP errors responses are now only returned if reaching the Enclave or processing the request fails. However, `enclave.send` could still throw an execution error if the intrinsic operation contains errors. For instance, linking an already linked identity will result on a 200 HTTP response from the Enclave's proxy but `enclave.send` will throw an error about `IdentityAlreadyLinked`.

### Removed

- `request.setUserShieldingKey`: It is no longer needed to set the user's shielding key on-chain. See the Shielding Key point on the Changed section for more information.
- `ky-universal` dependency was dropped.
=======
-   `request.getIdGraph`: fetch the user's idGraph from the Enclave Sidechain. It requires user signature.

### Changed

-   **Shielding key**: Users no longer need to set a shielding key on-chain. The data for network transportation is now protected by ephemeral shielding keys generated on the fly. Ephemeral shielding keys increase security and enhance the user experience.
-   **Direct responses**: Operation responses are no longer gathered from the Parachain but from the Enclave itself.
-   `request.linkIdentity`: The method now has a two level encryption: the information is encrypted with a different key that the one used for transportation.
-   `request.linkIdentity`: The call argument `data.encryptionNonce` was removed.
-   `request.linkIdentity`: The returned `send` callback now returns both the idGraph and the parsed sidechain response in a `WorkerRpcReturnValue` type.
-   `request.createChallengeCode`: The call argument `args.shield` was removed. The Challenge code no longer needs encrypted information.
-   `request.setIdentityNetworks`: The returned `send` callback now returns the transaction hash `txHash` and the parsed sidechain response in a `WorkerRpcReturnValue` type.
-   `request.requestVc`: The returned `send` callback now returns the `vcIndex`, `vcHash` and the VC's contents on `vcPayload`. As well as the parsed sidechain response in a `WorkerRpcReturnValue` type.
-   `enclave.getNonce` was moved as a requestor: `request.getEnclaveNonce`.
-   `KeyAesOutput` type is no longer part of the Parachain-runtime metadata and thus it can't be found on `@polkadot/types/lookup`. Use `KeyAesOutput` instead from `@litentry/chain-types`
-   `enclave.send`: Error thrown during Enclave operations include more information now.
-   `createEnclaveHttpProxyHandler`: HTTP errors responses are now only returned if reaching the Enclave or processing the request fails. However, `enclave.send` could still throw an execution error if the intrinsic operation contains errors. For instance, linking an already linked identity will result on a 200 HTTP response from the Enclave's proxy but `enclave.send` will throw an error about `IdentityAlreadyLinked`.

### Removed

-   `request.setUserShieldingKey`: It is no longer needed to set the user's shielding key on-chain. See the Shielding Key point on the Changed section for more information.
-   `ky-universal` dependency was dropped.
>>>>>>> dev

## 2023-11-01

Initial version
