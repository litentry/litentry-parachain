use crate::mock::*;

use codec::Encode;
use frame_support::{assert_noop, assert_ok};
use parity_crypto::{
	publickey::{sign, Generator, KeyPair, Message, Random},
	Keccak256,
};
use sp_runtime::AccountId32;

use sp_core::{crypto::Pair, ecdsa, ed25519, sr25519};

fn generate_eth_raw_message(account: &AccountId32, block_number: u32) -> Message {
	let mut bytes = b"\x19Ethereum Signed Message:\n51Link Litentry: ".encode();
	let mut account_vec = account.encode();
	let mut expiring_block_number_vec = block_number.encode();

	bytes.append(&mut account_vec);
	bytes.append(&mut expiring_block_number_vec);

	Message::from(bytes.keccak256())
}

fn generate_sub_raw_message(
	account: &AccountId32,
	network_type: crate::NetworkType,
	expiring_block_number: u32,
) -> Vec<u8> {
	let mut bytes = b"Link Litentry: ".encode();
	let mut network_type_vec = network_type.encode();
	let mut account_vec = account.encode();
	let mut expiring_block_number_vec = expiring_block_number.encode();

	bytes.append(&mut network_type_vec);
	bytes.append(&mut account_vec);
	bytes.append(&mut expiring_block_number_vec);
	bytes
}

fn generate_sig(key_pair: &KeyPair, msg: &Message) -> [u8; 65] {
	sign(key_pair.secret(), &msg).unwrap().into_electrum()
}

fn generate_sr25519_sig(msg: [u8; 32]) -> sr25519::Signature {
	// serect seed for Alice 0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a
	let alice_seed_str = "e5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a";
	let decoded_seed = hex::decode(alice_seed_str).unwrap();
	let mut alice_seed = [0_u8; 32];
	alice_seed[0..32].copy_from_slice(&decoded_seed[0..32]);
	let pair = sr25519::Pair::from_seed(&alice_seed);
	pair.sign(&msg)
}

fn generate_ed25519_sig(msg: [u8; 32]) -> ed25519::Signature {
	//  bash-5.0$ target/release/subkey inspect //Alice --scheme Ed25519
	// Secret Key URI `//Alice` is account:
	//   Secret seed:       0xabf8e5bdbe30c65656c0a3cbd181ff8a56294a69dfedd27982aace4a76909115
	//   Public key (hex):  0x88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee
	//   Account ID:        0x88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee
	//   Public key (SS58): 5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu
	//   SS58 Address:      5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu
	let alice_seed_str = "abf8e5bdbe30c65656c0a3cbd181ff8a56294a69dfedd27982aace4a76909115";
	let decoded_seed = hex::decode(alice_seed_str).unwrap();
	let mut alice_seed = [0_u8; 32];
	alice_seed[0..32].copy_from_slice(&decoded_seed[0..32]);
	let pair = ed25519::Pair::from_seed(&alice_seed);
	pair.sign(&msg)
}

fn generate_ecdsa_sig(msg: &[u8]) -> ecdsa::Signature {
	// bash-5.0$ target/release/subkey inspect //Alice --scheme Ecdsa
	// Secret Key URI `//Alice` is account:
	//   Secret seed:       0xcb6df9de1efca7a3998a8ead4e02159d5fa99c3e0d4fd6432667390bb4726854
	//   Public key (hex):  0x020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a1
	//   Account ID:        0x01e552298e47454041ea31273b4b630c64c104e4514aa3643490b8aaca9cf8ed
	//   Public key (SS58): KW39r9CJjAVzmkf9zQ4YDb2hqfAVGdRqn53eRqyruqpxAP5YL
	//   SS58 Address:      5C7C2Z5sWbytvHpuLTvzKunnnRwQxft1jiqrLD5rhucQ5S9X
	let alice_seed_str = "cb6df9de1efca7a3998a8ead4e02159d5fa99c3e0d4fd6432667390bb4726854";
	let decoded_seed = hex::decode(alice_seed_str).unwrap();
	let mut alice_seed = [0_u8; 32];
	alice_seed[0..32].copy_from_slice(&decoded_seed[0..32]);
	let pair = ecdsa::Pair::from_seed(&alice_seed);
	pair.sign(&msg)
}

