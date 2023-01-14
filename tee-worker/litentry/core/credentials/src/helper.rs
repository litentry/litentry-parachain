use chrono::{DateTime, FixedOffset, NaiveDateTime, SecondsFormat};
use std::{
	string::String,
	time::{SystemTime, UNIX_EPOCH},
};

pub fn now() -> String {
	let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
	let naive = NaiveDateTime::from_timestamp_opt(ts.as_secs() as i64, ts.subsec_nanos()).unwrap();
	let datenow_time = DateTime::<FixedOffset>::from_utc(naive, FixedOffset::east_opt(0).unwrap());
	datenow_time.to_rfc3339_opts(SecondsFormat::Secs, true)
}
