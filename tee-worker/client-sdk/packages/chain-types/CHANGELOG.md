# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- (breaking) `RequestVcResultOrError` type definition (litentry-parachain#2836).

## [2.1.0] - 2024-06-03

### Changed

- Upgrade `@polkadot/api*`, `@polkadot/rpc*`, `@polkadot/types*` to 10.9.1, and `@polkadot/util*` to `12.5.1`

### Removed

- Drop unused `@polkadot/keyring` dependency.

## [2.0.0] - 2024-06-03

- Introduce oAuth2 proofs support for Web2 identity validation
- Introduce Solidity Assertions.
- Extend enum types to broad token support of certain assertions.

### Added

- OAuth verification for Web2 identities.
- Solidity assertions are introduced via `CorePrimitivesAssertion::Dynamic`
- `CorePrimitivesAssertionWeb3TokenWeb3TokenType` has the following new token types: `Ada`, `Doge`, `Shib`, `Uni`, `Bch`, `Etc`, `Atom`, `Dai`, `Leo`, `Fil`, `Imx`, `Cro`, `Inj`.
- `CorePrimitivesAssertionPlatformUserPlatformUserType` has the following new platform type: `MagicCraftStakingUser`.
- Add `Combo` network to the `Web3Network` type struct.

### Changed

- Web2ValidationData type definition changed to support the new oAuth verification method.

## [1.1.1] - 2024-05-16

- `PrimeIdentity`. Type that represents the identities that can hold an idGraph. I.e., `Substrate`, `Evm`, `Bitcoin`, `Solana`.

## [1.1.0] - 2024-05-14

Release a stable version based on the latest Litentry `tee-prod`'s metadata. No major changes compared to previous version.

## [1.0.1] - 2024-05-08

Routinely update

## [1.0.0] - 2024-04-24

- Initial public version
- [Litentry-parachain p0.9.17-9176-w0.0.2-106](https://github.com/litentry/litentry-parachain/releases/tag/p0.9.17-9176-w0.0.2-106)