#[test]
fn test_expired_block_number_eth() {
	new_test_ext().execute_with(|| {
		let account: AccountId32 = AccountId32::from([0u8; 32]);
		let block_number: u32 = 100;
		let layer_one_blocknumber: u32 = 1000;

		let mut gen = Random {};
		let key_pair = gen.generate();

		let msg = generate_eth_raw_message(&account, block_number);
		let sig = generate_sig(&key_pair, &msg);

		assert_noop!(
			SgxAccountLinker::do_link_eth(
				account.clone(),
				0,
				key_pair.address().to_fixed_bytes(),
				block_number,
				layer_one_blocknumber,
				sig
			),
			SgxAccountLinkerError::LinkRequestExpired
		);
	});
}

#[test]
fn test_invalid_expiring_block_number_eth() {
	new_test_ext().execute_with(|| {
		let account: AccountId32 = AccountId32::from([0u8; 32]);
		let block_number: u32 = crate::EXPIRING_BLOCK_NUMBER_MAX + 10;
		let layer_one_blocknumber: u32 = 1;

		let mut gen = Random {};
		let key_pair = gen.generate();

		let msg = generate_eth_raw_message(&account, block_number);
		let sig = generate_sig(&key_pair, &msg);

		assert_noop!(
			SgxAccountLinker::do_link_eth(
				account.clone(),
				0,
				key_pair.address().to_fixed_bytes(),
				block_number,
				layer_one_blocknumber,
				sig
			),
			SgxAccountLinkerError::InvalidExpiringBlockNumber
		);
	});
}

#[test]
fn test_unexpected_address_eth() {
	new_test_ext().execute_with(|| {
		let account: AccountId32 = AccountId32::from([72u8; 32]);
		let block_number: u32 = 99999;
		let layer_one_blocknumber: u32 = 10;

		let mut gen = Random {};
		let key_pair = gen.generate();

		let msg = generate_eth_raw_message(&account, block_number);
		let sig = generate_sig(&key_pair, &msg);

		assert_noop!(
			SgxAccountLinker::do_link_eth(
				account.clone(),
				0,
				gen.generate().address().to_fixed_bytes(),
				block_number,
				layer_one_blocknumber,
				sig
			),
			SgxAccountLinkerError::UnexpectedAddress
		);
	});
}

#[test]
fn test_insert_eth_address() {
	new_test_ext().execute_with(|| {
		run_to_block(1);

		let account: AccountId32 = AccountId32::from([5u8; 32]);
		let block_number: u32 = 99999;
		let layer_one_blocknumber: u32 = 10;

		let mut gen = Random {};
		let mut expected_vec = Vec::new();

		for i in 0..(MAX_ETH_LINKS) {
			let key_pair = gen.generate();

			let msg = generate_eth_raw_message(&account, block_number + i as u32);
			let sig = generate_sig(&key_pair, &msg);

			assert_ok!(SgxAccountLinker::do_link_eth(
				account.clone(),
				i as u32,
				key_pair.address().to_fixed_bytes(),
				block_number + i as u32,
				layer_one_blocknumber,
				sig
			));

			assert_eq!(SgxAccountLinker::eth_addresses(&account).len(), i + 1);
			expected_vec.push(key_pair.address().to_fixed_bytes());
			assert_eq!(
				events(),
				[Event::SgxAccountLinker(crate::Event::EthAddressLinked(
					account.clone(),
					key_pair.address().to_fixed_bytes().to_vec()
				)),]
			);
		}
		assert_eq!(SgxAccountLinker::eth_addresses(&account), expected_vec);
	});
}

