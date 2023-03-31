// Copyright 2020-2023 Litentry Technologies GmbH.
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

use crate::{
	error::{Error, Result},
	ScheduledEnclave,
};
use codec::{Decode, Encode};
use itp_settings::files::SCHEDULED_ENCLAVE_FILE;
use itp_sgx_io::{seal, unseal, StaticSealedIO};
use log::*;
use std::{boxed::Box, fs, sgxfs::SgxFile, sync::Arc};

#[derive(Copy, Clone, Debug)]
pub struct ScheduledEnclaveSeal;

impl StaticSealedIO for ScheduledEnclaveSeal {
	type Error = Error;
	type Unsealed = ScheduledEnclave;

	fn unseal_from_static_file() -> Result<Self::Unsealed> {
		Ok(unseal(SCHEDULED_ENCLAVE_FILE).map(|b| Decode::decode(&mut b.as_slice()))??)
	}

	fn seal_to_static_file(unsealed: &Self::Unsealed) -> Result<()> {
		debug!("Seal scheduled enclave. Current state: {:?}", unsealed);
		Ok(unsealed.using_encoded(|bytes| seal(bytes, SCHEDULED_ENCLAVE_FILE))?)
	}
}
