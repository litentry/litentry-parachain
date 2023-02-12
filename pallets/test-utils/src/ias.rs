/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.

*/

use core::default::Default;
use teerex_primitives::{Enclave, MrEnclave};

pub trait TestEnclave<AccountId, Url> {
	fn test_enclave(pubkey: AccountId) -> Enclave<AccountId, Url>;
	fn with_mr_enclave(self, mr_enclave: MrEnclave) -> Enclave<AccountId, Url>;
	fn with_timestamp(self, timestamp: u64) -> Enclave<AccountId, Url>;
	fn with_url(self, url: Url) -> Enclave<AccountId, Url>;
}

impl<AccountId, Url: Default> TestEnclave<AccountId, Url> for Enclave<AccountId, Url> {
	fn test_enclave(pubkey: AccountId) -> Self {
		Enclave::new(
			pubkey,
			Default::default(),
			Default::default(),
			Default::default(),
			Default::default(),
			Default::default(),
			Default::default(),
		)
	}

	fn with_mr_enclave(mut self, mr_enclave: MrEnclave) -> Self {
		self.mr_enclave = mr_enclave;
		self
	}

	fn with_timestamp(mut self, timestamp: u64) -> Self {
		self.timestamp = timestamp;
		self
	}

	fn with_url(mut self, url: Url) -> Self {
		self.url = url;
		self
	}
}

pub mod setups {
	use super::consts::*;
	use teerex_primitives::MrEnclave;

	#[derive(Copy, Clone)]
	pub struct IasSetup {
		pub cert: &'static [u8],
		pub signer_pub: &'static [u8; 32],
		pub mrenclave: MrEnclave,
		pub timestamp: u64,
	}

	pub const TEST4_SETUP: IasSetup = IasSetup {
		cert: TEST4_CERT,
		signer_pub: TEST4_SIGNER_PUB,
		mrenclave: TEST4_MRENCLAVE,
		timestamp: TEST4_TIMESTAMP,
	};

	pub const TEST5_SETUP: IasSetup = IasSetup {
		cert: TEST5_CERT,
		signer_pub: TEST5_SIGNER_PUB,
		mrenclave: TEST5_MRENCLAVE,
		timestamp: TEST5_TIMESTAMP,
	};

	pub const TEST6_SETUP: IasSetup = IasSetup {
		cert: TEST6_CERT,
		signer_pub: TEST6_SIGNER_PUB,
		mrenclave: TEST6_MRENCLAVE,
		timestamp: TEST6_TIMESTAMP,
	};

	pub const TEST7_SETUP: IasSetup = IasSetup {
		cert: TEST7_CERT,
		signer_pub: TEST7_SIGNER_PUB,
		mrenclave: TEST7_MRENCLAVE,
		timestamp: TEST7_TIMESTAMP,
	};
}

pub mod consts {
	use hex_literal::hex;
	use teerex_primitives::{MrEnclave, MrSigner};

	pub const INCOGNITO_ACCOUNT: [u8; 32] = [
		44, 106, 196, 170, 141, 51, 4, 200, 143, 12, 167, 255, 252, 221, 15, 119, 228, 141, 94, 2,
		132, 145, 21, 17, 52, 41, 40, 220, 157, 130, 48, 176,
	];

	// reproduce with "integritee_service dump_ra"
	pub const TEST4_CERT: &[u8] = include_bytes!("./ias-data/ra_dump_cert_TEST4.der");
	pub const TEST5_CERT: &[u8] = include_bytes!("./ias-data/ra_dump_cert_TEST5.der");
	pub const TEST6_CERT: &[u8] = include_bytes!("./ias-data/ra_dump_cert_TEST6.der");
	pub const TEST7_CERT: &[u8] = include_bytes!("./ias-data/ra_dump_cert_TEST7.der");
	pub const TEST8_CERT: &[u8] = include_bytes!("./ias-data/ra_dump_cert_TEST8_PRODUCTION.der");
	pub const TEST9_CERT: &[u8] = include_bytes!("./ias-data/ra_dump_cert_TEST9_enclave_add.der");

	pub const TEST1_DCAP_QUOTE: &[u8] = include_bytes!("./ias-data/ra_dcap_dump_quote.ra");

	// reproduce with integritee-service signing-key
	pub const TEST4_SIGNER_PUB: &MrSigner =
		include_bytes!("./ias-data/enclave-signing-pubkey-TEST4.bin");
	// equal to TEST4! because of MRSIGNER policy it was possible to change the MRENCLAVE but keep
	// the secret
	pub const TEST5_SIGNER_PUB: &MrSigner =
		include_bytes!("./ias-data/enclave-signing-pubkey-TEST5.bin");
	pub const TEST6_SIGNER_PUB: &MrSigner =
		include_bytes!("./ias-data/enclave-signing-pubkey-TEST6.bin");
	pub const TEST7_SIGNER_PUB: &MrSigner =
		include_bytes!("./ias-data/enclave-signing-pubkey-TEST7.bin");
	pub const TEST8_SIGNER_PUB: &MrSigner =
		include_bytes!("./ias-data/enclave-signing-pubkey-TEST8-PRODUCTION.bin");
	pub const TEST9_SIGNER_PUB: &[u8; 32] =
		include_bytes!("./ias-data/enclave-signing-pubkey-TEST9.bin");

	// reproduce with "make mrenclave" in worker repo root
	// MRSIGNER is always 83d719e77deaca1470f6baf62a4d774303c899db69020f9c70ee1dfc08c7ce9e
	pub const TEST4_MRENCLAVE: MrEnclave =
		hex!("7a3454ec8f42e265cb5be7dfd111e1d95ac6076ed82a0948b2e2a45cf17b62a0");
	pub const TEST5_MRENCLAVE: MrEnclave =
		hex!("f4dedfc9e5fcc48443332bc9b23161c34a3c3f5a692eaffdb228db27b704d9d1");
	pub const TEST6_MRENCLAVE: MrEnclave =
		hex!("f4dedfc9e5fcc48443332bc9b23161c34a3c3f5a692eaffdb228db27b704d9d1");
	pub const TEST7_MRENCLAVE: MrEnclave =
		hex!("f4dedfc9e5fcc48443332bc9b23161c34a3c3f5a692eaffdb228db27b704d9d1");
	// production mode
	// MRSIGNER is 117f95f65f06afb5764b572156b8b525c6230db7d6b1c94e8ebdb7fba068f4e8
	pub const TEST8_MRENCLAVE: MrEnclave =
		hex!("bcf66abfc6b3ef259e9ecfe4cf8df667a7f5a546525dee16822741b38f6e6050");
	pub const TEST9_MRENCLAVE: [u8; 32] =
		hex!("318d72b1fee37a7844da18a108be720561b7e75c4276e0216a0a07760fc421be");

	// unix epoch. must be later than this
	pub const TEST4_TIMESTAMP: u64 = 1587899785000;
	pub const TEST5_TIMESTAMP: u64 = 1587900013000;
	pub const TEST6_TIMESTAMP: u64 = 1587900233000;
	pub const TEST7_TIMESTAMP: u64 = 1587900450000;
	pub const TEST8_TIMESTAMP: u64 = 1634156700000;
	pub const TEST9_TIMESTAMP: u64 = 1673007200000;

	pub const TWENTY_FOUR_HOURS: u64 = 60 * 60 * 24 * 1000;

	pub const URL: &[u8] =
		&[119, 115, 58, 47, 47, 49, 50, 55, 46, 48, 46, 48, 46, 49, 58, 57, 57, 57, 49];
}
