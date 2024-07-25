# 🏥 post checks

This package features a series of tests to assert if the given Parachain Node and Enclave Worker are healthy and ready for clients to connect.

> ⚠️ Heads-up: the assertions are only meant to be run against tee-prod only for the moment.

## Quick start

1. Install dependencies

    ```
    pnpm install
    ```

1. Target the desired environment (optional)

    Set the `LITENTRY_NETWORK` environment to any of the following values:

    - `litentry-prod`: (default) will point to `tee-prod`'s Enclave.
    - `litentry-dev`: will point to `tee-dev`'s Enclave.
    - `litentry-staging`: will point to `tee-staging`'s Enclave.
    - `litentry-local`: will point to a local enclave `ws://localhost:2000`

1. Run the checks

    ```
    pnpm start
    ```

    Running the checks against a specific environment

    ```
    LITENTRY_NETWORK=litentry-prod pnpm start
    ```
