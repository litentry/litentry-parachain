#!/usr/bin/env bash
# It is recommended to execute the script before commit
# which will help us to reduce test/fmt/clippy failures in CI

set -eo pipefail

function worker_clippy() {
    cargo clippy -- -D warnings
    cargo clippy --features evm -- -D warnings
    cargo clippy --features sidechain -- -D warnings
    cargo clippy --features teeracle -- -D warnings
    cargo clippy --features offchain-worker -- -D warnings
}

function parachain_check() {
    make clippy
    cargo test --locked -p pallet-* --lib
    cargo test --locked -p pallet-* --lib --features=skip-ias-check
    cargo test --locked -p pallet-* --lib --features=runtime-benchmarks
    cargo test --locked -p pallet-* --lib --features=skip-ias-check,runtime-benchmarks
    cargo test --locked -p rococo-parachain-runtime --lib
    cargo test --locked -p litmus-parachain-runtime --lib
    cargo test --locked -p litentry-parachain-runtime --lib
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
# make shellcheck # _shellcheck is not enforced in CI though

echo "[Step 1], Parachain clippy"
cd "$root_dir" && parachain_check

echo "[Step 2], Worker clippy"
cd "$root_dir/tee-worker" && worker_clippy

echo "[Step 3], Enclave clippy"
cd "$root_dir/tee-worker/enclave-runtime" && worker_clippy

echo "[Step 4], Worker cargo test"
cd "$root_dir/tee-worker"
RUST_LOG=info SKIP_WASM_BUILD=1 cargo test -- --show-output

echo "[Step 5], Service test"
clean_up
cd "$root_dir/tee-worker"
SGX_MODE=SW SKIP_WASM_BUILD=1 make
cd "$root_dir/tee-worker/bin"
./litentry-worker test --all

end=$(date +%s)
echo "Elapsed Time: $((end-start)) seconds"
