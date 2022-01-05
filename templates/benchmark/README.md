This folder containers the templates that are used for benchmarking.

`pallet-weight-template.hbs` is based on https://github.com/paritytech/substrate/blob/master/.maintain/frame-weight-template.hbs
with a few modifications:

- use `{{header}}` to configure header from command line
- add `#![allow(clippy::unnecessary_cast)]` to make clippy happy
- `SubstrateWeight` -> `LitentryWeight`