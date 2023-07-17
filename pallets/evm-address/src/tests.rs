#![cfg(test)]
use super::mock::*;
use codec::Encode;
use frame_support::assert_ok;
use hex_literal::hex;
use pallet_evm::EnsureAddressOrigin;
use sp_core::H160;
#[test]
fn address_mapping() {
	new_test_ext().execute_with(|| {
		pub const ALICE: AccountId = sp_runtime::AccountId32::new(hex![
			"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
		]);
		// Check address mapping logic state
		assert_eq!(
			H160::from_slice(&ALICE.encode()[0..20]),
			H160::from_slice(&hex!["d43593c715Fdd31c61141ABd04a99FD6822c8558"])
		);
		assert_ok!(EnsureAddressEqualAndStore::<Test>::try_address_origin(
			&H160::from_slice(&hex!["d43593c715Fdd31c61141ABd04a99FD6822c8558"]),
			RuntimeOrigin::signed(ALICE),
		));
	})
}
