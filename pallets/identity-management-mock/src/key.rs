// Copyright 2020-2023 Litentry Technologies GmbH.
// This file is part of Litentry.
//
// Litentry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Litentry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

#[cfg(feature = "std")]
extern crate std;

use rsa::pkcs8::DecodePublicKey;
pub use rsa::{
	pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey, EncodeRsaPrivateKey, EncodeRsaPublicKey},
	pkcs1v15::{SigningKey, VerifyingKey},
	Hash, PaddingScheme, PublicKey, PublicKeyParts, RsaPrivateKey, RsaPublicKey,
};

use aes_gcm::{
	aead::{Aead, KeyInit, Payload},
	Aes256Gcm,
};

pub use core_primitives::{AesOutput, NONCE_LEN, USER_SHIELDING_KEY_LEN};

#[rustfmt::skip]
// The hardcoded exemplary shielding keys for mocking
// openssl always generates a mix of X509 pub key and pkcs1 private key
// see https://stackoverflow.com/questions/10783366/how-to-generate-pkcs1-rsa-keys-in-pem-format
//
// commands used to generate keys:
// 1. openssl genrsa -out pallets/identity-management-mock/src/rsa_key_examples/pkcs1/3072-priv.pem 3072
// 2. openssl rsa -in pallets/identity-management-mock/src/rsa_key_examples/pkcs1/3072-priv.pem \
//    -pubout -out pallets/identity-management-mock/src/rsa_key_examples/pkcs1/3072-pub.pem
//
// we use 3072-bit RSA as TEE shielding key
const MOCK_TEE_SHIELDING_KEY_PRIV: &str = include_str!("rsa_key_examples/pkcs1/3072-priv.pem");
const MOCK_TEE_SHIELDING_KEY_PUB: &str = include_str!("rsa_key_examples/pkcs1/3072-pub.pem");

// use a fake nonce for this pallet
// in real situations we should either use Randomness pallet for wasm runtime, or
// Aes256Gcm::generate_nonce for non-wasm case
const MOCK_NONCE: [u8; NONCE_LEN] = [2u8; NONCE_LEN];

pub fn get_mock_tee_shielding_key() -> (RsaPublicKey, RsaPrivateKey) {
	(
		RsaPublicKey::from_public_key_pem(MOCK_TEE_SHIELDING_KEY_PUB).unwrap(),
		RsaPrivateKey::from_pkcs1_pem(MOCK_TEE_SHIELDING_KEY_PRIV).unwrap(),
	)
}

// encrypt the plaintext `data` with given `aad` and `nonce`
pub(crate) fn aes_encrypt(
	key: &[u8; USER_SHIELDING_KEY_LEN],
	data: &[u8],
	aad: &[u8],
	nonce: [u8; NONCE_LEN],
) -> AesOutput {
	let cipher = Aes256Gcm::new(key.into());
	let payload = Payload { msg: data, aad };
	let ciphertext = cipher.encrypt(&nonce.into(), payload).unwrap();
	AesOutput { ciphertext, aad: aad.to_vec(), nonce }
}

// encrypt the plaintext `data` with null aad and random nonce
pub fn aes_encrypt_default(key: &[u8; USER_SHIELDING_KEY_LEN], data: &[u8]) -> AesOutput {
	aes_encrypt(key, data, b"", MOCK_NONCE)
}

// encrypt the given data using the mock tee shielding key
#[cfg(test)]
pub fn tee_encrypt(data: &[u8]) -> Vec<u8> {
	use sha2::Sha256;
	let (public_key, _) = get_mock_tee_shielding_key();
	// encrypt with public key
	let mut rng = rand::thread_rng();

	public_key
		.encrypt(&mut rng, PaddingScheme::new_oaep::<Sha256>(), data)
		.expect("failed to encrypt")
}

#[cfg(test)]
mod test {
	use super::*;
	use aes_gcm::{aead::OsRng, AeadCore};
	use hex_literal::hex;
	use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
	use sha2::{Digest, Sha256};
	use signature::{RandomizedSigner, Verifier};

