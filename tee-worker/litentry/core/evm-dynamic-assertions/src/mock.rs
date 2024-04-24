use crate::sealing::{SealingError, UnsealedAssertions};
pub use itp_sgx_io::SealedIO;
pub use std::path::PathBuf;

#[derive(Clone, Debug, Default)]
pub struct AssertionsSealMock {}

impl AssertionsSealMock {
	pub fn new() -> Self {
		Self {}
	}
}

impl SealedIO for AssertionsSealMock {
	type Error = SealingError;
	type Unsealed = UnsealedAssertions;

	fn unseal(&self) -> Result<Self::Unsealed, Self::Error> {
		Ok(UnsealedAssertions::default())
	}

	fn seal(&self, _unsealed: &Self::Unsealed) -> Result<(), Self::Error> {
		Ok(())
	}
}
