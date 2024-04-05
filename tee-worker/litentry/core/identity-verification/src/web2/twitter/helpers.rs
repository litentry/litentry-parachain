use sgx_rand::{thread_rng, Rng};
use std::string::String;

pub fn get_code_verifier() -> String {
	get_random_string(128)
}

pub fn get_state_verifier() -> String {
	get_random_string(32)
}

fn get_random_string(length: usize) -> String {
	let mut rng = thread_rng();
	let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
	let random_string: String = (0..length)
		.map(|_| {
			let idx = rng.gen_range(0, charset.len());
			charset[idx] as char
		})
		.collect();

	random_string
}
