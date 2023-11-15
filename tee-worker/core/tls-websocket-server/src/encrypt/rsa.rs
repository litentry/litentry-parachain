use crate::error::{WebSocketError, WebSocketResult};
use core::num::NonZeroU32;
#[cfg(feature = "sgx")]
use sgx_crypto_helper::RsaKeyPair;

#[derive(Clone)]
pub struct RsaPrivateKey(
	#[cfg(feature = "std")] rsa::RsaPrivateKey,
	#[cfg(feature = "sgx")] sgx_crypto_helper::rsa3072::Rsa3072KeyPair,
);

#[derive(Clone)]
pub struct RsaPublicKey(
	#[cfg(feature = "std")] rsa::RsaPublicKey,
	#[cfg(feature = "sgx")] sgx_crypto_helper::rsa3072::Rsa3072PubKey,
);

impl RsaPrivateKey {
	pub fn new() -> WebSocketResult<Self> {
		Ok(Self(
			#[cfg(feature = "std")]
			{
				let mut rng = ThreadRng::new();
				rsa::RsaPrivateKey::new(&mut rng, 3072)?
			},
			#[cfg(feature = "sgx")]
			sgx_crypto_helper::rsa3072::Rsa3072KeyPair::new()?,
		))
	}

	pub fn encrypt(&self, v: &[u8]) -> WebSocketResult<std::vec::Vec<u8>> {
		#[cfg(feature = "std")]
		{
			let mut rng = ThreadRng::new();
			Ok(self.0.to_public_key().encrypt(&mut rng, rsa::Pkcs1v15Encrypt, v)?)
		}
		#[cfg(feature = "sgx")]
		{
			let mut result = Default::default();
			let size = self.0.encrypt_buffer(v, &mut result)?;
			while result.len() > size {
				result.pop();
			}
			Ok(result)
		}
	}

	pub fn decrypt(&self, v: &[u8]) -> WebSocketResult<std::vec::Vec<u8>> {
		#[cfg(feature = "std")]
		{
			Ok(self.0.decrypt(rsa::Pkcs1v15Encrypt, v)?)
		}
		#[cfg(feature = "sgx")]
		{
			let mut result = Default::default();
			let size = self.0.decrypt_buffer(v, &mut result)?;
			while result.len() > size {
				result.pop();
			}
			Ok(result)
		}
	}

	pub fn to_public_key(&self) -> RsaPublicKey {
		RsaPublicKey(
			#[cfg(feature = "std")]
			self.0.to_public_key(),
			#[cfg(feature = "sgx")]
			self.0.export_pubkey().unwrap(),
		)
	}
}

impl RsaPublicKey {
	pub fn encrypt(&self, v: &[u8]) -> WebSocketResult<std::vec::Vec<u8>> {
		#[cfg(feature = "std")]
		{
			let mut rng = ThreadRng::new();
			Ok(self.0.encrypt(&mut rng, rsa::Pkcs1v15Encrypt, v)?)
		}
		#[cfg(feature = "sgx")]
		{
			let mut result = Default::default();
			let size = self.0.encrypt_buffer(v, &mut result)?;
			while result.len() > size {
				result.pop();
			}
			Ok(result)
		}
	}
}

impl TryFrom<&sgx_crypto_helper::rsa3072::Rsa3072KeyPair> for RsaPrivateKey {
	type Error = WebSocketError;
	fn try_from(key: &sgx_crypto_helper::rsa3072::Rsa3072KeyPair) -> Result<Self, Self::Error> {
		Ok(Self(
			#[cfg(feature = "std")]
			hack_rsa::to_rsa_priv_key(key)?,
			#[cfg(feature = "sgx")]
			*key,
		))
	}
}

// sgx_ucrypto may fail on some machine, use crate rsa instead
#[cfg(feature = "std")]
mod hack_rsa {
	use rsa::{BigUint, RsaPrivateKey};
	use serde::{Deserialize, Serialize};
	use serde_big_array::big_array;

	use crate::error::{WebSocketError, WebSocketResult};
	big_array! { BigArray; }

	const SGX_RSA3072_KEY_SIZE: usize = 384;
	const SGX_RSA3072_PRI_EXP_SIZE: usize = 384;
	const SGX_RSA3072_PUB_EXP_SIZE: usize = 4;

	// Mimics the serialized format of sgx_crypto_helper's structure
	#[derive(Serialize, Deserialize, Clone, Copy)]
	struct Rsa3072KeyPair {
		#[serde(with = "BigArray")]
		n: [u8; SGX_RSA3072_KEY_SIZE],
		#[serde(with = "BigArray")]
		d: [u8; SGX_RSA3072_PRI_EXP_SIZE],
		e: [u8; SGX_RSA3072_PUB_EXP_SIZE],
		#[serde(with = "BigArray")]
		p: [u8; SGX_RSA3072_KEY_SIZE / 2],
		#[serde(with = "BigArray")]
		q: [u8; SGX_RSA3072_KEY_SIZE / 2],
		#[serde(with = "BigArray")]
		dmp1: [u8; SGX_RSA3072_KEY_SIZE / 2],
		#[serde(with = "BigArray")]
		dmq1: [u8; SGX_RSA3072_KEY_SIZE / 2],
		#[serde(with = "BigArray")]
		iqmp: [u8; SGX_RSA3072_KEY_SIZE / 2],
	}

	impl TryFrom<Rsa3072KeyPair> for RsaPrivateKey {
		type Error = WebSocketError;
		fn try_from(v: Rsa3072KeyPair) -> WebSocketResult<Self> {
			let mut result = RsaPrivateKey::from_components(
				to_big_uint(&v.n),
				to_big_uint(&v.e),
				to_big_uint(&v.d),
				vec![to_big_uint(&v.p), to_big_uint(&v.q)],
			)?;
			result.precompute()?;
			Ok(result)
		}
	}

	fn to_big_uint(mut v: &[u8]) -> BigUint {
		while let Some((0, rest)) = v.split_last() {
			v = rest;
		}
		BigUint::from_bytes_le(v)
	}

	pub fn to_rsa_priv_key(
		key: &sgx_crypto_helper::rsa3072::Rsa3072KeyPair,
	) -> WebSocketResult<RsaPrivateKey> {
		//There is no way else to construct Rsa3072KeyPair without calling sgx_ucrypto
		let value = serde_json::to_value(key)?;
		let key: Rsa3072KeyPair = serde_json::from_value(value)?;
		key.try_into()
	}
}

#[cfg(feature = "std")]
use rand as rand_x;
#[cfg(feature = "sgx")]
use rand_sgx as rand_x;

use rand_x::RngCore;

pub struct ThreadRng(rand_x::rngs::ThreadRng);

impl ThreadRng {
	pub fn new() -> Self {
		Self(rand_x::thread_rng())
	}
}

impl ::rand_core::CryptoRng for ThreadRng {}
impl ::rand_core::RngCore for ThreadRng {
	fn next_u32(&mut self) -> u32 {
		self.0.next_u32()
	}

	fn next_u64(&mut self) -> u64 {
		self.0.next_u64()
	}

	fn fill_bytes(&mut self, dest: &mut [u8]) {
		self.0.fill_bytes(dest)
	}

	fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), ::rand_core::Error> {
		self.0
			.try_fill_bytes(dest)
			.map_err(|e| e.code().unwrap_or_else(|| NonZeroU32::new(u32::MAX).unwrap()).into())
	}
}
