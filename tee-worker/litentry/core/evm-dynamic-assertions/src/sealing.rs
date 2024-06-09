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
	use crate::sealing::{SealingError, UnsealedAssertions};
	use std::vec;

	pub fn seal_state(_path: &str, _state: UnsealedAssertions) -> Result<(), SealingError> {
		Ok(())
	}

	pub fn unseal_state(_path: &str) -> Result<UnsealedAssertions, SealingError> {
		Ok(vec![])
	}
}

#[cfg(feature = "sgx")]
pub mod io {
	use crate::sealing::{SealingError, UnsealedAssertions};
	pub use codec::{Decode, Encode};
	use itp_sgx_io::{seal as io_seal, unseal as io_unseal, SealedIO};
	use log::{debug, info};
	use sgx_tprotected_fs::SgxFile;
	use std::{path::PathBuf, vec};

	pub fn seal_state(path: &str, state: UnsealedAssertions) -> Result<(), SealingError> {
		AssertionsSeal::new(path.into()).seal(&state)
	}

	pub fn unseal_state(path: &str) -> Result<UnsealedAssertions, SealingError> {
		AssertionsSeal::new(path.into()).unseal()
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
		type Unsealed = UnsealedAssertions;

		fn unseal(&self) -> Result<Self::Unsealed, Self::Error> {
			if SgxFile::open(self.base_path.clone()).is_err() {
				info!("Assertions seal file not found, creating new! {:?}", self.base_path);
				self.seal(&vec![])?;
			}

			Ok(io_unseal(self.base_path.clone()).map(|b| Decode::decode(&mut b.as_slice()))??)
		}

		fn seal(&self, unsealed: &Self::Unsealed) -> Result<(), Self::Error> {
			debug!("Seal assertions to file: {:?}", unsealed);
			Ok(unsealed.using_encoded(|bytes| io_seal(bytes, self.base_path.clone()))?)
		}
	}
}
