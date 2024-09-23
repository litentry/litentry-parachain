// Copyright 2020-2024 Trust Computing GmbH.
// This file is part of Litentry.
//
// Litentry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Litentry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

use crate::{MrEnclave, MrSigner};
use hex_literal::hex;

pub fn get_signer<AccountId: From<[u8; 32]>>(pubkey: &[u8; 32]) -> AccountId {
	AccountId::from(*pubkey)
}

pub const INCOGNITO_ACCOUNT: [u8; 32] = [
	44, 106, 196, 170, 141, 51, 4, 200, 143, 12, 167, 255, 252, 221, 15, 119, 228, 141, 94, 2, 132,
	145, 21, 17, 52, 41, 40, 220, 157, 130, 48, 176,
];

// reproduce with "litentry-worker dump_ra"
pub const TEST4_CERT: &[u8] = include_bytes!("./ias-data/ra_dump_cert_TEST4.der");
pub const TEST5_CERT: &[u8] = include_bytes!("./ias-data/ra_dump_cert_TEST5.der");
pub const TEST6_CERT: &[u8] = include_bytes!("./ias-data/ra_dump_cert_TEST6.der");
pub const TEST7_CERT: &[u8] = include_bytes!("./ias-data/ra_dump_cert_TEST7.der");
pub const TEST8_CERT: &[u8] = include_bytes!("./ias-data/ra_dump_cert_TEST8_PRODUCTION.der");
pub const TEST9_CERT: &[u8] = include_bytes!("./ias-data/ra_dump_cert_TEST9_enclave_add.der");

pub const TEST1_DCAP_QUOTE: &[u8] = include_bytes!("./ias-data/ra_dcap_dump_quote.ra");

// reproduce with litentry-worker signing-key
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
