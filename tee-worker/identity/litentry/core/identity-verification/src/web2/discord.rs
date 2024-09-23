use crate::{Error, Result};
use lc_data_providers::discord_official::DiscordMessage;
use litentry_primitives::ErrorDetail;
use std::vec::Vec;

pub fn payload_from_discord(discord: &DiscordMessage) -> Result<Vec<u8>> {
	let data = &discord.content;
	hex::decode(data.strip_prefix("0x").unwrap_or(data.as_str()))
		.map_err(|_| Error::LinkIdentityFailed(ErrorDetail::ParseError))
}
