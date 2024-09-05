<div align="center">
![](https://res.cloudinary.com/brandpad/image/upload/c_scale,dpr_auto,f_auto,w_768/v1673016042/19618/parachain-logo-color-black-t)

[![general ci](https://github.com/litentry/litentry-parachain/actions/workflows/ci.yml/badge.svg?branch=dev)](https://github.com/litentry/litentry-parachain/actions/workflows/ci.yml)
[![release](https://github.com/litentry/litentry-parachain/actions/workflows/create-release-draft.yml/badge.svg)](https://github.com/litentry/litentry-parachain/actions/workflows/create-release-draft.yml)
[![runtime upgrade](https://github.com/litentry/litentry-parachain/actions/workflows/check-runtime-upgrade.yml/badge.svg)](https://github.com/litentry/litentry-parachain/actions/workflows/check-runtime-upgrade.yml)

</div>

Litentry parachain is a substrate-based, EVM-compatible blockchain that connects to the relaychain (e.g. [Polkadot](https://polkadot.com/)) which ensures shared security and interoperability. It serves as the backbone of Litentry protocol:
- LIT token native features: transfer, governance, staking ...
- parentchain of identity-worker, which is a TEE-based sidechain to achieve identity aggregation and crediential issuance without promising users' privacy 
- parentchain of bitacross-worker, which is a TEE-based offchain-worker to bridge assets across chains using native custodian and multisig

## Build parachain

To build the binary:

```
make build-node
```

To build the `litentry/litentry-parachain` docker image, based on cargo profile:

```
make build-docker-release
make build-docker-production
```

To build the litentry-parachain runtime wasm:

```
make build-runtime-litentry
```

The wasms should be located under `target/release/wbuild/litentry-parachain-runtime/`

Similarly, use `make build-runtime-rococo` to build the rococo-parachain-runtime.

## Launch parachain
### Launch a parachain network with relaychains

Litentry takes use of zombinet(https://github.com/paritytech/zombienet) to spin up local networks with 2 relaychain nodes and 1 parachain node:
```
make launch-network-litentry`
```
It will firstly look for the `litentry-collator` binary under `target/release/`, if not found, it will try to copy binaries out from `litentry/litentry-parachain:latest` image if you are on Linux.

When finished with the network, run

```
make clean-network
```

to stop the processes and tidy things up.

### Launch a standalone parachain node

To speed up the development, it's possible to launch the parachain without relaychain.
In this case, parachain will author blocks by itself with instant block finalisation, please refer to [this PR](https://github.com/litentry/litentry-parachain/pull/1059).

```
make launch-standalone
```

## How to build and run tee-worker

### Preparation

- Env: [Setup **SGX TEE** Environment](https://web3builders.notion.site/Setup-SGX-TEE-Environment-68066770831b45b7b632e682cf159477?pvs=4) 

### Build

```
cd /tee-worker
source /opt/intel/sgxsdk/environment
SGX_MODE=SW WORKER_DEV=1 make
```

### Launch

Before executing `launch.py`, the following Python libraries need to be installed
```
pip install python-dotenv pycurl docker toml
```

TEE-workers need a running parachain to become operational. We have an all-in-one script `local-setup/launch.py` to launch both parachain and workers:
```
./local-setup/launch.py -p standalone
./local-setup/launch.py -p network
./local-setup/launch.py -p remote
```

They stand for different parachain launching options:
- standalone parachain
- parachain network with relaychains
- parachain is remotely launched (elsewhere), so don't launch parachain in `launch.py`
respectively.

### TEE worker tests 

Refer to [tee-worker ts-tests](https://github.com/litentry/litentry-parachain/blob/dev/tee-worker/ts-tests/README.md)

### Clean-up

In the worker launch terminal, `Ctrl + C` should interrupt and clean everything up automatically.

### How to tell the worker is running

![image (2)](https://github.com/cryptoade1/litentry-parachain/assets/88367184/87dd72f6-0124-4007-9b14-dddc97d3d252)
Waiting for block production to start

![image (3)](https://github.com/cryptoade1/litentry-parachain/assets/88367184/83872a38-abfe-4dc3-878f-9e25b7da6c2d)
Block produced

![image (4)](https://github.com/cryptoade1/litentry-parachain/assets/88367184/d04c76f7-484a-4172-ac10-53a6d4714766)
Parachain up; waiting for the worker to start

![image (5)](https://github.com/cryptoade1/litentry-parachain/assets/88367184/cb1cea60-bc5d-4b62-bae7-503583a135ee)
Worker started!

![image (6)](https://github.com/cryptoade1/litentry-parachain/assets/88367184/21ff630c-baa3-439d-b70a-03f621f49258)
In logs, you’ll see the sidechain starts to produce blocks

### Additional Info:

1. Change the RUST_LOG level: `litentry-parachain/local-setup/py/worker.py`
2. Check existing ts-tests: `litentry-parachain/tee-worker/ts-tests/package.json`
3. JSON config parameters: `litentry-parachain/tee-worker/service/src/cli.yml`