// Copyright 2020-2023 Trust Computing GmbH.
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

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use crate::{vec_to_string, Error};
use litentry_primitives::ParameterString;
use std::string::ToString;

pub trait GetAchainableName {
	fn name(&self) -> &'static str;
}

#[derive(Debug)]
pub enum AchainableNameMirror {
	IsAPublicationOnMirror,
	HasWrittenOverQuantityPostsOnMirror,
}

impl GetAchainableName for AchainableNameMirror {
	fn name(&self) -> &'static str {
		match self {
			AchainableNameMirror::IsAPublicationOnMirror => "Is a publication on Mirror",
			AchainableNameMirror::HasWrittenOverQuantityPostsOnMirror =>
				"Has written over quantity posts on Mirror",
		}
	}
}

impl AchainableNameMirror {
	pub fn from(param: ParameterString) -> Result<AchainableNameMirror, Error> {
		let name_str = vec_to_string(param.to_vec())?;

		if name_str == AchainableNameMirror::IsAPublicationOnMirror.name() {
			return Ok(AchainableNameMirror::IsAPublicationOnMirror)
		} else if name_str == AchainableNameMirror::HasWrittenOverQuantityPostsOnMirror.name() {
			return Ok(AchainableNameMirror::HasWrittenOverQuantityPostsOnMirror)
		}

		Err(Error::AchainableError("Invalid Achainable Name".to_string()))
	}
}

#[derive(Debug, PartialEq)]
pub enum AchainableNameBalance {
	BalanceUnderAmount,
	BalanceOverAmount,
}

impl GetAchainableName for AchainableNameBalance {
	fn name(&self) -> &'static str {
		match self {
			AchainableNameBalance::BalanceUnderAmount => "Balance under {amount}",
			AchainableNameBalance::BalanceOverAmount => "Balance over {amount}",
		}
	}
}

impl AchainableNameBalance {
	pub fn from(param: ParameterString) -> Result<AchainableNameBalance, Error> {
		let name_str = vec_to_string(param.to_vec())?;

		if name_str == AchainableNameBalance::BalanceUnderAmount.name() {
			return Ok(AchainableNameBalance::BalanceUnderAmount)
		} else if name_str == AchainableNameBalance::BalanceOverAmount.name() {
			return Ok(AchainableNameBalance::BalanceOverAmount)
		}

		Err(Error::AchainableError("Invalid Achainable Name".to_string()))
	}
}
