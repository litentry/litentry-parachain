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

use crate::{
	vip3::{VIP3SBTInfo, VIP3SBTLogicInterface},
	*,
};
use lc_credentials::{vip3::UpdateVIP3MembershipCardCredential, IssuerRuntimeVersion};
use lc_data_providers::DataProviderConfig;
use litentry_primitives::VIP3MembershipCardLevel;

pub fn build(
	req: &AssertionBuildRequest,
	level: VIP3MembershipCardLevel,
	data_provider_config: &DataProviderConfig,
) -> Result<Credential> {
	debug!("Building VIP3 membership card level: {:?}", level);

	let identities = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();

	let mut sbt = VIP3SBTInfo::new(data_provider_config)
		.map_err(|e| Error::RequestVCFailed(Assertion::VIP3MembershipCard(level.clone()), e))?;
	let value = sbt
		.has_card_level(addresses, &level)
		.map_err(|e| Error::RequestVCFailed(Assertion::VIP3MembershipCard(level.clone()), e))?;

	let runtime_version = IssuerRuntimeVersion {
		parachain: req.parachain_runtime_version,
		sidechain: req.sidechain_runtime_version,
	};

	match Credential::new(&req.who, &req.shard, &runtime_version) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.update_vip3_membership_card(level, value);
			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::VIP3MembershipCard(level), e.into_error_detail()))
		},
	}
}