#[test]
fn test_update_eth_address() {
	new_test_ext().execute_with(|| {
		let account: AccountId32 = AccountId32::from([40u8; 32]);
		let block_number: u32 = 99999;
		let layer_one_blocknumber: u32 = 10;

		let mut gen = Random {};
		for i in 0..(MAX_ETH_LINKS) {
			let key_pair = gen.generate();
			let msg = generate_eth_raw_message(&account, block_number + i as u32);
			let sig = generate_sig(&key_pair, &msg);

			assert_ok!(SgxAccountLinker::do_link_eth(
				account.clone(),
				i as u32,
				key_pair.address().to_fixed_bytes(),
				block_number + i as u32,
				layer_one_blocknumber,
				sig
			));
		}

		let index: u32 = 2 as u32;
		// Retrieve previous addr
		let addr_before_update = SgxAccountLinker::eth_addresses(&account)[index as usize];
		// Update addr at slot `index`
		let key_pair = gen.generate();
		let block_number = block_number + 9 as u32;
		let msg = generate_eth_raw_message(&account, block_number);
		let sig = generate_sig(&key_pair, &msg);

		assert_ok!(SgxAccountLinker::do_link_eth(
			account.clone(),
			index,
			key_pair.address().to_fixed_bytes(),
			block_number,
			layer_one_blocknumber,
			sig
		));

		let updated_addr = SgxAccountLinker::eth_addresses(&account)[index as usize];
		assert_ne!(updated_addr, addr_before_update);
		assert_eq!(updated_addr, key_pair.address().to_fixed_bytes());
	});
}

#[test]
fn test_eth_address_pool_overflow() {
	new_test_ext().execute_with(|| {
		let account: AccountId32 = AccountId32::from([113u8; 32]);
		let block_number: u32 = 99999;
		let layer_one_blocknumber: u32 = 10;

		let mut gen = Random {};
		let mut expected_vec = Vec::new();

		for index in 0..(MAX_ETH_LINKS * 2) {
			let key_pair = gen.generate();

			let msg = generate_eth_raw_message(&account, block_number);
			let sig = generate_sig(&key_pair, &msg);

			assert_ok!(SgxAccountLinker::do_link_eth(
				account.clone(),
				index as u32,
				key_pair.address().to_fixed_bytes(),
				block_number,
				layer_one_blocknumber,
				sig
			));

			if index < MAX_ETH_LINKS {
				expected_vec.push(key_pair.address().to_fixed_bytes());
			} else {
				expected_vec[MAX_ETH_LINKS - 1] = key_pair.address().to_fixed_bytes();
			}
		}
		assert_eq!(SgxAccountLinker::eth_addresses(&account).len(), MAX_ETH_LINKS);
		assert_eq!(SgxAccountLinker::eth_addresses(&account), expected_vec);
	});
}

#[test]
fn test_insert_fix_data() {
	new_test_ext().execute_with(|| {

        run_to_block(1);

		// account id of Alice 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
		let account: AccountId32 = AccountId32::from([
			0xd4, 0x35, 0x93, 0xc7, 0x15, 0xfd, 0xd3, 0x1c, 0x61, 0x14, 0x1a, 0xbd, 0x04, 0xa9,
			0x9f, 0xd6, 0x82, 0x2c, 0x85, 0x58, 0x85, 0x4c, 0xcd, 0xe3, 0x9a, 0x56, 0x84, 0xe7,
			0xa5, 0x6d, 0xa2, 0x7d,
		]);

		let block_number: u32 = 10000;
		let layer_one_blocknumber: u32 = 10;

		let index = 0;
		let eth_address_str = "4d88dc5d528a33e4b8be579e9476715f60060582";
		let decoded_address = hex::decode(eth_address_str).unwrap();
		let mut eth_address = [0_u8; 20];
		eth_address[0..20].copy_from_slice(&decoded_address[0..20]);
		let signature_str = "318400f0f9bd15f0d8842870b510e996dffc944b77111ded03a4255c66e82d427132e765d5e6bb21ba046dbb98e28bb28cb2bebe0c8aced2c547aca60a5548921c";
		let decoded_signature = hex::decode(signature_str).unwrap();
		let mut signature = [0_u8; 65];
		signature[0..65].copy_from_slice(&decoded_signature[0..65]);

		assert_ok!(SgxAccountLinker::do_link_eth(
			account.clone(),
			index,
			eth_address,
			block_number,
			layer_one_blocknumber,
			signature
		));
	});
}

