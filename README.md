# litentry-parachain
[![Build & Test](https://github.com/litentry/litentry-parachain/actions/workflows/build_and_run_test.yml/badge.svg)](https://github.com/litentry/litentry-parachain/actions/workflows/build_and_run_test.yml)
[![Build wasm](https://github.com/litentry/litentry-parachain/actions/workflows/build_wasm.yml/badge.svg)](https://github.com/litentry/litentry-parachain/actions/workflows/build_wasm.yml)
[![Benchmark runtime weights](https://github.com/litentry/litentry-parachain/actions/workflows/benchmark_runtime_weights.yml/badge.svg)](https://github.com/litentry/litentry-parachain/actions/workflows/benchmark_runtime_weights.yml)
[![Create release draft](https://github.com/litentry/litentry-parachain/actions/workflows/create_release_draft.yml/badge.svg)](https://github.com/litentry/litentry-parachain/actions/workflows/create_release_draft.yml)

The Litentry parachain.

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

To build the litentry-parachain runtime wasm manually:
```
make build-runtime
```
The wasms should be located under `target/release/wbuild/litentry-parachain-runtime/`

To build the `litentry/litentry-parachain` docker image locally:
```
make build-docker
```

## launch of local dev network

To start a local dev network with 2 relaychain nodes and 1 parachain node, there're two ways:

### 1. use docker images for both polkadot and litentry-parachain (preferred)

```
make launch-local-docker
```
[parachain-launch](https://github.com/open-web3-stack/parachain-launch) will be installed and used to generate chain-specs and docker-compose files.

The generated files will be under `docker/generated-dev/`.

When finished with the dev network, run
```
make clean-local-docker
```
to stop the processes and tidy things up.

### 2. use raw binaries for both polkadot and litentry-parachain

Only when option 1 doesn't work and you suspect the docker-image went wrong.

In this case we could try to launch the dev network with raw binaries.

**On Linux host:**

- you should have the locally compiled `./target/release/litentry-collator` binary.
- run `make launch-local-binary`

**On Non-Linux host:**

- you should have locally compiled binaries, for both `polkadot` and `litentry-collator`
- run `./scripts/launch-local-binary.sh path-to-polkadot-bin path-to-litentry-parachain-bin`

When finished with the dev network, run
```
make clean-local-binary
```
to stop the processes and tidy things up.

## run CI tests locally

To run the CI tests locally, similar to launching the networks, it's possible to run them in either docker or binary mode:
```
make test-ci-docker
```
or
```
# if on Linux
make test-ci-binary

# otherwise
./scripts/launch-local-binary.sh path-to-polkadot-bin path-to-litentry-parachain-bin
./scripts/run-ci-test.sh
```
Remember to run the clean-up afterwards.

## extend the leasing period

The default leasing duration for parachain is 1 day, in case you want to extend it (even after it's downgraded to parathread), simply do a `forceLease` via sudo, it should be upgraded to parachain soon again and start to produce blocks.

![image](https://user-images.githubusercontent.com/7630809/135689832-1f57cd5c-7f83-4fce-9bb0-832b77a38dcc.png)

## License
GPLv3
