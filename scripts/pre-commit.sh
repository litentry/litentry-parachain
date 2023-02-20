#!/usr/bin/env bash
# It is recommended to execute the script before commit
# which will help us to reduce test/fmt/clippy failures in CI

start=$(date +%s)

make fmt
taplo fmt

make clippy

root_dir=$(git rev-parse --show-toplevel)
CARGO_TARGET_DIR=${root_dir}/target

cd "${root_dir}/tee-worker" || exit
taplo fmt
CARGO_TARGET_DIR=${CARGO_TARGET_DIR} cargo clippy --release -- -D warnings || exit
CARGO_TARGET_DIR=${CARGO_TARGET_DIR} cargo clippy --release --features evm -- -D warnings || exit
CARGO_TARGET_DIR=${CARGO_TARGET_DIR} cargo clippy --release --features sidechain -- -D warnings || exit
CARGO_TARGET_DIR=${CARGO_TARGET_DIR} cargo clippy --release --features teeracle -- -D warnings || exit
CARGO_TARGET_DIR=${CARGO_TARGET_DIR} cargo clippy --release --features offchain-worker -- -D warnings || exit


cd "${root_dir}/tee-worker/enclave-runtime" || exit
taplo fmt
CARGO_TARGET_DIR=${CARGO_TARGET_DIR} cargo clippy --release -- -D warnings || exit
CARGO_TARGET_DIR=${CARGO_TARGET_DIR} cargo clippy --release --features evm -- -D warnings || exit
CARGO_TARGET_DIR=${CARGO_TARGET_DIR} cargo clippy --release --features sidechain -- -D warnings || exit
CARGO_TARGET_DIR=${CARGO_TARGET_DIR} cargo clippy --release --features teeracle -- -D warnings || exit
CARGO_TARGET_DIR=${CARGO_TARGET_DIR} cargo clippy --release --features offchain-worker -- -D warnings || exit

cd "${root_dir}/tee-worker" || exit
#RUST_LOG=info CARGO_TARGET_DIR=/root/work/tmp SKIP_WASM_BUILD=1 cargo test --release -- --show-output
RUST_LOG=info CARGO_TARGET_DIR=${CARGO_TARGET_DIR} SKIP_WASM_BUILD=1 cargo test --release -- --show-output
SGX_MODE=SW SKIP_WASM_BUILD=1 make

cd "${root_dir}/tee-worker/bin" || exit
./integritee-service test --all

end=$(date +%s)
echo "Elapsed Time: $((end-start)) seconds"
