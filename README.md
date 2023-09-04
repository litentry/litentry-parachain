# Litentry Parachain
![](https://res.cloudinary.com/brandpad/image/upload/c_scale,dpr_auto,f_auto,w_768/v1673016042/19618/parachain-logo-color-black-t)

[![general ci](https://github.com/litentry/litentry-parachain/actions/workflows/ci.yml/badge.svg?branch=dev)](https://github.com/litentry/litentry-parachain/actions/workflows/ci.yml)
[![release](https://github.com/litentry/litentry-parachain/actions/workflows/create-release-draft.yml/badge.svg)](https://github.com/litentry/litentry-parachain/actions/workflows/create-release-draft.yml)
[![runtime upgrade](https://github.com/litentry/litentry-parachain/actions/workflows/runtime-upgrade-simulation.yml/badge.svg)](https://github.com/litentry/litentry-parachain/actions/workflows/runtime-upgrade-simulation.yml)


A parachain is an application-specific data structure that is globally coherent and validatable by the validators of the relaychain. They take their name from the concept of parallelized chains that run parallel to the relaychain. Most commonly, a parachain will take the form of a blockchain, but there is no specific need for them to be actual blockchains.

Basically, parachains are layer-1 blockchains that connect to the relaychains (Polkadot or Kusama), which validates the state transition of connected parachains, providing a shared state across the entire ecosystem. Since the validator set on the relaychain is expected to be secure with a large amount of stake put up to back it, it is desirable for parachains to benefit from this shared security.

To achieve identity aggregation, Litentry has a requirement to store sensitive user data, like web3 addresses, computed credit scores, and VCs in the trusted execution environment (TEE). Litentry builds a TEE side chain for this purpose and it is composed of multiple TEE-equipped nodes, to guarantee the security of data storage and data processing without exposing users' private data. A core component of this is the Litentry TEE worker which is based on Integritee's worker. It executes functions with specified inputs and resource limits in response to TEE calls and operations to ensure a sufficient level of scaling.

Overall, our architecture is made of up Relaychains ( Polkadot and Kusama), Parachains (Litentry and Litmus), and the TEE sidechain which is supported by  and enables the runtime to execute in an SGX secure run environment.

To serve as the backbone platform for various Litentry products and achieve a transparent and decentralized user experience, we have different chain-specs/runtimes compiled into one single binary. They are:

- litentry-parachain-runtime (on polkadot)
- litmus-parachain-runtime (on kusama)
- rococo-parachain-runtime (on rococo testnet)

Therefore, when building node binary or docker image, no distinction is required. But when building runtime/starting binary/running tests, the chain type must be explicitly given. See the examples below.

## Lists of make targets

Simply run

```
make help
```

to see the full lists of market targets and their short descriptions.

## Manual builds

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


## Launch a local network with relaychain + parachain

Take rococo-parachain for example, but generally speaking, launching a local network works with either of the three chain-settings.

To start a local network with 2 relaychain nodes and 1 parachain node, there're two ways:

### 1. Use docker images for both polkadot and parachain (preferred)

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

### 2. Use raw binaries for both polkadot and parachain

Only when option 1 doesn't work and you suspect the docker-image went wrong.

In this case, try to launch the network with raw binaries.

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

## Run ts Tests Locally

To run the ts tests locally, similar to launching the networks, it's possible to run them in either docker or binary mode.

### Docker Mode

```
make test-ts-docker-rococo
```

### Binary Mode

Please refer to (tee-worker/ts-tests/README.md) for instructions on setting up your Node environment first.

```
# if on Linux
make test-ts-binary-rococo

# otherwise
./scripts/launch-local-binary.sh rococo path-to-polkadot-bin path-to-litentry-parachain-bin
./scripts/run-ts-test.sh rococo
```

Remember to run the clean-up afterwards.

## License

GPLv3


## How to Build and Run Parachain and Tee-worker

## Preparation

- Env: [Setup **SGX TEE** Environment](https://web3builders.notion.site/Setup-SGX-TEE-Environment-68066770831b45b7b632e682cf159477?pvs=4) 

## Build

The `Litentry-worker` needs to be built before the launch of the entire system

```
git clone https://github.com/litentry/litentry-parachain
cd ./litentry-parachain/tee-worker
source /opt/intel/sgxsdk/environment
SGX_MODE=SW make
```

## Launch

### 1. Start a local docker setup

In order to create a local docker setup, you can run the following command
```
./local-setup/launch.py --config ./local-setup/github-action-config-one-worker.json
```
This will create three docker containers, 2 Relay Chain Validators, and 1 Parachain Collator. However, it will use the default ports as present in .env.dev. If you want to run the system by offsetting the default ports, you can run this command instead:

```
./local-setup/launch.py --config local-setup/development-worker.json --offset 100
```
This will run the same containers and use the offset value of 100.

### 2. Start a local binary setup

In order to create a local binary setup, using default ports, you can run the following command:
```
/local-setup/launch.py --config ./local-setup/github-action-config-one-worker.json --parachain local-binary
```

If you want to launch the same system by offsetting the port values, you can use this command: 
```
./local-setup/worker-log-level-config.toml 100 
```
In case you receive the following error:
```ModuleNotFoundError: No module named 'pycurl'```

Fix it manually by installing pycurl using pip3. 

### 3. Remote <> Integritee Node

If you are running the integritee node manually, Then be mindful of the port that is being set while launching the integritee node, For example:
```
cargo build --release --features "skip-extrinsic-filtering skip-ias-check skip-scheduled-enclave-check"
./target/release/integritee-node --rpc-cors=all --ws-external --tmp --dev  --state-pruning archive  --blocks-pruning archive --rpc-port 9933 --ws-port 9944
```

`--rpc-port` is used to set the rpc port and `--ws-port` is used to set the ws port for the node. You then have to update the `config.json` with the port that is being used and then run the command 

**Note: The `Integritee-Node` is not maintained by Parachain Team actively, Please check the [repository]([url](https://github.com/litentry/integritee-node)) if it has all the latest updates from `Litentry-Parachain` .**

### TEE Worker Tests 

Refer to [tee-worker ts-tests](https://github.com/litentry/litentry-parachain/blob/dev/tee-worker/ts-tests/README.md)

### Clean-up

In the worker launch terminal, `Ctrl + C` should interrupt and clean everything up automatically.
Additionally, if you launch the parentchain with binaries (integritee-node or parachain), you have to stop the parentchain by `Ctrl + C` too, or using `kill`

If you want to still call the scripts responsible for cleaning up the process, 
If launched via docker
```
make clean-docker-rococo
```
Docker can sometimes still leave behind remnants of an old build, run:
```
docker system prune 
docker builder prune
```

If launched via binary 

```
make clean-binary 
```
### Additional Info:

1. Change the RUST_LOG level: `litentry-parachain/tee-worker/local-setup/py/worker.py`
2. Check existing ts-tests: `litentry-parachain/tee-worker/ts-tests/package.json`
3. JSON config parameters: `litentry-parachain/tee-worker/service/src/cli.yml`


