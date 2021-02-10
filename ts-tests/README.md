# Litentry Integration Test

This node.js project aims to test and verify Litentry Runtime as a whole, including the interactions between user, AccountLink and OffChainWorker.

## Install

`npm i`

## Run

1. Launch Relay Nodes

`./scripts/start-alice-and-bob.sh`

2. Run tests

`npm test`

3. After it's done, you need to manually kill relay nodes 

`killall polkadot`
## Output

The current test runs through the following steps: link eth account -> check account linking state -> asset claim -> check asset balances.

There are test assertions in step 2 and step 4. Therefore if assert fails while you are running it, probably something is broken.