#[test]
fn test_link_sub_sr25519_address() {
	new_test_ext().execute_with(|| {
		run_to_block(1);

		// account id of Alice 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
		let account: AccountId32 = AccountId32::from([
			0xd4, 0x35, 0x93, 0xc7, 0x15, 0xfd, 0xd3, 0x1c, 0x61, 0x14, 0x1a, 0xbd, 0x04, 0xa9,
			0x9f, 0xd6, 0x82, 0x2c, 0x85, 0x58, 0x85, 0x4c, 0xcd, 0xe3, 0x9a, 0x56, 0x84, 0xe7,
			0xa5, 0x6d, 0xa2, 0x7d,
		]);

		let block_number: u32 = 10000;
		let layer_one_blocknumber: u32 = 10;

		let index = 0_u32;
		let network_type = crate::NetworkType::Kusama;

		let bytes = generate_sub_raw_message(&account.clone(), network_type, block_number);
		let msg = sp_io::hashing::keccak_256(&bytes);
		let signature_raw = generate_sr25519_sig(msg);

		//signature is 4cdfda29585e33fe1969ee67bab9d29e440973b8cfde47346043c91d6d763c317659686089c2642e9a64fa28a00b1767fd134484fc0146de6d9eefdf366d2c81
		let signature = crate::MultiSignature::Sr25519Signature(signature_raw.0);

		assert_ok!(SgxAccountLinker::do_link_sub(
			account.clone(),
			index,
			network_type,
			account.clone(),
			block_number,
			layer_one_blocknumber,
			signature
		));
	});
}

#[test]
fn test_link_sub_ed25519_address() {
	new_test_ext().execute_with(|| {
		run_to_block(1);

		// account id of Alice 0x88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee
		let account: AccountId32 = AccountId32::from([
			136, 220, 52, 23, 213, 5, 142, 196, 180, 80, 62, 12, 18, 234, 26, 10, 137, 190, 32, 15,
			233, 137, 34, 66, 61, 67, 52, 1, 79, 166, 176, 238,
		]);

		let block_number: u32 = 10000;
		let layer_one_blocknumber: u32 = 10;

		let index = 0_u32;
		let network_type = crate::NetworkType::Kusama;

		let bytes = generate_sub_raw_message(&account.clone(), network_type, block_number);
		let msg = sp_io::hashing::keccak_256(&bytes);

		// signature is 0x08b1284cdaf008c80740d52be923cf24d45f8ba6e009bfd61a0a364c23df98c268dff3a8baabb47c8011ba39f76729aeab3c0281bfa45ef9389162fd78c18b08
		let signature_raw = generate_ed25519_sig(msg);

		let signature = crate::MultiSignature::Ed25519Signature(signature_raw.0);

		assert_ok!(SgxAccountLinker::do_link_sub(
			account.clone(),
			index,
			network_type,
			account.clone(),
			block_number,
			layer_one_blocknumber,
			signature
		));
	});
}

#[test]
fn test_link_sub_ecdsa_address() {
	new_test_ext().execute_with(|| {
		run_to_block(1);

		// account id of Alice 0x01e552298e47454041ea31273b4b630c64c104e4514aa3643490b8aaca9cf8ed
		let account: AccountId32 = AccountId32::from([
			1, 229, 82, 41, 142, 71, 69, 64, 65, 234, 49, 39, 59, 75, 99, 12, 100, 193, 4, 228, 81,
			74, 163, 100, 52, 144, 184, 170, 202, 156, 248, 237,
		]);

		let block_number: u32 = 10000;
		let layer_one_blocknumber: u32 = 10;

		let index = 0_u32;
		let network_type = crate::NetworkType::KusamaParachain(1);

		let bytes = generate_sub_raw_message(&account.clone(), network_type, block_number);

		// signature is 0xbf9484a706e5fadbd9ae8fd2e61f58c8aea387816903ef5b549af19cbf8c4fd831782058ea8f7acddc0e024f4d1ca155052fb04ad4115eeed75fefd6a7a6764301
		let signature_raw = generate_ecdsa_sig(&bytes[..]);

		let signature = crate::MultiSignature::EcdsaSignature(signature_raw.0);

		assert_ok!(SgxAccountLinker::do_link_sub(
			account.clone(),
			index,
			network_type,
			account.clone(),
			block_number,
			layer_one_blocknumber,
			signature
		));
	});
}
