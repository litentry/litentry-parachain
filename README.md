# litentry-parachain
[![Build & Test](https://github.com/litentry/litentry-parachain/actions/workflows/build_and_run_test.yml/badge.svg)](https://github.com/litentry/litentry-parachain/actions/workflows/build_and_run_test.yml)
[![Build wasm](https://github.com/litentry/litentry-parachain/actions/workflows/build_wasm.yml/badge.svg)](https://github.com/litentry/litentry-parachain/actions/workflows/build_wasm.yml)
[![Update Pallets](https://github.com/litentry/litentry-parachain/actions/workflows/update_pallets.yml/badge.svg)](https://github.com/litentry/litentry-parachain/actions/workflows/update_pallets.yml)
[![Build docker with features](https://github.com/litentry/litentry-parachain/actions/workflows/build_docker_with_features.yml/badge.svg)](https://github.com/litentry/litentry-parachain/actions/workflows/build_docker_with_features.yml)
[![Create release draft](https://github.com/litentry/litentry-parachain/actions/workflows/create_release_draft.yml/badge.svg)](https://github.com/litentry/litentry-parachain/actions/workflows/create_release_draft.yml)

The Litentry parachain.


## launch of local dev network

To start a dev network locally with 2 relaychain nodes and 1 parachain node:
```
make launch-local-docker
```
During this a few files will be generated under `docker/generated-dev/`.

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

To build staging env chain-specs:
```
make generate-docker-compose-staging
```
Staging env doesn't really run the service inside docker container, but the generated chain specs are useful.

For a full list of make targets, run:
```
make help
```

The default leasing duration for parachain is 1 day, in case you want to extend it (even after it's downgraded to parathread), simply do a `forceLease` via sudo, it should be upgraded to parachain soon again and start to produce blocks.

![image](https://user-images.githubusercontent.com/7630809/135689832-1f57cd5c-7f83-4fce-9bb0-832b77a38dcc.png)


## run CI tests locally

To run the CI tests locally, dev network must be launched first as above, then:
```
make test-ci
```
You may want to run `make clean-local-docker` to stop the containers and tidy them.
Please note that this command also removes all local `litentry/litentry-parachain` images except the one with `latest` tag.

## License
Apache-2.0
