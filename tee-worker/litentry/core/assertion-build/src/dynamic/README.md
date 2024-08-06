## Description

Dynamic VC assertion contract is written by solidity, using [Hardhat](https://hardhat.org) for compilation and testing.

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

After compiling, the contract bytecode will generate in file `artifacts/contracts/**/{contractName}.sol/{contractName}.json`, e.g. the bytecode of A1 is in the file `artifacts/contracts/A1.sol/A1.json`.

2. Using [Remix IDE](https://remix.ethereum.org).

Should use the `dynamic` as your project root path in Remix IDE as below:

```shell
remixd -s your_repo_path/tee-worker/litentry/core/assertion-build/src/dynamic --remix-ide https://remix.ethereum.org
```

If you have not installed Remixd before, run the below script to install it.

```shell
npm install -g @remix-project/remixd
```

### Deploy

The deployment script can be used to deploy a specific contract to the specified chain. Below is the usage of the deployment script.

#### Command Syntax

```shell
pnpm run deploy-contract --contract <ContractName> --chain <ChainName> [--mnemonic <MnemonicValue>] [--secrets <Secret1> <Secret2> ...]
```

#### Parameters

-   --contract: Specify the name of the contract you wish to deploy.
-   --chain: Specify the target chain environment. Supported values are:
    -   local
    -   dev
    -   staging
    -   prod
-   --mnemonic: Optional, the mnemonic string required to generate the wallet for contract deployment on the staging and production chains.
-   --secrets: Optional, provide the required secret values for the contract. These may include API keys, private keys, or other sensitive information needed for the deployment contract, multiple secrets must be separated by blank, and the secret item does not support line breaks.

#### Example

To deploy the `TokenMapping` contract to the `dev` chain with specific secrets, you would run the following command:

```shell
pnpm run deploy-contract --contract TokenMapping --chain dev --mnemonic "angle total unfold"  --secrets "abc" "vna" "poi xyz"
```

#### Troubleshooting

-   If you meet below error, run `chmod +x ./scripts/run_deploy.sh` can fix it.

```plain
sh: 1: ./scripts/run_deploy.sh: Permission denied
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
