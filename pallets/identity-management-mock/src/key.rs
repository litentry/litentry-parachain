// Copyright 2020-2022 Litentry Technologies GmbH.
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

#[cfg(not(feature = "std"))]
use sp_std::prelude::*;

pub use rsa::{
	pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey, EncodeRsaPrivateKey, EncodeRsaPublicKey},
	pkcs1v15::{SigningKey, VerifyingKey},
	Hash, PaddingScheme, PublicKey, PublicKeyParts, RsaPrivateKey, RsaPublicKey,
};

use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
pub use rsa::pkcs8::DecodePublicKey;

// The hardcoded exemplary shielding keys for mocking
// openssl always generates a mix of X509 pub key and pkcs1 private key
// see https://stackoverflow.com/questions/10783366/how-to-generate-pkcs1-rsa-keys-in-pem-format
//
// commands used to generate keys:
// 1. openssl genrsa -out pallets/identity-management-mock/src/rsa_key_examples/pkcs1/2048-priv.pem
// 2048 2. openssl rsa -in pallets/identity-management-mock/src/rsa_key_examples/pkcs1/2048-priv.pem
// -pubout -out \   	pallets/identity-management-mock/src/rsa_key_examples/pkcs1/2048-pub.pem
//
// we use 2048 bit keys as user shielding key and 3072 bit for TEE shielding key
const USER_SHIELDING_KEY_PRIV: &str = include_str!("rsa_key_examples/pkcs1/2048-priv.pem");
const USER_SHIELDING_KEY_PUB: &str = include_str!("rsa_key_examples/pkcs1/2048-pub.pem");
const TEE_SHIELDING_KEY_PRIV: &str = include_str!("rsa_key_examples/pkcs1/3072-priv.pem");
const TEE_SHIELDING_KEY_PUB: &str = include_str!("rsa_key_examples/pkcs1/3072-pub.pem");

// TODO: remove unwrap and expect(..)
pub fn get_mock_user_shielding_key() -> (RsaPublicKey, RsaPrivateKey) {
	(
		RsaPublicKey::from_public_key_pem(USER_SHIELDING_KEY_PUB).unwrap(),
		RsaPrivateKey::from_pkcs1_pem(USER_SHIELDING_KEY_PRIV).unwrap(),
	)
}

pub fn get_mock_tee_shielding_key() -> (RsaPublicKey, RsaPrivateKey) {
	(
		RsaPublicKey::from_public_key_pem(TEE_SHIELDING_KEY_PUB).unwrap(),
		RsaPrivateKey::from_pkcs1_pem(TEE_SHIELDING_KEY_PRIV).unwrap(),
	)
}

pub fn encrypt_with_public_key(k: &[u8], data: &[u8]) -> Vec<u8> {
	// use a derandomized seed
	// rand::thread_rng() would require std in crate `rand`
	let mut rng = ChaCha8Rng::from_seed([42; 32]);
	let public_key = RsaPublicKey::from_public_key_pem(sp_std::str::from_utf8(k).unwrap()).unwrap();
	public_key
		.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), data)
		.expect("failed to encrypt")
}

#[cfg(test)]
mod test {
	use super::*;
	use hex_literal::hex;
	use sha2::{Digest, Sha256};
	use signature::{RandomizedSigner, Verifier};

