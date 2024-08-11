# Smart Contract on Ethereum

## Build Contracts

These contracts come from [Repo: chainbridge-solidity](https://github.com/collab-ai-network/chainbridge-solidity)

Run command: `make compile`in the project(chainbridge-solidity) root directory to generate the files.

Copy files from chainbridge-solidity(build/contracts) to current directory.

In the CI test, the version is: [Branch: release-0.8.19](https://github.com/collab-ai-network/chainbridge-solidity/commit/1aa527d23001d9d134624a6e63dcdda1b7287fbc)

## Why use different versions

In the integration test, after launching the local test network, We want to run the test script multiple times(The contract is redeployed every time).

After running the test script, the nonce on Parachain will be changed, so a function is needed to set the value of the nonce on smart contract.

In the branch Phala-Bridge, we can call the function: adminSetDepositNonce. In addition, we can also flexibly set the value of decimals for different tokens (adminSetDecimals).
