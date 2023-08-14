# Litentry Integration Test

This node.js project aims to test and verify Litentry Runtime as a whole, including the interactions between user, AccountLink and OffChainWorker.

## Environment setup

-   Install [nvm](https://github.com/nvm-sh/nvm)
-   Inside the repository, run `nvm use` to set the correct Node version.
    -   If the version is not installed, run `nvm install`.

## To run tests with one-line command:

```
./scripts/run-test.sh
```

To run separate yarn targets, please check package.json

## Output

The current test runs through the following steps: link eth account -> check account linking state -> asset claim -> check asset balances.

There are test assertions in step 2 and step 4. Therefore if assert fails while you are running it, probably something is broken.
