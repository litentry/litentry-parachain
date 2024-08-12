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

use bc_enclave_registry::EnclaveRegistryLookup;
use bc_musig2_ceremony::CeremonyCommand;
use codec::Encode;
use litentry_primitives::Identity;

#[derive(Encode, Debug)]
pub enum KillCeremonyError {
	InvalidSigner,
}

pub fn handle<ER: EnclaveRegistryLookup>(
	signer: Identity,
	enclave_registry: &ER,
) -> Result<CeremonyCommand, KillCeremonyError> {
	let is_valid_signer = match signer {
		Identity::Substrate(address) => enclave_registry.contains_key(&address),
		_ => false,
	};
	if !is_valid_signer {
		return Err(KillCeremonyError::InvalidSigner)
	}

	match signer {
		Identity::Substrate(_) => Ok(CeremonyCommand::KillCeremony),
		_ => Err(KillCeremonyError::InvalidSigner),
	}
}