	#[test]
	fn test_read_rsa_key_pair_works() {
		let (public_key, private_key) = get_mock_tee_shielding_key();
		// openssl rsa -pubin -inform PEM -text -modulus <
		// pallets/identity-management-mock/src/rsa_key_examples/pkcs1/3072-pub.pem
		assert_eq!(&public_key.n().to_bytes_be(), &hex!("E802BF953FA5AEBEF77EE7587F1F94A05C604665EA272DC3A5A421F3566CBB6F9726F2D468B2A932EEA0502651309B4149D892D318C3EABB0C006C506A0F1F12DB7AE1257932F39DD2E2309F137F6274283CE80CCB55A4760AC3A44E96EC74826AA5C001ABBC9749B988063F29A75DF0007364AADA17BD45FC711519F46B04E71CFA04CE8C952BC58F1C4E377CB851CF5F95A66CA5BD2E64065D626848F6F5E5A79268C5DE255E4E408C5D19770AF9F9A89E23961864930FC0FC6AD1863C2F69AEC85AC06CAB5226524FEA21BB1DF702888EF2C50C5EBDCB5F5F4F92818F1CDB37E29363787AF48CF63A68D1C021484628C5C7BE9E359C36E0281BE9A90B3FBF8A86DDA69544D282615F8EC62AE680EA779F8B6D89E132EFFDD4F66A39002C99832BD8B633A2740E030B305A2C72F1D611C707F7E36A15B8D7B3C14572B99838BAA89283998AEA09DED098B819F71241BC55865CF63B732D2B5BC96FBAAFBDA24713D14A44E51616761F436F99B756E56DF05C12B6582CB7CBAB97750CBEB083"));
		assert_eq!(&public_key.e().to_bytes_be(), &hex!("010001"));

		// openssl asn1parse -in <path-to-file>
		assert_eq!(&private_key.n().to_bytes_be(), &hex!("E802BF953FA5AEBEF77EE7587F1F94A05C604665EA272DC3A5A421F3566CBB6F9726F2D468B2A932EEA0502651309B4149D892D318C3EABB0C006C506A0F1F12DB7AE1257932F39DD2E2309F137F6274283CE80CCB55A4760AC3A44E96EC74826AA5C001ABBC9749B988063F29A75DF0007364AADA17BD45FC711519F46B04E71CFA04CE8C952BC58F1C4E377CB851CF5F95A66CA5BD2E64065D626848F6F5E5A79268C5DE255E4E408C5D19770AF9F9A89E23961864930FC0FC6AD1863C2F69AEC85AC06CAB5226524FEA21BB1DF702888EF2C50C5EBDCB5F5F4F92818F1CDB37E29363787AF48CF63A68D1C021484628C5C7BE9E359C36E0281BE9A90B3FBF8A86DDA69544D282615F8EC62AE680EA779F8B6D89E132EFFDD4F66A39002C99832BD8B633A2740E030B305A2C72F1D611C707F7E36A15B8D7B3C14572B99838BAA89283998AEA09DED098B819F71241BC55865CF63B732D2B5BC96FBAAFBDA24713D14A44E51616761F436F99B756E56DF05C12B6582CB7CBAB97750CBEB083"));
		assert_eq!(&private_key.e().to_bytes_be(), &hex!("010001"));
		assert_eq!(&private_key.d().to_bytes_be(), &hex!("3A54BE902A91604ED8F0C9FF60EEB2B262A73DEBCFA40C087D73B7A973582103DC4FC98B87CB2B6907BFBC86F5B0AFC80965EEFC2DE4CBF63CFD3A3E397C15C6EAF188FB9FEE247BD09257C116E8D6FAF746E0DC9E9EA89B98F7392F1D18D3EE1A1C141B176F71E5F24475B599A65FA2C0AA426C062B23C61DC2DC984AF0412A4E09B9FDA830B4F1959A7B3BCE1A954EFAEC280C76DB0A77D175D710FB0F44217A310030873A83EC4EA43B9FF463091830C19996DA09274435B400B32EF9A0BD89AA5ED810FB68939AF56A30094312F5B2B27942DDA3E09DBD0DF3AFD8A29164080CF8D96BA8CB2379DC7BAC6CE384269A56EEBF418CA0292BF749F408405D68AF554126D7C8A4CD33EACA195CF8DE14468986986A4DD9C8C9D8A3DC5A20B7749E4E26313F2C7216A5FC60720BEC94EC8DAE598B08D41B70C7047095D32B77794A8727612488D40BC81574F3F3F5759DDCD812B22A81A7D226B56BE9930E5E9AEBF0605E70F046E61465D3292F46E2F1E2A6C16B079B6AC0003F6F2C3B631839"));
		assert_eq!(&private_key.primes()[0].to_bytes_be(), &hex!("FF710DDA71F7104DB9475474142F70E23E21302419A49627DF01877180BD94A3202E0B274C15D0C5D62770BA38F391588BC57D4032E8A8E72932D2717C70C8C03646FFA7678FE89870644156093951CB829CB27A2203CD12537CCE0C984369133C2C3B13D60A48154A2B63DBCC616C3020840436919591CE848BB4433E9C27E463E966A146EF2FC353793AC957948C49A26124A145E08216AF774CDF5AE60861BDDFABBE255E5C7A012BF7505F8FED6DC37FD04B6799DF7197771D3678184505"));
		assert_eq!(&private_key.primes()[1].to_bytes_be(), &hex!("E884950F5F40402EF56648EF5DA494B00AE95F2C1CE5E70AE76D42ABBDED141672D41AC1B0588AE435CF8BED129F51B04034FD917277035938A3015BAD3204220AD6C8C1A75E2D9C2F19121DB9242A05CB1C871A935925B05DA5E4942989B364613673F5578E2D72340CB5821533CC72858FE72160C2415F7B152A0AF460E3BD99C7FB7383F81DB3A04FF0940695C532EE9F9AC3DBACB4F6A011A07C313D923E3E5BB6D1C11D0BD38FB0D27BF1AC734259878E10D7C95FC1ACCB8119F3A315E7"));
	}

