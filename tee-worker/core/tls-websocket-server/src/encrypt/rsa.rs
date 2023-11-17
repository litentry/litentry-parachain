use crate::error::{WebSocketError, WebSocketResult};
use core::num::NonZeroU32;
#[cfg(not(feature = "rsa"))]
use sgx_crypto_helper::RsaKeyPair;

#[derive(Clone)]
pub struct RsaPrivateKey(
	#[cfg(feature = "rsa")] rsa::RsaPrivateKey,
	#[cfg(not(feature = "rsa"))] sgx_crypto_helper::rsa3072::Rsa3072KeyPair,
);

#[derive(Clone)]
pub struct RsaPublicKey(
	#[cfg(feature = "rsa")] rsa::RsaPublicKey,
	#[cfg(not(feature = "rsa"))] sgx_crypto_helper::rsa3072::Rsa3072PubKey,
);

impl RsaPrivateKey {
	pub fn new() -> WebSocketResult<Self> {
		Ok(Self(
			#[cfg(feature = "rsa")]
			{
				let mut rng = ThreadRng::new();
				rsa::RsaPrivateKey::new(&mut rng, 3072)?
			},
			#[cfg(not(feature = "rsa"))]
			sgx_crypto_helper::rsa3072::Rsa3072KeyPair::new()?,
		))
	}

	pub fn encrypt(&self, v: &[u8]) -> WebSocketResult<std::vec::Vec<u8>> {
		#[cfg(feature = "rsa")]
		{
			let mut rng = ThreadRng::new();
			Ok(self.0.to_public_key().encrypt(&mut rng, rsa::Pkcs1v15Encrypt, v)?)
		}
		#[cfg(not(feature = "rsa"))]
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
		#[cfg(feature = "rsa")]
		{
			Ok(self.0.decrypt(rsa::Pkcs1v15Encrypt, v)?)
		}
		#[cfg(not(feature = "rsa"))]
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
			#[cfg(feature = "rsa")]
			self.0.to_public_key(),
			#[cfg(not(feature = "rsa"))]
			self.0.export_pubkey().unwrap(),
		)
	}
}

impl RsaPublicKey {
	pub fn encrypt(&self, v: &[u8]) -> WebSocketResult<std::vec::Vec<u8>> {
		#[cfg(feature = "rsa")]
		{
			let mut rng = ThreadRng::new();
			Ok(self.0.encrypt(&mut rng, rsa::Pkcs1v15Encrypt, v)?)
		}
		#[cfg(not(feature = "rsa"))]
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
			#[cfg(feature = "rsa")]
			hack_rsa::to_rsa_priv_key(key)?,
			#[cfg(not(feature = "rsa"))]
			*key,
		))
	}
}

// sgx_ucrypto may fail on some machine, use crate rsa instead
#[cfg(feature = "rsa")]
mod hack_rsa {
	use rsa::{BigUint, RsaPrivateKey};
	#[cfg(feature = "sgx")]
	use std::vec;
	#[cfg(feature = "std")]
	use ::{
		serde_big_array::big_array,
		serde_derive::{Deserialize, Serialize},
		serde_json::{from_value, to_value},
	};
	#[cfg(feature = "sgx")]
	use ::{
		serde_big_array_sgx::big_array,
		serde_derive_sgx::{Deserialize, Serialize},
		serde_json_sgx::{from_value, to_value},
	};

	use crate::error::{WebSocketError, WebSocketResult};
	big_array! { BigArray; }

	const SGX_RSA3072_KEY_SIZE: usize = 384;
	const SGX_RSA3072_PRI_EXP_SIZE: usize = 384;
	const SGX_RSA3072_PUB_EXP_SIZE: usize = 4;

	// Mimics the serialized format of sgx_crypto_helper's structure
	#[cfg(feature = "std")]
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
	#[cfg(feature = "sgx")]
	#[derive(Serialize, Deserialize, Clone, Copy)]
	#[serde(crate = "serde_sgx")]
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
		BigUint::from_bytes_be(v)
	}

	pub fn to_rsa_priv_key(
		key: &sgx_crypto_helper::rsa3072::Rsa3072KeyPair,
	) -> WebSocketResult<RsaPrivateKey> {
		//There is no way else to construct Rsa3072KeyPair without calling sgx_ucrypto
		let value = to_value(key)?;
		let key: Rsa3072KeyPair = from_value(value)?;
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
