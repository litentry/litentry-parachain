# Litentry Integration Test

This node.js project aims to test and verify Litentry Runtime as a whole, including the interactions between user, AccountLink and OffChainWorker.

## Install

`npm i`

## Run

Run tests with command

`npm test`

## Output

The current test runs through the following steps: link eth account -> check account linking state -> asset claim -> check asset balances.

There are test assertions in step 2 and step 4. Therefore if assert fails while you are running it, probably something is broken.