use codec::Encode;
use sp_std::prelude::*;

pub fn addr_from_sig(msg: [u8; 32], sig: [u8; 65]) -> Result<[u8; 20], sp_io::EcdsaVerifyError> {
	let pubkey = sp_io::crypto::secp256k1_ecdsa_recover(&sig, &msg)?;
	let hashed_pk = sp_io::hashing::keccak_256(&pubkey);

	let mut addr = [0u8; 20];
	addr[..20].copy_from_slice(&hashed_pk[12..32]);
	Ok(addr)
}

/// Returns a eth_sign-compatible hash of data to sign.
/// The data is prefixed with special message to prevent
/// malicious DApps from using the function to sign forged transactions.
pub fn eth_data_hash(mut data: Vec<u8>) -> Result<[u8; 32], &'static str> {
	const MSG_LEN: usize = 51;
	if data.len() != MSG_LEN {
		log::error!(
			"Ethereum message has an unexpected length {} !!! Expected is {}.",
			data.len(),
			MSG_LEN
		);
		return Err("Unexpected ethereum message length!")
	}
	let mut length_bytes = usize_to_u8_array(data.len())?;
	let mut eth_data = b"\x19Ethereum Signed Message:\n".encode();
	eth_data.append(&mut length_bytes);
	eth_data.append(&mut data);
	Ok(sp_io::hashing::keccak_256(&eth_data))
}

/// Convert a usize type to a u8 array.
/// The input is first converted as a string with decimal presentation,
/// and then this string is converted to a byte array with UTF8 encoding.
/// To avoid unnecessary complexity, the current function supports up to
/// 2 digits unsigned decimal (range 0 - 99)
fn usize_to_u8_array(length: usize) -> Result<Vec<u8>, &'static str> {
	if length >= 100 {
		Err("Unexpected ethereum message length!")
	} else {
		let digits = b"0123456789".encode();
		let tens = length / 10;
		let ones = length % 10;

		let mut vec_res: Vec<u8> = Vec::new();
		if tens != 0 {
			vec_res.push(digits[tens]);
		}
		vec_res.push(digits[ones]);
		Ok(vec_res)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use hex::decode;

	// A test helper function to add ethereum prefix before message hashing
	pub fn eth_data_hash_test_helper(mut data: Vec<u8>) -> [u8; 32] {
		let mut message_data = format!("\x19Ethereum Signed Message:\n{}", data.len()).into_bytes();
		message_data.append(&mut data);
		sp_io::hashing::keccak_256(&message_data)
	}

	#[test]
	fn correct_recover() {
		let msg = decode("61626364656667").unwrap();
		let msg = eth_data_hash_test_helper(msg);

		let sig_bytes = decode("5900a81f236e27be7ee2c796e0de9b383aadcd8b3c53fd881dd378f4c2bc1a54406be632a464c197131c668432f32a966a19354920686a8f8fdd9c9ab0a0dd011b").unwrap();
		let mut sig = [0u8; 65];
		sig[0..65].copy_from_slice(&sig_bytes[0..65]);

		let addr_expected_bytes = decode("Fe7cef4F3A7eF57Ac2401122fB51590bfDf9350a").unwrap();
		let mut addr_expected = [0u8; 20];
		addr_expected[0..20].copy_from_slice(&addr_expected_bytes[0..20]);

		let addr = addr_from_sig(msg, sig).ok().unwrap();
		assert_eq!(addr, addr_expected);
	}

	#[test]
	fn wrong_msg() {
		let msg = decode("626364656667").unwrap();
		let msg = eth_data_hash_test_helper(msg);

		let sig_bytes = decode("5900a81f236e27be7ee2c796e0de9b383aadcd8b3c53fd881dd378f4c2bc1a54406be632a464c197131c668432f32a966a19354920686a8f8fdd9c9ab0a0dd011b").unwrap();
		let mut sig = [0u8; 65];
		sig[0..65].copy_from_slice(&sig_bytes[0..65]);

		let addr_expected_bytes = decode("Fe7cef4F3A7eF57Ac2401122fB51590bfDf9350a").unwrap();
		let mut addr_expected = [0u8; 20];
		addr_expected[0..20].copy_from_slice(&addr_expected_bytes[0..20]);

		let addr = addr_from_sig(msg, sig).ok().unwrap();
		assert_ne!(addr, addr_expected);
	}

	#[test]
	fn sig_from_another_addr() {
		let msg = decode("61626364656667").unwrap();
		let msg = eth_data_hash_test_helper(msg);

		let sig_bytes = decode("a4543cd17d07a9b5207bbf4ccf3c9d47e0a292a6ce461427ebc50de24387887b14584651c3bc11376ba9fe662df325ced20f5c30dd782b6bee15cb474c206a341b").unwrap();
		let mut sig = [0u8; 65];
		sig[0..65].copy_from_slice(&sig_bytes[0..65]);

		let addr_expected_bytes = decode("Fe7cef4F3A7eF57Ac2401122fB51590bfDf9350a").unwrap();
		let mut addr_expected = [0u8; 20];
		addr_expected[0..20].copy_from_slice(&addr_expected_bytes[0..20]);

		let addr = addr_from_sig(msg, sig).ok().unwrap();
		assert_ne!(addr, addr_expected);
	}

	#[test]
	fn msg_with_unexpected_length() {
		let msg = b"Link Litentry: 0123456789abcdef0123456789abcdef999".encode();
		assert_eq!(Err("Unexpected ethereum message length!"), eth_data_hash(msg));
	}

	#[test]
	fn msg_with_expected_length() {
		let msg = b"Link Litentry: 0123456789abcdef0123456789abcdef9999".encode();
		let res = eth_data_hash(msg.clone()).ok().unwrap();
		assert_eq!(eth_data_hash_test_helper(msg), res);
	}

	// Test input with more than 2 digits
	#[test]
	fn usize_to_u8_array_input_too_large() {
		let len: usize = 105;
		assert_eq!(Err("Unexpected ethereum message length!"), usize_to_u8_array(len))
	}

	// Test inputs with one and two digits respectively
	// UTF8 Table:
	// 4 - 0x34 - 52
	// 0 - 0x30 - 48
	#[test]
	fn usize_to_u8_array_input_one_digit() {
		let len: usize = 4;
		assert_eq!(Ok(vec![52]), usize_to_u8_array(len))
	}

	#[test]
	fn usize_to_u8_array_input_two_digits() {
		let len: usize = 40;
		assert_eq!(Ok(vec![52, 48]), usize_to_u8_array(len))
	}
}
