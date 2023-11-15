#[cfg(feature = "sgx")]
use std::{boxed::Box, vec::Vec};

use crate::error::{WebSocketError, WebSocketResult};
use aes_gcm::{aead::Aead, AeadCore, Aes256Gcm, Key, KeyInit};

mod rsa;

use self::rsa::ThreadRng;
pub use self::rsa::{RsaPrivateKey, RsaPublicKey};

pub struct Encryptor {
	key: Box<Aes256Gcm>,
}

impl Encryptor {
	const NONCE_SIZE: usize = 12;

	pub fn export(encryptor: &RsaPublicKey) -> WebSocketResult<(Self, Vec<u8>)> {
		let mut rng = ThreadRng::new();
		let key = Aes256Gcm::generate_key(&mut rng);
		let data = encryptor.encrypt(&key)?;
		Ok((Self { key: Box::new(Aes256Gcm::new(&key)) }, data))
	}

	pub fn import(key: &[u8], decryptor: &RsaPrivateKey) -> WebSocketResult<Self> {
		let key: [u8; 32] = decryptor
			.decrypt(key)?
			.try_into()
			.map_err(|x: Vec<_>| WebSocketError::InvalidUserKeyLength(x.len()))?;
		let key: Key<Aes256Gcm> = key.into();
		Ok(Self { key: Box::new(Aes256Gcm::new(&key)) })
	}

	pub fn encrypt(&self, input: &[u8]) -> WebSocketResult<Vec<u8>> {
		let mut rng = ThreadRng::new();
		let nonce = Aes256Gcm::generate_nonce(&mut rng);
		let cipher_text = self
			.key
			.encrypt(&nonce, input)
			.map_err(|_| WebSocketError::AesEncryptionError)?;
		let mut result = Vec::with_capacity(Self::NONCE_SIZE + cipher_text.len());
		result.extend_from_slice(&nonce);
		result.extend_from_slice(&cipher_text);

		Ok(result)
	}

	pub fn decrypt(&self, input: &[u8]) -> WebSocketResult<Vec<u8>> {
		if input.len() < Self::NONCE_SIZE {
			return Err(WebSocketError::InvalidCipherLength(input.len()))
		}
		let (nonce, input) = input.split_at(Self::NONCE_SIZE);
		let nonce: [u8; Self::NONCE_SIZE] = nonce.try_into().unwrap();
		let plain_text = self
			.key
			.decrypt(&nonce.into(), input)
			.map_err(|_| WebSocketError::AesEncryptionError)?;

		Ok(plain_text)
	}
}