	#[test]
	fn test_encrypt_decrypt_rsa_works() {
		let (public_key, private_key) = get_mock_tee_shielding_key();
		assert_eq!(private_key.to_public_key(), public_key);

		// encrypt with public key
		let mut rng = ChaCha8Rng::from_seed([42; 32]);
		let data = b"hello world";
		let enc_data = public_key
			.encrypt(&mut rng, PaddingScheme::new_oaep::<Sha256>(), &data[..])
			.expect("failed to encrypt");

		// decrypt with private key
		let dec_data = private_key
			.decrypt(PaddingScheme::new_oaep::<Sha256>(), &enc_data)
			.expect("failed to decrypt");
		assert_eq!(&data[..], &dec_data[..]);
	}

	#[test]
	fn test_sign_verify_rsa_works() {
		let (_public_key, private_key) = crate::get_mock_tee_shielding_key();
		let signing_key = SigningKey::new_with_hash(private_key, Hash::SHA2_256);
		let verifying_key: VerifyingKey = (&signing_key).into();

		// sign with private key
		let mut rng = ChaCha8Rng::from_seed([42; 32]);
		let data = b"hello world";
		let digest = Sha256::digest(data).to_vec();
		let signature = signing_key.sign_with_rng(&mut rng, &digest);

		// verify
		verifying_key.verify(&digest, &signature).expect("failed to verify");
	}

	#[test]
	fn test_encrypt_decrypt_aes_gcm_default_works() {
		const PLAINTEXT: &[u8] = b"hello world";
		const KEY: &[u8; USER_SHIELDING_KEY_LEN] =
			&hex!("b52c505a37d78eda5dd34f20c22540ea1b58963cf8e5bf8ffa85f9f2492505b4");
		let output = aes_encrypt_default(KEY, PLAINTEXT);
		assert_eq!(output.nonce, MOCK_NONCE);
		let cipher = Aes256Gcm::new(KEY.into());
		let decrypted_plaintext =
			cipher.decrypt(&output.nonce.into(), output.ciphertext.as_ref()).unwrap();
		assert_eq!(PLAINTEXT, &decrypted_plaintext);
	}

