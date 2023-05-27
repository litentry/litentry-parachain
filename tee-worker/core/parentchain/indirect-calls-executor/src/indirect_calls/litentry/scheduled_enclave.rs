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

use crate::{error::Result, IndirectDispatch, IndirectExecutor};
use codec::{Decode, Encode};
use ita_sgx_runtime::{pallet_imt::MetadataOf, Runtime};
use ita_stf::{TrustedCall, TrustedOperation};
use itp_node_api::{
	api_client::ParentchainUncheckedExtrinsic,
	metadata::{
		pallet_imp::IMPCallIndexes, pallet_teerex::TeerexCallIndexes,
		pallet_utility::UtilityCallIndexes, pallet_vcmp::VCMPCallIndexes,
		provider::AccessNodeMetadata,
	},
};
use itp_sgx_crypto::{key_repository::AccessKey, ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_stf_executor::traits::StfEnclaveSigning;
use itp_stf_primitives::types::AccountId;
use itp_top_pool_author::traits::AuthorApi;
use itp_types::{
	Balance, CreateIdentityFn, MrEnclave, RemoveScheduledEnclaveFn, ShardIdentifier,
	SidechainBlockNumber, UpdateScheduledEnclaveFn, H256,
};
use itp_utils::stringify::account_id_to_string;
use lc_scheduled_enclave::{ScheduledEnclaveUpdater, GLOBAL_SCHEDULED_ENCLAVE};
use litentry_primitives::{Identity, ParentchainBlockNumber};
use log::{debug, info, *};
use std::vec::Vec;

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub struct UpdateScheduledEnclaveArgs {
	sbn: SidechainBlockNumber,
	mrenclave: MrEnclave,
}

impl<Executor: IndirectExecutor> IndirectDispatch<Executor> for UpdateScheduledEnclaveArgs {
	type Args = ();
	fn dispatch(&self, executor: &Executor, args: Self::Args) -> Result<()> {
		debug!("execute indirect call: UpdateScheduledEnclave, sidechain_block_number: {}, mrenclave: {:?}", self.sbn, self.mrenclave);
		GLOBAL_SCHEDULED_ENCLAVE.update(self.sbn, self.mrenclave)?;
		Ok(())
	}
}

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub struct RemoveScheduledEnclaveArgs {
	sbn: SidechainBlockNumber,
}

impl<Executor: IndirectExecutor> IndirectDispatch<Executor> for RemoveScheduledEnclaveArgs {
	type Args = ();
	fn dispatch(&self, executor: &Executor, args: Self::Args) -> Result<()> {
		debug!(
			"execute indirect call: RemoveScheduledEnclave, sidechain_block_number: {}",
			self.sbn
		);
		GLOBAL_SCHEDULED_ENCLAVE.remove(self.sbn)?;
		Ok(())
	}
}
