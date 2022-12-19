# litentry-parachain
[![parachain](https://github.com/litentry/litentry-parachain/actions/workflows/parachain-ci.yml/badge.svg)](https://github.com/litentry/litentry-parachain/actions/workflows/parachain-ci.yml)
[![tee-worker](https://github.com/litentry/litentry-parachain/actions/workflows/tee-worker-ci.yml/badge.svg)](https://github.com/litentry/litentry-parachain/actions/workflows/tee-worker-ci.yml)
[![release](https://github.com/litentry/litentry-parachain/actions/workflows/create-release-draft.yml/badge.svg)](https://github.com/litentry/litentry-parachain/actions/workflows/create-release-draft.yml)
[![runtime upgrade](https://github.com/litentry/litentry-parachain/actions/workflows/runtime-upgrade-simulation.yml/badge.svg)](https://github.com/litentry/litentry-parachain/actions/workflows/runtime-upgrade-simulation.yml)

The Litentry parachain.

Similar to polkadot, different chain-specs/runtimes are compiled into one single binary: in our case it's:
- litentry-parachain-runtime (on polkadot)
- litmus-parachain-runtime   (on kusama)
- rococo-parachain-runtime   (on rococo)

Therefore, when building node binary or docker image, no distinction is required. But when building runtime/starting binary/running tests, the chain type must be explicitly given. See the examples below.
## Lists of make targets
Simply run
```
make help
```
to see the full lists of market targets and their short descriptions.

## manual builds

To build the litentry-parachain raw binary manually:
```
make build-node
```

To build the `litentry/litentry-parachain` docker image locally:
```
make build-docker-release
or
make build-docker-production
```
they will use `release` or `production` cargo profile, respectively.

To build the litentry-parachain runtime wasm manually:
```
make build-runtime-litentry
```
The wasms should be located under `target/release/wbuild/litentry-parachain-runtime/`

Similarly, use `make build-runtime-litmus` and `make build-runtime-rococo` to build the litmus-parachain-runtime and rococo-parachain-runtime, respectively.

## launch a standalone node

Simply run
```
make launch-standalone
```
A standalone node will be launched without relaychain, where blocks are finalised immediately. The node is accessible via ws 9944 port.

## launch a local network with relaychain + parachain

The following steps take rococo-parachain for example, because `sudo` will be removed for litentry-parachain and [was removed](https://github.com/litentry/litentry-parachain/issues/775) for litmus-parachain. But generally speaking, lauching a local network works with either of the three chain-settings.

To start a local network with 2 relaychain nodes and 1 parachain node, there're two ways:

### 1. use docker images for both polkadot and parachain (preferred)
```
make launch-docker-rococo
```
[parachain-launch](https://github.com/open-web3-stack/parachain-launch) will be installed and used to generate chain-specs and docker-compose files.

The generated files will be under `docker/generated-rococo/`.

When finished with the network, run
```
make clean-docker-rococo
```
to stop the processes and tidy things up.

### 2. use raw binaries for both polkadot and parachain

Only when option 1 doesn't work and you suspect the docker-image went wrong.

In this case we could try to launch the network with raw binaries.

**On Linux host:**

- you should have the locally compiled `./target/release/litentry-collator` binary.
- run `make launch-binary-rococo`

**On Non-Linux host:**

- you should have locally compiled binaries, for both `polkadot` and `litentry-collator`
- run `./scripts/launch-local-binary.sh rococo path-to-polkadot-bin path-to-litentry-parachain-bin`

After launching, the parachain node is reachable via ws 9944 port and the relaychain nodes are reachable via ws 9946/9947 ports.

When finished with the network, run
```
make clean-binary
```
to stop the processes and tidy things up.
Note this command should work for all parachain types (you don't have to differentiate them).

## run ts tests locally

To run the ts tests locally, similar to launching the networks, it's possible to run them in either docker or binary mode:
```
make test-ts-docker-rococo
```
or
```
# if on Linux
make test-ts-binary-rococo

# otherwise
./scripts/launch-local-binary.sh rococo path-to-polkadot-bin path-to-litentry-parachain-bin
./scripts/run-ts-test.sh rococo
```
Remember to run the clean-up afterwards.

## extend the leasing period

The default leasing duration for parachain is 1 day, in case you want to extend it (even after it's downgraded to parathread), simply do a `forceLease` via sudo, it should be upgraded to parachain soon again and start to produce blocks.

![image](https://user-images.githubusercontent.com/7630809/135689832-1f57cd5c-7f83-4fce-9bb0-832b77a38dcc.png)

## License
GPLv3
