// Copyright 2020-2022 Litentry Technologies GmbH.
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

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use base64_sgx as base64;

use codec::{Decode, Encode};
use std::{string::String, vec::Vec};

#[derive(Encode, Decode, Clone, Debug)]
pub struct EnclaveAdd {
	pub spid: [u8; 16],
	pub nonce: [u8; 16],
	pub sig_rl: Vec<u8>,
	pub quote: Vec<u8>,
}

impl EnclaveAdd {
	pub fn new(spid: [u8; 16], nonce: [u8; 16], sig_rl: Vec<u8>, quote: Vec<u8>) -> Self {
		EnclaveAdd { spid, nonce, sig_rl, quote }
	}

	// correspond with create_ra_report_and_signature
	// concat the information
	pub fn format(&self) -> String {
		let spid: String = base64::encode(self.spid);
		let nonce: String = base64::encode(self.nonce);
		let sig_rl: String = base64::encode(self.sig_rl.clone());
		let quote: String = base64::encode(self.quote.clone());

		spid + "|" + &nonce + "|" + &sig_rl + "|" + &quote
	}
}