	#[test]
	fn test_encrypt_decrypt_aes_gcm_random_works() {
		let key = Aes256Gcm::generate_key(&mut OsRng);
		let cipher = Aes256Gcm::new(&key);
		let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
		let ciphertext = cipher.encrypt(&nonce, b"plaintext message".as_ref()).unwrap();
		let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).unwrap();
		assert_eq!(&plaintext, b"plaintext message");
	}

	#[test]
	fn test_encrypt_decrypt_aes_gcm_hardcoded_with_aad_works() {
		// values are from TestVector is RSA crate:
		const KEY: &[u8] =
			&hex!("d33ea320cec0e43dfc1e3d1d8ccca2dd7e30ad3ea18ad7141cc83645d18771ae");
		const NONCE: &[u8] = &hex!("540009f321f41d00202e473b");
		const PLAINTEXT: &[u8] = &hex!("e56cdd522d526d8d0cd18131a19ee4fd");
		const AAD: &[u8] = &hex!("a41162e1fe875a81fbb5667f73c5d4cbbb9c3956002f7867047edec15bdcac1206e519ee9c238c371a38a485c710da60");
		const CIPHERTEXT: &[u8] = &hex!("8b624b6f5483f42f36c85dc7cf3e9609");
		const TAG: &[u8] = &hex!("2651e978d9eaa6c5f4db52391ac9bc7c");

		let cipher = Aes256Gcm::new(KEY.into());
		let ct = aes_encrypt(KEY.try_into().unwrap(), PLAINTEXT, AAD, NONCE.try_into().unwrap());
		// verify ciphertext
		let ct_ct = &ct.ciphertext.as_slice()[..(ct.ciphertext.len() - 16)];
		assert_eq!(ct_ct, CIPHERTEXT);
		// verify tag
		let ct_tag = &ct.ciphertext.as_slice()[(ct.ciphertext.len() - 16)..];
		assert_eq!(ct_tag, TAG);
		// try decrypt and verify, we need to include `aad` into payload as it's non-null
		let payload = Payload { msg: &ct.ciphertext, aad: &ct.aad };
		let decrypted_plaintext = cipher.decrypt(NONCE.try_into().unwrap(), payload).unwrap();
		assert_eq!(&decrypted_plaintext, PLAINTEXT);
	}

	#[test]
	fn test_encrypt_decrypt_aes_gcm_hardcoded_no_aad_works() {
		// values are from TestVector is RSA crate:
		const KEY: &[u8] =
			&hex!("56690798978c154ff250ba78e463765f2f0ce69709a4551bd8cb3addeda087b6");
		const NONCE: &[u8] = &hex!("cf37c286c18ad4ea3d0ba6a0");
		const PLAINTEXT: &[u8] = &hex!("2d328124a8d58d56d0775eed93de1a88");
		const AAD: &[u8] = b"";
		const CIPHERTEXT: &[u8] = &hex!("3b0a0267f6ecde3a78b30903ebd4ca6e");
		const TAG: &[u8] = &hex!("1fd2006409fc636379f3d4067eca0988");

		let cipher = Aes256Gcm::new(KEY.into());
		let ct = aes_encrypt(KEY.try_into().unwrap(), PLAINTEXT, AAD, NONCE.try_into().unwrap());
		// verify ciphertext
		let ct_ct = &ct.ciphertext.as_slice()[..(ct.ciphertext.len() - 16)];
		assert_eq!(ct_ct, CIPHERTEXT);
		// verify tag
		let ct_tag = &ct.ciphertext.as_slice()[(ct.ciphertext.len() - 16)..];
		assert_eq!(ct_tag, TAG);
		// try decrypt and verify, we pass ciphertext directly as `aad` is null
		let decrypted_plaintext =
			cipher.decrypt(NONCE.try_into().unwrap(), ct.ciphertext.as_ref()).unwrap();
		assert_eq!(&decrypted_plaintext, PLAINTEXT);
	}
}
