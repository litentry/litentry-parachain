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

#![cfg(test)]
pub use crate::{
	self as pallet_identity_management_mock,
	key::{aes_encrypt_default, tee_encrypt},
	ChallengeCode,
};

use aes_gcm::{
	aead::{KeyInit, OsRng},
	Aes256Gcm,
};
use codec::Encode;
use frame_support::{
	assert_ok, ord_parameter_types, parameter_types,
	traits::{ConstU128, ConstU16, ConstU32, ConstU64, Everything},
};
use frame_system as system;
pub use mock_tee_primitives::{
	EthereumSignature, EvmNetwork, Identity, IdentityMultiSignature, IdentityString,
	SubstrateNetwork, TwitterValidationData, UserShieldingKeyType, ValidationData, Web2Network,
	Web2ValidationData, Web3CommonValidationData, Web3ValidationData,
};
pub use parity_crypto::publickey::{sign, Generator, KeyPair as EvmPair, Message, Random};
use sp_core::sr25519::Pair as SubstratePair; // TODO: maybe use more generic struct
use sp_core::{Pair, H256};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
use system::{EnsureRoot, EnsureSignedBy};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub type Balance = u128;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
		IdentityManagementMock: pallet_identity_management_mock::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<31>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<10000>;
	type WeightInfo = ();
}

impl pallet_balances::Config for Test {
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = Balance; // the type that is relevant to us
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
	type WeightInfo = ();
}

ord_parameter_types! {
	pub const One: u64 = 1;
}

impl pallet_identity_management_mock::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MaxVerificationDelay = ConstU64<10>;
	type TEECallOrigin = EnsureSignedBy<One, u64>;
	type DelegateeAdminOrigin = EnsureRoot<Self::AccountId>;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();

	pallet_balances::GenesisConfig::<Test> { balances: vec![(1u64, 100)] }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		System::set_block_number(1);
	});
	ext
}

pub fn create_mock_twitter_identity(twitter_handle: &[u8]) -> Identity {
	let address =
		IdentityString::try_from(twitter_handle.to_vec()).expect("convert to BoundedVec failed");
	Identity::Web2 { network: Web2Network::Twitter, address }
}

pub fn create_mock_polkadot_identity(address: [u8; 32]) -> Identity {
	Identity::Substrate { network: SubstrateNetwork::Polkadot, address: address.into() }
}

pub fn create_mock_eth_identity(address: [u8; 20]) -> Identity {
	Identity::Evm { network: EvmNetwork::Ethereum, address: address.into() }
}

pub fn create_mock_twitter_validation_data() -> ValidationData {
	ValidationData::Web2(Web2ValidationData::Twitter(TwitterValidationData {
		tweet_id: b"0903".to_vec().try_into().expect("convert to BoundedVec failed"),
	}))
}

pub fn create_mock_polkadot_validation_data(
	who: <Test as frame_system::Config>::AccountId,
	p: SubstratePair,
	code: ChallengeCode,
	is_wrapped_signature: bool,
) -> ValidationData {
	let identity = create_mock_polkadot_identity(p.public().0);
	let msg = IdentityManagementMock::get_expected_raw_message(&who, &identity, &code)
		.expect("cannot calculate web3 message");
	let mut sig = p.sign(&msg);

	if !is_wrapped_signature {
		sig = p.sign(&IdentityManagementMock::get_expected_wrapped_message(msg.clone()));
	}

	let common_validation_data = Web3CommonValidationData {
		message: msg.try_into().unwrap(),
		signature: IdentityMultiSignature::Sr25519(sig),
	};
	ValidationData::Web3(Web3ValidationData::Substrate(common_validation_data))
}

pub fn create_mock_eth_validation_data(
	who: <Test as frame_system::Config>::AccountId,
	p: EvmPair,
	code: ChallengeCode,
) -> ValidationData {
	let identity = create_mock_eth_identity(p.address().0);
	let msg = IdentityManagementMock::get_expected_raw_message(&who, &identity, &code)
		.expect("cannot calculate web3 message");
	let digest = IdentityManagementMock::compute_evm_msg_digest(&msg);
	let sig = sign(p.secret(), &Message::from(digest)).unwrap();

	let common_validation_data = Web3CommonValidationData {
		message: msg.try_into().unwrap(),
		signature: IdentityMultiSignature::Ethereum(EthereumSignature(sig.into_electrum())),
	};
	ValidationData::Web3(Web3ValidationData::Evm(common_validation_data))
}

// generate a random user shielding key, encrypt it and store it for account `who`
pub fn setup_user_shieding_key(
	who: <Test as frame_system::Config>::AccountId,
) -> UserShieldingKeyType {
	// generate user shielding key
	let shielding_key = Aes256Gcm::generate_key(&mut OsRng);
	let encrpted_shielding_key = tee_encrypt(&shielding_key);
	// whitelist caller
	assert_ok!(IdentityManagementMock::set_user_shielding_key(
		RuntimeOrigin::signed(who),
		H256::random(),
		encrpted_shielding_key.to_vec()
	));
	System::assert_has_event(RuntimeEvent::IdentityManagementMock(
		crate::Event::UserShieldingKeySetPlain { account: who },
	));
	// enrypt the result
	let key = IdentityManagementMock::user_shielding_keys(&who).unwrap();
	System::assert_has_event(RuntimeEvent::IdentityManagementMock(
		crate::Event::UserShieldingKeySet { account: who },
	));
	key
}

