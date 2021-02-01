use crate::{mock::*, Event};

use codec::Encode;
use parity_crypto::Keccak256;
use frame_support::{assert_ok, assert_noop};
use sp_runtime::AccountId32;

use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use bitcoin::util::key;
use bitcoin::secp256k1::{Secp256k1, Message as BTCMessage};
use bitcoin::secp256k1::rand::thread_rng;

#[test]
fn test_invalid_expiring_block_number_btc() {
	new_test_ext().execute_with(|| {
		// Generate random key pair
		let s = Secp256k1::new();
		let pair = s.generate_keypair(&mut thread_rng());
		let public_key = key::PublicKey {
			compressed: true,
			key: pair.1,
		};

		// Generate pay-to-pubkey-hash address
		let address = Address::p2pkh(&public_key, Network::Bitcoin);

		let account: AccountId32 = AccountId32::from([255u8; 32]);
		let block_number: u32 = crate::EXPIRING_BLOCK_NUMBER_MAX + 1;

		let mut bytes = b"Link Litentry: ".encode();
		let mut account_vec = account.encode();
		let mut expiring_block_number_vec = block_number.encode();

		bytes.append(&mut account_vec);
		bytes.append(&mut expiring_block_number_vec);

		let message = BTCMessage::from_slice(&bytes.keccak256()).unwrap();

		let (v, rs) = s.sign_recoverable(&message, &pair.0).serialize_compact();

		let mut r = [0u8; 32];
		let mut s = [0u8; 32];

		r[..32].copy_from_slice(&rs[..32]);
		s[..32].copy_from_slice(&rs[32..64]);

		assert_noop!(
			AccountLinker::link_btc(
				Origin::signed(account.clone()),
				account.clone(),
				0,
				address.clone().to_string().as_bytes().to_vec(),
				block_number,
				r,
				s,
				v.to_i32() as u8),
			AccountLinkerError::InvalidExpiringBlockNumber
		);

	});
}

#[test]
fn test_btc_link_p2pkh() {
	new_test_ext().execute_with(|| {

        run_to_block(1);

		// Generate random key pair
		let s = Secp256k1::new();
		let pair = s.generate_keypair(&mut thread_rng());
		let public_key = key::PublicKey {
			compressed: true,
			key: pair.1,
		};

		// Generate pay-to-pubkey-hash address
		let address = Address::p2pkh(&public_key, Network::Bitcoin);

		let account: AccountId32 = AccountId32::from([255u8; 32]);
		let block_number: u32 = 99999;

		let mut bytes = b"Link Litentry: ".encode();
		let mut account_vec = account.encode();
		let mut expiring_block_number_vec = block_number.encode();

		bytes.append(&mut account_vec);
		bytes.append(&mut expiring_block_number_vec);

		let message = BTCMessage::from_slice(&bytes.keccak256()).unwrap();

		let (v, rs) = s.sign_recoverable(&message, &pair.0).serialize_compact();

		let mut r = [0u8; 32];
		let mut s = [0u8; 32];

		r[..32].copy_from_slice(&rs[..32]);
		s[..32].copy_from_slice(&rs[32..64]);

        let addr_expected = address.clone().to_string().as_bytes().to_vec();

		assert_ok!(AccountLinker::link_btc(
			Origin::signed(account.clone()),
			account.clone(),
			0,
			addr_expected.clone(),
			block_number,
			r,
			s,
			v.to_i32() as u8
		));

		let addr_stored = String::from_utf8(AccountLinker::btc_addresses(&account)[0].clone()).unwrap();

        assert_eq!(addr_stored, address.to_string());
        
        assert_eq!(
            System::events()[0].event,  
            TestEvent::account_linker( Event::<Test>::BtcAddressLinked(account, addr_expected) )
        );

	});
}

#[test]
fn test_btc_link_p2wpkh() {
	new_test_ext().execute_with(|| {

        run_to_block(1);

		// Generate random key pair
		let s = Secp256k1::new();
		let pair = s.generate_keypair(&mut thread_rng());
		let public_key = key::PublicKey {
			compressed: true,
			key: pair.1,
		};

		// Generate pay-to-pubkey-hash address
		let address = Address::p2wpkh(&public_key, Network::Bitcoin).unwrap();

		println!("{}", address);
		let account: AccountId32 = AccountId32::from([255u8; 32]);
		let block_number: u32 = 99999;

		let mut bytes = b"Link Litentry: ".encode();
		let mut account_vec = account.encode();
		let mut expiring_block_number_vec = block_number.encode();

		bytes.append(&mut account_vec);
		bytes.append(&mut expiring_block_number_vec);

		let message = BTCMessage::from_slice(&bytes.keccak256()).unwrap();

		let (v, rs) = s.sign_recoverable(&message, &pair.0).serialize_compact();

		let mut r = [0u8; 32];
		let mut s = [0u8; 32];

		r[..32].copy_from_slice(&rs[..32]);
		s[..32].copy_from_slice(&rs[32..64]);

        let addr_expected = address.clone().to_string().as_bytes().to_vec();

		assert_ok!(AccountLinker::link_btc(
			Origin::signed(account.clone()),
			account.clone(),
			0,
			addr_expected.clone(),
			block_number,
			r,
			s,
			v.to_i32() as u8
		));

		let addr_stored = String::from_utf8(AccountLinker::btc_addresses(&account)[0].clone()).unwrap();

        assert_eq!(addr_stored, address.to_string());
        
        assert_eq!(
            System::events()[0].event,  
            TestEvent::account_linker( Event::<Test>::BtcAddressLinked(account, addr_expected) )
        );

	});
}