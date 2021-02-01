# Litentry Integration Test

This node.js project aims to test and verify Litentry Runtime as a whole, including the interactions between user, AccountLink and OffChainWorker.

## Install

`npm i`

## Run

For now, you need to start the node manually first by the command

`./target/debug/litentry-node --dev --tmp`

And then run the command

`ts-node ts-tests/tests/test-eth-balance.ts`

Later a better test suite will be built with better framework integrated and full test automation.

## Output

The current test runs through the following steps: link eth account -> check account linking state -> asset claim -> check asset balances.

There are test assertions in step 2 and step 4. Therefore if assert fails while you are running it, probably something is broken.