pub fn setup_create_identity(
	who: <Test as frame_system::Config>::AccountId,
	identity: Identity,
	bn: <Test as frame_system::Config>::BlockNumber,
) {
	let key = setup_user_shieding_key(who);
	let encrypted_identity = tee_encrypt(identity.encode().as_slice());
	let code = IdentityManagementMock::get_mock_challenge_code(
		bn,
		IdentityManagementMock::challenge_codes(&who, &identity),
	);

	System::assert_has_event(RuntimeEvent::IdentityManagementMock(
		crate::Event::UserShieldingKeySet { account: who },
	));

	assert_ok!(IdentityManagementMock::create_identity(
		RuntimeOrigin::signed(who),
		H256::random(),
		who,
		encrypted_identity.to_vec(),
		None
	));
	System::assert_has_event(RuntimeEvent::IdentityManagementMock(
		crate::Event::IdentityCreatedPlain {
			account: who,
			identity: identity.clone(),
			code,
			id_graph: IdentityManagementMock::get_id_graph(&who),
		},
	));
	// encrypt the result
	let aes_encrypted_identity = aes_encrypt_default(&key, identity.encode().as_slice());
	let aes_encrypted_code = aes_encrypt_default(&key, code.as_ref());
	let aes_encrypted_id_graph =
		aes_encrypt_default(&key, IdentityManagementMock::get_id_graph(&who).encode().as_slice());

	System::assert_has_event(RuntimeEvent::IdentityManagementMock(crate::Event::IdentityCreated {
		account: who,
		identity: aes_encrypted_identity,
		code: aes_encrypted_code,
		id_graph: aes_encrypted_id_graph,
	}));
}

pub fn setup_verify_twitter_identity(
	who: <Test as frame_system::Config>::AccountId,
	identity: Identity,
	bn: <Test as frame_system::Config>::BlockNumber,
) {
	setup_create_identity(who, identity.clone(), bn);
	let encrypted_identity = tee_encrypt(identity.encode().as_slice());
	let validation_data = if let Identity::Web2 { network, .. } = &identity {
		match network {
			Web2Network::Twitter => create_mock_twitter_validation_data(),
			_ => panic!("unexpected network, expect twitter network"),
		}
	} else {
		panic!("invalid identity type")
	};
	assert_ok!(IdentityManagementMock::verify_identity(
		RuntimeOrigin::signed(who),
		H256::random(),
		encrypted_identity,
		tee_encrypt(validation_data.encode().as_slice()),
	));
	System::assert_has_event(RuntimeEvent::IdentityManagementMock(
		crate::Event::IdentityVerifiedPlain {
			account: who,
			identity,
			id_graph: IdentityManagementMock::get_id_graph(&who),
		},
	));
}

pub fn setup_verify_polkadot_identity(
	who: <Test as frame_system::Config>::AccountId,
	p: SubstratePair,
	bn: <Test as frame_system::Config>::BlockNumber,
	is_wrapped_signature: bool,
) {
	let identity = create_mock_polkadot_identity(p.public().0);
	setup_create_identity(who, identity.clone(), bn);
	let encrypted_identity = tee_encrypt(identity.encode().as_slice());
	let code = IdentityManagementMock::challenge_codes(&who, &identity).unwrap();
	let validation_data = if let Identity::Substrate { .. } = &identity {
		create_mock_polkadot_validation_data(who, p, code, is_wrapped_signature)
	} else {
		panic!("unxpected network")
	};
	assert_ok!(IdentityManagementMock::verify_identity(
		RuntimeOrigin::signed(who),
		H256::random(),
		encrypted_identity,
		tee_encrypt(validation_data.encode().as_slice()),
	));
	System::assert_has_event(RuntimeEvent::IdentityManagementMock(
		crate::Event::IdentityVerifiedPlain {
			account: who,
			identity,
			id_graph: IdentityManagementMock::get_id_graph(&who),
		},
	));
}

pub fn setup_verify_eth_identity(
	who: <Test as frame_system::Config>::AccountId,
	p: EvmPair,
	bn: <Test as frame_system::Config>::BlockNumber,
) {
	let identity = create_mock_eth_identity(p.address().0);
	setup_create_identity(who, identity.clone(), bn);
	let encrypted_identity = tee_encrypt(identity.encode().as_slice());
	let code = IdentityManagementMock::challenge_codes(&who, &identity).unwrap();
	let validation_data = if let Identity::Evm { .. } = identity {
		create_mock_eth_validation_data(who, p, code)
	} else {
		panic!("unexpected network, expect Evm")
	};
	assert_ok!(IdentityManagementMock::verify_identity(
		RuntimeOrigin::signed(who),
		H256::random(),
		encrypted_identity,
		tee_encrypt(validation_data.encode().as_slice()),
	));
	System::assert_has_event(RuntimeEvent::IdentityManagementMock(
		crate::Event::IdentityVerifiedPlain {
			account: who,
			identity,
			id_graph: IdentityManagementMock::get_id_graph(&who),
		},
	));
}
