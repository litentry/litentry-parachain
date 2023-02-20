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

// This file includes the predefined rulesets and the corresponding parameters
// when requesting VCs.

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::chrono::{offset::Utc as TzUtc, DateTime, NaiveDateTime};

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(feature = "std")]
use chrono::offset::Utc as TzUtc;

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::{traits::ConstU32, BoundedVec};
use std::{format, string::String};

pub type Balance = u128;
type MaxStringLength = ConstU32<64>;
pub type ParameterString = BoundedVec<u8, MaxStringLength>;

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum Assertion {
	A1,
	A2(ParameterString),                                   // (guild_id)
	A3(ParameterString, ParameterString, ParameterString), // (guild_id, channel_id, role_id)
	A4(Balance),                                           // (minimum_amount)
	A5(ParameterString, ParameterString),                  // (twitter_account, tweet_id)
	A6,
	A7(Balance), // (minimum_amount)
	A8,
	A9,
	A10(Balance), // (minimum_amount)
	A11(Balance), // (minimum_amount)
	A13(u32),     // (Karma_amount) - TODO: unsupported
}

pub const ASSERTION_FROM_DATE: [&str; 7] = [
	"2017-01-01",
	"2018-01-01",
	"2019-01-01",
	"2020-01-01",
	"2021-01-01",
	"2022-01-01",
	"2023-01-01",
];

pub fn format_assertion_from_date() -> String {
	let mut from_date = String::new();
	for date in ASSERTION_FROM_DATE.iter() {
		from_date.push(',');
		from_date.push_str(date)
	}
	from_date.remove(0);

	from_date
}

pub fn format_assertion_to_date() -> String {
	#[cfg(feature = "std")]
	{
		let now = TzUtc::now();
		format!("{}", now.format("%Y-%m-%d"))
	}

	#[cfg(all(not(feature = "std"), feature = "sgx"))]
	{
		let now = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.expect("system time before Unix epoch");
		let naive =
			NaiveDateTime::from_timestamp_opt(now.as_secs() as i64, now.subsec_nanos() as u32)
				.unwrap();
		let datetime: DateTime<TzUtc> = DateTime::from_utc(naive, TzUtc);

		format!("{}", datetime.format("%Y-%m-%d"))
	}
}
