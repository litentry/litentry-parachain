#!/usr/bin/env bash
# It is recommended to execute the script before commit
# which will help us to reduce test/fmt/clippy failures in CI

set -eo pipefail

function worker_clippy() {
    taplo fmt
    cargo clippy --release -- -D warnings
    cargo clippy --release --features evm -- -D warnings
    cargo clippy --release --features sidechain -- -D warnings
    cargo clippy --release --features teeracle -- -D warnings
    cargo clippy --release --features offchain-worker -- -D warnings
}

root_dir=$(git rev-parse --show-toplevel)
cd "$root_dir"

start=$(date +%s)

make fmt
make clippy
make shellcheck # _shellcheck is not enforced in CI though
cargo test --locked --release -p pallet-* --lib
cargo test --locked --release -p pallet-* --lib --features=skip-ias-check
cargo test --locked --release -p pallet-* --lib --features=runtime-benchmarks
cargo test --locked --release -p pallet-* --lib --features=skip-ias-check,runtime-benchmarks
cargo test --locked --release -p rococo-parachain-runtime --lib
cargo test --locked --release -p litmus-parachain-runtime --lib
cargo test --locked --release -p litentry-parachain-runtime --lib

cd "$root_dir/tee-worker" && worker_clippy

cd "$root_dir/tee-worker/enclave-runtime" && worker_clippy

cd "$root_dir/tee-worker"
RUST_LOG=info SKIP_WASM_BUILD=1 cargo test --release -- --show-output
SGX_MODE=SW SKIP_WASM_BUILD=1 make

cd "$root_dir/tee-worker/bin"
./integritee-service test --all

end=$(date +%s)
echo "Elapsed Time: $((end-start)) seconds"
