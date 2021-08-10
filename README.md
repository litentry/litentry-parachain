# litentry-parachain
[![Rust](https://github.com/litentry/litentry-parachain/actions/workflows/build_test.yml/badge.svg)](https://github.com/litentry/litentry-parachain/actions/workflows/build_test.yml)
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
npm install
npm test
```


## License
Apache-2.0


