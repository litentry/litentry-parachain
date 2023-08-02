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

use crate::{ensure, Error, Result};
use ita_stf::helpers::get_expected_raw_message;
use itp_types::Index;
use itp_utils::stringify::account_id_to_string;
use litentry_primitives::{
	ErrorDetail, Identity, UserShieldingKeyNonceType, UserShieldingKeyType, Web3ValidationData,
};
use log::*;

pub fn verify(
	who: &Identity,
	identity: &Identity,
	sidechain_nonce: Index,
	key: UserShieldingKeyType,
	nonce: UserShieldingKeyNonceType,
	data: &Web3ValidationData,
) -> Result<()> {
	debug!("verify web3 identity, who: {}", account_id_to_string(&who));
	let raw_msg = get_expected_raw_message(who, identity, sidechain_nonce, key, nonce);

	ensure!(
		raw_msg.as_slice() == data.message().as_slice(),
		Error::LinkIdentityFailed(ErrorDetail::UnexpectedMessage)
	);

	// TODO: just to make it backwards compatible
	//       will merge it to `VerifyWeb3SignatureFailed` after the campaign
	if !data.signature().verify(&raw_msg, identity) {
		match data {
			Web3ValidationData::Substrate(_) =>
				return Err(Error::LinkIdentityFailed(ErrorDetail::VerifySubstrateSignatureFailed)),
			Web3ValidationData::Evm(_) =>
				return Err(Error::LinkIdentityFailed(ErrorDetail::VerifyEvmSignatureFailed)),
		}
	}

	Ok(())
}
