#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_rand as rand;
#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use crate::alloc::string::String;
use rand::{thread_rng, Rng};
use itp_types::Index;
use litentry_primitives::Identity;

pub(crate) fn get_random_string(length: usize) -> String {
	let mut rng = thread_rng();
	let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
	let random_string: String = (0..length)
		.map(|_| {
			#[cfg(all(not(feature = "std"), feature = "sgx"))]
			let idx = rng.gen_range(0, charset.len());
			#[cfg(all(not(feature = "sgx"), feature = "std"))]
			let idx = rng.gen_range(0..charset.len());

			charset[idx] as char
		})
		.collect();

	random_string
}

// verification message format:
// ```
// blake2_256(<sidechain nonce> + <primary account> + <identity-to-be-linked>)
// ```
// where <> means SCALE-encoded
// see https://github.com/litentry/litentry-parachain/issues/1739 and P-174
pub fn get_expected_raw_message(
	who: &Identity,
	identity: &Identity,
	sidechain_nonce: Index,
) -> Vec<u8> {
	let mut payload = Vec::new();
	payload.append(&mut sidechain_nonce.encode());
	payload.append(&mut who.encode());
	payload.append(&mut identity.encode());
	blake2_256(payload.as_slice()).to_vec()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_get_random_string() {
		let random_string = get_random_string(128);
		assert_eq!(random_string.len(), 128);
	}
}
