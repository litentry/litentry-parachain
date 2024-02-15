#!/usr/bin/env bash
# It is recommended to execute the script before commit
# which will help us to reduce test/fmt/clippy failures in CI

set -eo pipefail

function worker_clippy() {
    cargo clippy --release -- -D warnings
    cargo clippy --release --features evm -- -D warnings
    cargo clippy --release --features sidechain -- -D warnings
    cargo clippy --release --features offchain-worker -- -D warnings
}

function bitacross_clippy() {
    cargo clippy --release -- -D warnings
    cargo clippy --release --features evm -- -D warnings
    cargo clippy --release --features offchain-worker -- -D warnings
}

function parachain_check() {
    make clippy
    cargo test --locked --release -p pallet-* --lib
    cargo test --locked --release -p pallet-* --lib --features=skip-ias-check
    cargo test --locked --release -p pallet-* --lib --features=runtime-benchmarks
    cargo test --locked --release -p pallet-* --lib --features=skip-ias-check,runtime-benchmarks
    cargo test --locked --release -p rococo-parachain-runtime --lib
    cargo test --locked --release -p litmus-parachain-runtime --lib
    cargo test --locked --release -p litentry-parachain-runtime --lib
}

function clean_up() {
    cd "$root_dir"
    cargo clean
    cd "$root_dir/tee-worker"
    cargo clean
    cd "$root_dir/tee-worker/enclave-runtime"
    cargo clean
}

root_dir=$(git rev-parse --show-toplevel)

start=$(date +%s)

clean_up

cd "$root_dir"
make fmt

echo "[Step 1], Parachain clippy"
cd "$root_dir" && parachain_check

echo "[Step 2], tee-worker clippy"
cd "$root_dir/tee-worker" && worker_clippy

echo "[Step 3], tee-worker enclave clippy"
cd "$root_dir/tee-worker/enclave-runtime" && worker_clippy

echo "[Step 4], tee-worker cargo test"
cd "$root_dir/tee-worker"
RUST_LOG=info SKIP_WASM_BUILD=1 cargo test --release -- --show-output

echo "[Step 5], tee-worker service test"
clean_up
cd "$root_dir/tee-worker"
SGX_MODE=SW SKIP_WASM_BUILD=1 make
cd "$root_dir/tee-worker/bin"
./litentry-worker test --all

echo "[Step 6], bitacross-worker clippy"
cd "$root_dir/bitacross-worker" && bitacross_clippy

echo "[Step 7], bitacross-worker enclave clippy"
cd "$root_dir/bitacross-worker/enclave-runtime" && bitacross_clippy

echo "[Step 8], bitacross-worker cargo test"
cd "$root_dir/bitacross-worker"
RUST_LOG=info SKIP_WASM_BUILD=1 cargo test --release -- --show-output

echo "[Step 9], bitacross-worker service test"
clean_up
cd "$root_dir/bitacross-worker"
SGX_MODE=SW SKIP_WASM_BUILD=1 make
cd "$root_dir/bitacross-worker/bin"
./bitacross-worker test --all

end=$(date +%s)
echo "Elapsed Time: $((end-start)) seconds"
