#[cfg(feature = "sgx")]
use thiserror_sgx as thiserror;

use crate::{AssertionId, SmartContractByteCode};
use std::{boxed::Box, string::String, vec::Vec};

pub type UnsealedAssertions = Vec<(AssertionId, (SmartContractByteCode, Vec<String>))>;

#[derive(Debug, thiserror::Error)]
pub enum SealingError {
	#[error(transparent)]
	Other(#[from] Box<dyn std::error::Error + Sync + Send + 'static>),
}

impl From<std::io::Error> for SealingError {
	fn from(e: std::io::Error) -> Self {
		Self::Other(e.into())
	}
}

impl From<codec::Error> for SealingError {
	#[cfg(feature = "std")]
	fn from(e: codec::Error) -> Self {
		Self::Other(e.into())
	}

	#[cfg(feature = "sgx")]
	fn from(e: codec::Error) -> Self {
		Self::Other(std::format!("{:?}", e).into())
	}
}

#[cfg(feature = "std")]
pub mod io {
	use std::{vec, vec::Vec};

	pub fn seal_state(_path: &str, _state: Vec<u8>) {}

	pub fn unseal_state(_path: &str) -> Vec<u8> {
		vec![]
	}
}

#[cfg(feature = "sgx")]
pub mod io {
	use crate::{sealing::SealingError, AssertionId, SmartContractByteCode};
	pub use codec::{Decode, Encode};
	use itp_sgx_io::{seal as io_seal, seal, unseal as io_unseal, unseal, SealedIO};
	use log::info;
	use std::{path::PathBuf, string::String, vec::Vec};

	pub fn seal_state(path: &str, state: Vec<u8>) {
		io_seal(&state, path).unwrap()
	}

	pub fn unseal_state(path: &str) -> Vec<u8> {
		io_unseal(path).unwrap()
	}

	#[derive(Clone, Debug)]
	pub struct AssertionsSeal {
		pub base_path: PathBuf,
	}

	impl AssertionsSeal {
		pub fn new(path: PathBuf) -> Self {
			Self { base_path: path }
		}
	}

	impl SealedIO for AssertionsSeal {
		type Error = SealingError;
		type Unsealed = Vec<(AssertionId, (SmartContractByteCode, Vec<String>))>;

		fn unseal(&self) -> Result<Self::Unsealed, Self::Error> {
			Ok(unseal(self.base_path.clone()).map(|b| Decode::decode(&mut b.as_slice()))??)
		}

		fn seal(&self, unsealed: &Self::Unsealed) -> Result<(), Self::Error> {
			info!("Seal assertions to file: {:?}", unsealed);
			Ok(unsealed.using_encoded(|bytes| seal(bytes, self.base_path.clone()))?)
		}
	}
}
