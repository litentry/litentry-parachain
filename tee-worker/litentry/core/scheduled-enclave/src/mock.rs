use crate::{
	error::{Error, Result},
	ScheduledEnclaveMap,
};
pub use itp_sgx_io::SealedIO;
pub use std::path::PathBuf;

#[derive(Clone, Debug, Default)]
pub struct ScheduledEnclaveSealMock {}

impl ScheduledEnclaveSealMock {
	pub fn new() -> Self {
		Self {}
	}
}

impl SealedIO for ScheduledEnclaveSealMock {
	type Error = Error;
	type Unsealed = ScheduledEnclaveMap;

	fn unseal(&self) -> Result<Self::Unsealed> {
		Ok(ScheduledEnclaveMap::default())
	}

	fn seal(&self, _unsealed: &Self::Unsealed) -> Result<()> {
		Ok(())
	}
}
