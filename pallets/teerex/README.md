# pallet-teerex

A pallet for [Integritee](https://integritee.network) that acts as a verified registry for SGX enclaves. Its goal is to provide public auditability of remote attestation of SGX enclaves. Given deterministic builds of enclave code, this pallet closes the trust gap from source code to the MRENCLAVE of an enclave running on a genuine Intel SGX platfrom. Without the need for a license with Intel, everyone can verify what code is executed by registered service providers and that it is executed with confidentiality. A blockchain that integrates this pallet will, therefore, act as a public registry of remote attestated services.

The pallet also acts as an indirect-invocation proxy for calls to the confidential state transition function executed in SGX enclaves off-chain.

More documentation available at:
* High-level: https://www.integritee.network/for-developer
* In-depth: https://book.integritee.network/

## IAS verify

A helper crate that verifies IAS report certificates against Intel'x root CA (hard-coded). It also parses IAs reports and extracts information for filtering and registering by pallet-teerex
## Build

Install Rust:
```bash
curl https://sh.rustup.rs -sSf | sh
```

In order to compile *ring* into wasm, you'll need LLVM-9 or above or you'll get linker errors. Here the instructions for Ubuntu 18.04

```bash
wget https://apt.llvm.org/llvm.sh
chmod +x llvm.sh
sudo ./llvm.sh 10
export CC=/usr/bin/clang-10
export AR=/usr/bin/llvm-ar-10
# if you already built, make sure to run cargo clean
```

## Test

Run all unit tests with 

```
cargo test --all
```

