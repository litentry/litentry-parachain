# litentry-parachain
[![Build & Test](https://github.com/litentry/litentry-parachain/actions/workflows/build_and_run_test.yml/badge.svg)](https://github.com/litentry/litentry-parachain/actions/workflows/build_and_run_test.yml)
[![Update Pallets](https://github.com/litentry/litentry-parachain/actions/workflows/update_pallets.yml/badge.svg)](https://github.com/litentry/litentry-parachain/actions/workflows/update_pallets.yml)

The Litentry parachain.


## Setup
1. Update git submodule
```
git submodule update --init --recursive
```
2. Build polkadot binary
```
cd polkadot
cargo build --release
```
3. Build Litentry parachain binary
```
cargo build --release
```
4. Build token server binary
```
cd token-server
cargo build --release
```
5. Run test
```
cd ts-tests
./scripts/run-test.sh
```


## License
Apache-2.0


