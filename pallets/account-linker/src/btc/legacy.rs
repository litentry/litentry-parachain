use sha2::{Digest, Sha256};
use ripemd160::Ripemd160;

pub fn btc_addr_from_pk(pk: &[u8]) -> [u8; 25] {
    let mut result = [0u8; 25];

    // Now only support P2PKH (Mainnet) prefix = 0
    result[0] = 0;
    result[1..21].copy_from_slice(&hash160(pk));
    let cs = checksum(&result[0..21]);
    result[21..25].copy_from_slice(&cs);
    result
}

pub fn hash160(bytes: &[u8]) -> [u8; 20] {
    let mut hasher_sha256 = Sha256::new();
    hasher_sha256.update(bytes);
    let digest = hasher_sha256.finalize();

    let mut hasher_ripemd = Ripemd160::new();
    hasher_ripemd.update(digest);

    let mut ret = [0; 20];
    ret.copy_from_slice(&hasher_ripemd.finalize()[..]);
    ret
}

fn checksum(input: &[u8]) -> [u8; 4] {
	let mut result = [0u8; 4];
	result.copy_from_slice(&dsha256(input)[0..4]);
	result
}

/// Computes Bitcoin's double SHA256 hash over a LE byte encoded input
///
/// # Arguments
/// * data: LE bytes encoded input
///
/// # Returns
/// * The double SHA256 hash encoded as LE bytes from data
fn dsha256(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();

    let mut second_hasher = Sha256::new();
    second_hasher.update(digest);

    let mut ret = [0; 32];
    ret.copy_from_slice(&second_hasher.finalize()[..]);
    ret
}

// test data can be obtained from here http://gobittest.appspot.com/Address
#[cfg(test)]
mod tests {
	use super::*;
	use hex::decode;

	#[test]
	fn correct_dhash160() {

		let pk = decode("0450863AD64A87AE8A2FE83C1AF1A8403CB53F53E486D8511DAD8A04887E5B23522CD470243453A299FA9E77237716103ABC11A1DF38855ED6F2EE187E9C582BA6").unwrap();

        let hash = hash160(&pk);

        let result = decode("010966776006953D5567439E5E39F86A0D273BEE").unwrap();
		let mut hash_expected = [0u8; 20];
		hash_expected[0..20].copy_from_slice(&result[0..20]);

		assert_eq!(hash, hash_expected);
    }

    #[test]
    fn correct_btc_addr_from_pk() {
        let pk = decode("0450863AD64A87AE8A2FE83C1AF1A8403CB53F53E486D8511DAD8A04887E5B23522CD470243453A299FA9E77237716103ABC11A1DF38855ED6F2EE187E9C582BA6").unwrap();
        let mut pk_input = [0u8; 65];
        pk_input[0..65].copy_from_slice(&pk[0..65]);

        let addr = btc_addr_from_pk(&pk_input);

        let addr_expected_hex = decode("00010966776006953D5567439E5E39F86A0D273BEED61967F6").unwrap();
        let mut addr_expected = [0u8; 25];
        addr_expected[0..25].copy_from_slice(&addr_expected_hex[0..25]);
        assert_eq!(addr, addr_expected);
    }

}