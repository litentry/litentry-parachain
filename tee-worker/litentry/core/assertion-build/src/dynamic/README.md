## Description

Dynamic VC assertion contract written by solidity, using [Hardhat](https://hardhat.org) for compile and test.

## Environment setup

-   Install [nvm](https://github.com/nvm-sh/nvm)
-   Inside the repository, run `nvm use` to set the correct Node version.
    -   If the version is not installed, run `nvm install`.

## Installation

```shell
nvm use
corepack enable pnpm
pnpm install
```

## Usage

### Compile

1. Using hardhat.

```shell
pnpm compile
```

After compiled, the contract bytecode will generate in file `artifacts/contracts/**/{contractName}.sol/{contractName}.json`, e.g. the bytecode of A1 is in the file `artifacts/contracts/A1.sol/A1.json`.

2. Using [Remix IDE](https://remix.ethereum.org).

Should use the `dynamic` as your project root path in Remix IDE as below:

```shell
remixd -s your_repo_path/tee-worker/litentry/core/assertion-build/src/dynamic --remix-ide https://remix.ethereum.org
```

If you have not install remixd before, rub below script to install it.

```shell
npm install -g @remix-project/remixd
```

### Testing

-   Test all: `pnpm test`.

```shell
pnpm test
```

-   Test single file: `pnpm test {testFilePath}`.

Example:

```shell
pnpm test tests/token-holding-amount.ts
```
