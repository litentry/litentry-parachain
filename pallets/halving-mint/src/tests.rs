use crate::{mock::*, Error, Event, Instance1, State};
use frame_support::{assert_noop, assert_ok};

#[test]
fn set_mint_state_check_works() {
	new_test_ext().execute_with(|| {
		assert_eq!(HalvingMint::mint_state(), State::Stopped);
		assert_noop!(
			HalvingMint::set_mint_state(RuntimeOrigin::signed(1), State::Running),
			sp_runtime::DispatchError::BadOrigin,
		);
		assert_noop!(
			HalvingMint::set_mint_state(RuntimeOrigin::root(), State::Running),
			Error::<Test, Instance1>::MintNotStarted,
		);
		assert_ok!(HalvingMint::start_mint_from_next_block(RuntimeOrigin::root()));
		assert_eq!(HalvingMint::mint_state(), State::Running);
		assert_noop!(
			HalvingMint::set_mint_state(RuntimeOrigin::root(), State::Running),
			Error::<Test, Instance1>::MintStateUnchanged,
		);
		assert_ok!(HalvingMint::set_mint_state(RuntimeOrigin::root(), State::Stopped));
		assert_eq!(HalvingMint::mint_state(), State::Stopped);
		System::assert_last_event(Event::MintStateChanged { new_state: State::Stopped }.into());
	});
}

#[test]
fn start_mint_too_early_fails() {
	new_test_ext().execute_with(|| {
		assert_eq!(System::block_number(), 1);
		assert_noop!(
			HalvingMint::start_mint_from_block(RuntimeOrigin::root(), 0),
			Error::<Test, Instance1>::StartBlockTooEarly,
		);
		assert_noop!(
			HalvingMint::start_mint_from_block(RuntimeOrigin::root(), 1),
			Error::<Test, Instance1>::StartBlockTooEarly,
		);
		assert_ok!(HalvingMint::start_mint_from_block(RuntimeOrigin::root(), 2));
		System::assert_last_event(Event::MintStarted { start_block: 2 }.into());
	});
}

#[test]
fn halving_mint_works() {
	new_test_ext().execute_with(|| {
		let beneficiary = HalvingMint::beneficiary_account();

		assert_eq!(System::block_number(), 1);
		assert_eq!(Balances::total_issuance(), 10);
		assert_eq!(Balances::free_balance(&beneficiary), 10);
		assert_ok!(HalvingMint::start_mint_from_next_block(RuntimeOrigin::root()));
		System::assert_last_event(Event::MintStarted { start_block: 2 }.into());

		run_to_block(2);
		// 50 tokens are minted
		assert_eq!(Balances::total_issuance(), 60);
		assert_eq!(Balances::free_balance(&beneficiary), 10);
		assert_eq!(Balances::free_balance(&1), 50);

		run_to_block(11);
		assert_eq!(Balances::total_issuance(), 510);
		assert_eq!(Balances::free_balance(&beneficiary), 10);
		assert_eq!(Balances::free_balance(&1), 500);

		run_to_block(12);
		// the first halving
		assert_eq!(Balances::total_issuance(), 535);
		assert_eq!(Balances::free_balance(&beneficiary), 10);
		assert_eq!(Balances::free_balance(&1), 525);

		run_to_block(22);
		// the second halving
		assert_eq!(Balances::total_issuance(), 772);
		assert_eq!(Balances::free_balance(&beneficiary), 10);
		assert_eq!(Balances::free_balance(&1), 762);

		run_to_block(52);
		// the fifth halving - only 1 token is minted
		assert_eq!(Balances::total_issuance(), 971);
		assert_eq!(Balances::free_balance(&beneficiary), 10);
		assert_eq!(Balances::free_balance(&1), 961);

		run_to_block(62);
		// the sixth halving - but 0 tokens will be minted
		assert_eq!(Balances::total_issuance(), 980);
		assert_eq!(Balances::free_balance(&beneficiary), 10);
		assert_eq!(Balances::free_balance(&1), 970);

		run_to_block(1_000);
		// no changes since the sixth halving, the total minted token will be fixated on 980,
		// the "missing" 20 comes from the integer division and the total_issuance is too small.
		//
		// we'll have much accurate result in reality where token unit is 18 decimal
		assert_eq!(Balances::total_issuance(), 980);
		assert_eq!(Balances::free_balance(&beneficiary), 10);
		assert_eq!(Balances::free_balance(&1), 970);
	});
}

#[test]
fn set_on_token_minted_state_works() {
	new_test_ext().execute_with(|| {
		let beneficiary = HalvingMint::beneficiary_account();

		assert_ok!(HalvingMint::start_mint_from_next_block(RuntimeOrigin::root()));
		assert_ok!(HalvingMint::set_on_token_minted_state(RuntimeOrigin::root(), State::Stopped));
		System::assert_last_event(
			Event::OnTokenMintedStateChanged { new_state: State::Stopped }.into(),
		);

		run_to_block(2);
		// 50 tokens are minted, but none is transferred away
		assert_eq!(Balances::total_issuance(), 60);
		assert_eq!(Balances::free_balance(&beneficiary), 60);
		assert_eq!(Balances::free_balance(&1), 0);

		run_to_block(10);
		assert_ok!(HalvingMint::set_on_token_minted_state(RuntimeOrigin::root(), State::Running));
		System::assert_last_event(
			Event::OnTokenMintedStateChanged { new_state: State::Running }.into(),
		);

		run_to_block(11);
		// start to transfer token
		assert_eq!(Balances::total_issuance(), 510);
		assert_eq!(Balances::free_balance(&beneficiary), 460);
		assert_eq!(Balances::free_balance(&1), 50);
	});
}

#[test]
fn set_mint_state_works() {
	new_test_ext().execute_with(|| {
		let beneficiary = HalvingMint::beneficiary_account();

		assert_ok!(HalvingMint::start_mint_from_next_block(RuntimeOrigin::root()));

		run_to_block(2);
		assert_eq!(Balances::total_issuance(), 60);
		assert_eq!(Balances::free_balance(&beneficiary), 10);
		assert_eq!(Balances::free_balance(&1), 50);
		// stop the minting
		assert_ok!(HalvingMint::set_mint_state(RuntimeOrigin::root(), State::Stopped));

		run_to_block(3);
		// no new tokens should be minted
		assert_eq!(Balances::total_issuance(), 60);
		assert_eq!(Balances::free_balance(&beneficiary), 10);
		assert_eq!(Balances::free_balance(&1), 50);

		run_to_block(4);
		// resume the minting
		assert_ok!(HalvingMint::set_mint_state(RuntimeOrigin::root(), State::Running));

		run_to_block(5);
		assert_eq!(Balances::total_issuance(), 110);
		assert_eq!(Balances::free_balance(&beneficiary), 10);
		assert_eq!(Balances::free_balance(&1), 100);
		assert_eq!(HalvingMint::skipped_blocks(), 2);

		// the first halving should be delayed to block 14
		run_to_block(14);
		assert_eq!(Balances::total_issuance(), 535);
		assert_eq!(Balances::free_balance(&beneficiary), 10);
		assert_eq!(Balances::free_balance(&1), 525);
	});
}
