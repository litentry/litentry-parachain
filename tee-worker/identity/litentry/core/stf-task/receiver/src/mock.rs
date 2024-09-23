use itp_top_pool_author::{error::Result, mocks::GLOBAL_MOCK_AUTHOR_API};

use std::{sync::mpsc::Receiver, vec::Vec};

pub const COMMON_SEED: &[u8] =
	b"crouch whisper apple ladder skull blouse ridge oven despair cloth pony";

pub fn init_global_mock_author_api() -> Result<Receiver<Vec<u8>>> {
	let (sender, receiver) = std::sync::mpsc::channel();
	let mut stf_task_storage = GLOBAL_MOCK_AUTHOR_API.lock().unwrap();
	*stf_task_storage = Some(sender);
	Ok(receiver)
}
