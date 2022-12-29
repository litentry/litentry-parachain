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
#![cfg_attr(not(feature = "std"), no_std)]
//!Primitives for all pallets

#[cfg(not(feature = "std"))]
use sp_std::vec::Vec;

/// Substrate runtimes provide no string type. Hence, for arbitrary data of varying length the
/// `Vec<u8>` is used. In the polkadot-js the typedef `Text` is used to automatically
/// utf8 decode bytes into a string.
#[cfg(not(feature = "std"))]
pub type PalletString = Vec<u8>;

#[cfg(feature = "std")]
pub type PalletString = String;

pub trait AsByteOrNoop {
	fn as_bytes_or_noop(&self) -> &[u8];
}

impl AsByteOrNoop for PalletString {
	#[cfg(feature = "std")]
	fn as_bytes_or_noop(&self) -> &[u8] {
		self.as_bytes()
	}

	#[cfg(not(feature = "std"))]
	fn as_bytes_or_noop(&self) -> &[u8] {
		self
	}
}