	#[test]
	fn test_read_rsa_key_pair_works() {
		let (public_key, private_key) = get_mock_user_shielding_key();
		// openssl rsa -pubin -inform PEM -text -modulus <
		// pallets/identity-management-mock/src/rsa_key_examples/pkcs1/2048-pub.pem
		assert_eq!(&public_key.n().to_bytes_be(), &hex!("CBD08EFBD65343F9DFD9E6A3DC5B0296BC07174691D8C8B5594DF2F82E9F3E435B093DFA1496B406AF04287D3F851D51050846DE657B4C3A12467FD78E4ABF1A87B984A346FA0DC28AC09B7CFBFAD8AC183B87138CD1594BCFF5ABF46226A708569A02EC66B249A03AF8486638F6A12E2208D97C206C0DB6D8376EFFF464BB5B49FF4C9D2CDF82DBA59F36DC77092275975A689C7B87239F19D47A748C6631F5DDDCEF0B82ED1B46EFC48B202D37F9213ACDA317D326F6E0A07F27723F768B070981309C14C616079693CE7A0AF0EB3AE8DEE0E0A03DBAC0B29A5594C2582A7974205D5D4F70E2EA65A99588188D5F06B07647414E5BEAB33FF52A957FF2227B"));
		assert_eq!(&public_key.e().to_bytes_be(), &hex!("010001"));

		// openssl asn1parse -in <path-to-file>
		assert_eq!(&private_key.n().to_bytes_be(), &hex!("CBD08EFBD65343F9DFD9E6A3DC5B0296BC07174691D8C8B5594DF2F82E9F3E435B093DFA1496B406AF04287D3F851D51050846DE657B4C3A12467FD78E4ABF1A87B984A346FA0DC28AC09B7CFBFAD8AC183B87138CD1594BCFF5ABF46226A708569A02EC66B249A03AF8486638F6A12E2208D97C206C0DB6D8376EFFF464BB5B49FF4C9D2CDF82DBA59F36DC77092275975A689C7B87239F19D47A748C6631F5DDDCEF0B82ED1B46EFC48B202D37F9213ACDA317D326F6E0A07F27723F768B070981309C14C616079693CE7A0AF0EB3AE8DEE0E0A03DBAC0B29A5594C2582A7974205D5D4F70E2EA65A99588188D5F06B07647414E5BEAB33FF52A957FF2227B"));
		assert_eq!(&private_key.e().to_bytes_be(), &hex!("010001"));
		assert_eq!(&private_key.d().to_bytes_be(), &hex!("026A999760C621F32F753CE7CA7005CAD5B5DBCFC960E1984CD3C0C2B282CED12B9E236EF89984CEE37A502494013704C3E3823B96C66C73EFCD882C7D1263CBA3BA4E59453927BA9BBC86DB677D64DE3D774F35AB20BC474AD2E5D402E9E46713E7C58B19F89928DE2A1D69A0D943B5F14F5B8CBE31A9C3F6324A0D9CCF28ED79BB5BE9C36AEA2422F8609C959A876E9E97C20CA8AE0822D71AFE93FAAC4184AFCB7FCEF55721557F46D013B8A5903217898A881E0ED00AA1A0551F22492C13A7B7B1ED724D19DDE6B99387AC8BFEA1D28E16968DE0C9C4C84AB5A0CF2CA2DB3BC07E64BEA0934B46D4D7EACE40D79013234D8E880B7EF339C3255937CFAC31"));
		assert_eq!(&private_key.primes()[0].to_bytes_be(), &hex!("E7FD29DD53BD18133D2B8FF4B52567C1F6AA63EA6C5375DB2A49C70B8D00BC5A589C300507770A666E691C3FF3038C6482E686B397393DD154DEF1E6FFF74E72918E1D8B520518812D3B3E67F900C1DDFB2AC51A76A0328FBC39E8E060186DCF588F9D732738C76DD901F61DECE973733245E401CC3C7E0E2F02C89C1CA1B8B3"));
		assert_eq!(&private_key.primes()[1].to_bytes_be(), &hex!("E0E8E221EC053D4C7D5E5FC07AAFA930D1A791E6DD3957280406071AD47CC9F8A871F7DF8E36416EA2DC78412EE8B2B2EC148E2C0790C7690575CA4FCF0BCD46738D0E13A6917DB063966B6AAC3D8752A739D3BCC8A33FD94D86571E359F9A713CEE99F26316DB49C4F478FB695E21F76BF18F1F59EB0A396C6AEEDE518F0319"));
	}

	#[test]
	fn test_encrypt_decrypt_rsa_key_pair_works() {
		let (public_key, private_key) = get_mock_tee_shielding_key();
		assert_eq!(private_key.to_public_key(), public_key);

		// encrypt with public key
		let mut rng = ChaCha8Rng::from_seed([42; 32]);
		let data = b"hello world";
		let enc_data = public_key
			.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), &data[..])
			.expect("failed to encrypt");

		// decrypt with private key
		let dec_data = private_key
			.decrypt(PaddingScheme::new_pkcs1v15_encrypt(), &enc_data)
			.expect("failed to decrypt");
		assert_eq!(&data[..], &dec_data[..]);
	}

	#[test]
	fn test_sign_verify_rsa_key_pair_works() {
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
}
