Pallet for Litentry's tee-worker registration and management.

Currently it expects the following worker type:
- identity worker
- bitacross worker

It serves as the registry for public information such as MRENCLAVE, worker-endpoint, vc-pubkey etc.

TBA: MAA / RA information

This crate is partially based on `teerex` and `sidechain` crate on https://github.com/integritee-network/pallets/commit/e124aebb2d3d05a9a65f209f8e6304c6790f15d5 - for the code part / file that is (mostly) copied from intergritee, the original licence is kept.