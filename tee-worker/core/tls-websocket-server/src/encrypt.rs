use core::num::NonZeroU32;
#[cfg(feature = "sgx")]
use std::vec::Vec;

use log::warn;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};

pub struct Encryptor {
	key: RsaPublicKey,
}

impl Encryptor {
	pub fn new(key: &[u8]) -> Option<Self> {
		Some(Self {
			key: match rsa::pkcs1::DecodeRsaPublicKey::from_pkcs1_der(key) {
				Ok(key) => key,
				Err(e) => {
					warn!("Parsing DER public key failed: {e:?}");
					return None
				},
			},
		})
	}

	pub fn encrypt(&self, input: &[u8]) -> WebSocketResult<Vec<u8>> {
		let mut rng = ThreadRng::new();
		self.key
			.encrypt(&mut rng, Pkcs1v15Encrypt, input)
			.map_err(WebSocketError::EncryptionError)
	}
}

#[cfg(feature = "std")]
use rand as rand_x;
#[cfg(feature = "sgx")]
use rand_sgx as rand_x;

use rand_x::RngCore;

use crate::error::{WebSocketError, WebSocketResult};

pub struct ThreadRng(rand_x::rngs::ThreadRng);

impl ThreadRng {
	pub fn new() -> Self {
		Self(rand_x::thread_rng())
	}
}

impl rsa::rand_core::CryptoRng for ThreadRng {}
impl rsa::rand_core::RngCore for ThreadRng {
	fn next_u32(&mut self) -> u32 {
		self.0.next_u32()
	}

	fn next_u64(&mut self) -> u64 {
		self.0.next_u64()
	}

	fn fill_bytes(&mut self, dest: &mut [u8]) {
		self.0.fill_bytes(dest)
	}

	fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rsa::rand_core::Error> {
		self.0
			.try_fill_bytes(dest)
			.map_err(|e| e.code().unwrap_or_else(|| NonZeroU32::new(u32::MAX).unwrap()).into())
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use rsa::pkcs1::EncodeRsaPublicKey;

	#[test]
	fn encrypt_test() {
		let mut rng = ThreadRng::new();
		let privkey = rsa::RsaPrivateKey::new(&mut rng, 2048).unwrap();
		let pubkey = privkey.to_public_key().to_pkcs1_der().unwrap();
		let encryptor = Encryptor::new(pubkey.as_bytes()).unwrap();
		const DATA: &[u8] = b"hello world!";
		let result = encryptor.encrypt(DATA).unwrap();
		let restored = privkey.decrypt(Pkcs1v15Encrypt, &result).unwrap();
		assert_eq!(restored, DATA);
	}
}
