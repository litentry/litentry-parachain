# Smart Contract on Ethereum

## Build Contracts

These contracts come from [Repo: chainbridge-solidity](https://github.com/litentry/chainbridge-solidity)

Run command: `make compile`in the project(chainbridge-solidity) root directory to generate the files.

Copy files from chainbridge-solidity(build/contracts) to current directory.

The smart contract version deployed on Rinkeby or Mainnet is: [Phala_audited version](https://github.com/litentry/chainbridge-solidity/commit/**)

In the CI test, the version is: [Branch: phala-bridge](https://github.com/litentry/chainbridge-solidity/commit/**)

## Why use different versions

In the integration test, after launching the local test network, We want to run the test script multiple times(The contract is redeployed every time).

After running the test script, the nonce on Parachain will be changed, so a function is needed to set the value of the nonce on smart contract.

In the branch Phala-Bridge, we can call the function: adminSetDepositNonce. In addition, we can also flexibly set the value of decimals for different tokens (adminSetDecimals).

## Differences

There are some differences between the two versions of the contract code. These differences are mainly related to decimals and nonce, like: adminSetDecimals, adminSetDepositNonce.

Only some settings are changed, the core function like deposit, voteProposal, executeProposal have not changed

For more details, see at [Compare diffs](https://github.com/litentry/chainbridge-solidity/compare/minqi-dev..Phala-Network:chainbridge-solidity:phala-bridge)
