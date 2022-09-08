# Smart Contract on Ethereum

## Build Contracts

These contracts come from [Repo: chainbridge-solidity](https://github.com/Phala-Network/chainbridge-solidity)

Run command: `make compile`in the project(chainbridge-solidity) root directory to generate the files.

Copy files from chainbridge-solidity(build/contracts) to current directory.

The smart contract version deployed on Rinkeby or Mainnet is: [Phala_audited version](https://github.com/Phala-Network/chainbridge-solidity/commit/0561b64da85f3242d8ddf63d1dde7c203e1a7f9a)

In the CI test, the version is: [Branch: phala-bridge](https://github.com/Phala-Network/chainbridge-solidity/commit/9f0487a89b68abc8f60d1f77c92e0b5b4789b23f)

## Why use different versions

In the integration test, after launching the local test network, We want to run the test script multiple times(The contract is redeployed every time).

After running the test script, the nonce on Parachain will be changed, so a function is needed to set the value of the nonce on smart contract.

In the branch Phala-Bridge, we can call the function: adminSetDepositNonce. In addition, we can also flexibly set the value of decimals for different tokens (adminSetDecimals).

## Differences

There are some differences between the two versions of the contract code. These differences are mainly related to decimals and nonce, like: adminSetDecimals, adminSetDepositNonce.

Only some settings are changed, the core function like deposit, voteProposal, executeProposal have not changed

For more details, see at [Compare diffs](https://github.com/Phala-Network/chainbridge-solidity/compare/btclottery..Phala-Network:chainbridge-solidity:phala-bridge)
