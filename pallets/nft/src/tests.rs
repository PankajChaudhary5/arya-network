use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, Hashable};
use sp_core::H256;

#[test]
fn basic_mint() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		const TEST_MAX_TOKENS: u128 = 5;
		const TEST_MAX_TOKENS_FOR_ACCOUNT: u64 = 2;

		assert_eq!(Erc721Module::total_supply(), 0);
		assert_eq!(Erc721Module::_max_tokens(), TEST_MAX_TOKENS);
		assert_eq!(Erc721Module::_max_tokens_for_account(), TEST_MAX_TOKENS_FOR_ACCOUNT);

		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT, vec![0]));
		assert_eq!(Erc721Module::total_supply(), 1);
		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT), 1);

		let test_acc_tokens = Erc721Module::tokens_for_account(TEST_ACCOUNT);
		let test_acc_token_id = test_acc_tokens[0];
		assert_eq!(Erc721Module::_owner_of(&test_acc_token_id), Ok(TEST_ACCOUNT));
	});
}

#[test]
fn mint_when_not_origin_error() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;

		assert_eq!(Erc721Module::total_supply(), 0);
		assert_noop!(
			Erc721Module::mint(Origin::signed(TEST_ACCOUNT), TEST_ACCOUNT, Vec::<u8>::default()),
			sp_runtime::DispatchError::BadOrigin
		);
	});
}

#[test]
fn mint_duplicate_error() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		const TEST_ACCOUNT_2: <Test as frame_system::Config>::AccountId = 2;

		assert_eq!(Erc721Module::total_supply(), 0);
		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT, Vec::<u8>::default()));
		assert_noop!(
			Erc721Module::mint(Origin::root(), TEST_ACCOUNT_2, Vec::<u8>::default()),
			Error::<Test>::DuplicateToken
		);
	});
}

#[test]
fn mint_when_max_is_reached_for_token() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		const TEST_ACCOUNT_2: <Test as frame_system::Config>::AccountId = 2;
		const TEST_ACCOUNT_3: <Test as frame_system::Config>::AccountId = 3;

		assert_eq!(Erc721Module::total_supply(), 0);
		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT, vec![0]));
		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT, vec![1]));
		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT_2, vec![2]));
		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT_2, vec![4]));
		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT_3, vec![5]));
		assert_noop!(
			Erc721Module::mint(Origin::root(), TEST_ACCOUNT_3, vec![6]),
			Error::<Test>::TokenLimitExceeded
		);
	});
}

#[test]
fn mint_when_max_is_reached_for_account() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;

		assert_eq!(Erc721Module::total_supply(), 0);
		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT, vec![0]));
		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT, vec![1]));
		assert_noop!(
			Erc721Module::mint(Origin::root(), TEST_ACCOUNT, vec![2]),
			Error::<Test>::TokenLimitForAccountExceeded
		);
	});
}

#[test]
fn mint_with_lots_of_tokens() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		const TEST_MAX_TOKENS: u128 = 5;
		const TEST_MAX_TOKENS_FOR_ACCOUNT: u64 = 2;

		assert_eq!(Erc721Module::total_supply(), 0);
		assert_eq!(Erc721Module::_max_tokens(), TEST_MAX_TOKENS);
		assert_eq!(Erc721Module::_max_tokens_for_account(), TEST_MAX_TOKENS_FOR_ACCOUNT);

		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT, vec![0]));
		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT, vec![1]));
		assert_eq!(Erc721Module::total_supply(), 2);
		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT), 2);

		let test_acc_tokens = Erc721Module::tokens_for_account(TEST_ACCOUNT);
		let test_acc_token_id = test_acc_tokens[1];
		assert_eq!(Erc721Module::_owner_of(&test_acc_token_id), Ok(TEST_ACCOUNT));
	});
}

#[test]
fn basic_burn() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		const TEST_MAX_TOKENS: u128 = 5;
		const TEST_MAX_TOKENS_FOR_ACCOUNT: u64 = 2;

		assert_eq!(Erc721Module::total_supply(), 0);
		assert_eq!(Erc721Module::_max_tokens(), TEST_MAX_TOKENS);
		assert_eq!(Erc721Module::_max_tokens_for_account(), TEST_MAX_TOKENS_FOR_ACCOUNT);

		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT, Vec::<u8>::default()));

		assert_ok!(Erc721Module::burn(
			Origin::signed(TEST_ACCOUNT),
			Vec::<u8>::default().blake2_256().into()
		));
		assert_eq!(Erc721Module::total_supply(), 0);
		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT), 0);

		let test_acc_tokens = Erc721Module::tokens_for_account(TEST_ACCOUNT);
		assert_eq!(test_acc_tokens.len(), 0);
	});
}

#[test]
fn burn_when_token_doesnt_exist() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;

		assert_noop!(
			Erc721Module::burn(
				Origin::signed(TEST_ACCOUNT),
				Vec::<u8>::default().blake2_256().into()
			),
			Error::<Test>::TokenDoesntExist
		);
	});
}

#[test]
fn burn_when_not_owner() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		const TEST_ACCOUNT_2: <Test as frame_system::Config>::AccountId = 2;
		const TEST_MAX_TOKENS: u128 = 5;
		const TEST_MAX_TOKENS_FOR_ACCOUNT: u64 = 2;

		assert_eq!(Erc721Module::total_supply(), 0);
		assert_eq!(Erc721Module::_max_tokens(), TEST_MAX_TOKENS);
		assert_eq!(Erc721Module::_max_tokens_for_account(), TEST_MAX_TOKENS_FOR_ACCOUNT);
		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT, Vec::<u8>::default()));
		assert_noop!(
			Erc721Module::burn(
				Origin::signed(TEST_ACCOUNT_2),
				Vec::<u8>::default().blake2_256().into()
			),
			Error::<Test>::InvalidTokenOwner
		);
	});
}

#[test]
fn burn_with_lots_of_tokens() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		const TEST_ACCOUNT_2: <Test as frame_system::Config>::AccountId = 2;
		const TEST_MAX_TOKENS: u128 = 5;
		const TEST_MAX_TOKENS_FOR_ACCOUNT: u64 = 2;

		assert_eq!(Erc721Module::total_supply(), 0);
		assert_eq!(Erc721Module::_max_tokens(), TEST_MAX_TOKENS);
		assert_eq!(Erc721Module::_max_tokens_for_account(), TEST_MAX_TOKENS_FOR_ACCOUNT);

		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT, Vec::<u8>::default()));
		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT, vec![1]));
		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT_2, vec![2]));
		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT_2, vec![3]));
		assert_eq!(Erc721Module::total_supply(), 4);
		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT), 2);
		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT_2), 2);

		assert_ok!(Erc721Module::burn(
			Origin::signed(TEST_ACCOUNT),
			Vec::<u8>::default().blake2_256().into()
		));
		assert_ok!(Erc721Module::burn(
			Origin::signed(TEST_ACCOUNT_2),
			vec![2u8].blake2_256().into()
		));
		assert_eq!(Erc721Module::total_supply(), 2);
		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT), 1);
		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT_2), 1);
	});
}

#[test]
fn custodian_burn_when_custodian() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		const TEST_ACCOUNT_CUSTODIAN: <Test as frame_system::Config>::AccountId = 2;
		const TEST_MAX_TOKENS: u128 = 5;
		const TEST_MAX_TOKENS_FOR_ACCOUNT: u64 = 2;

		assert_eq!(Erc721Module::total_supply(), 0);
		assert_eq!(Erc721Module::_max_tokens(), TEST_MAX_TOKENS);
		assert_eq!(Erc721Module::_max_tokens_for_account(), TEST_MAX_TOKENS_FOR_ACCOUNT);
		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT, Vec::<u8>::default()));

		assert_ok!(Erc721Module::set_custodian(
			Origin::signed(TEST_ACCOUNT),
			TEST_ACCOUNT_CUSTODIAN,
			Vec::<u8>::default().blake2_256().into()
		));

		assert_ok!(Erc721Module::custodian_burn(
			Origin::signed(TEST_ACCOUNT_CUSTODIAN),
			Vec::<u8>::default().blake2_256().into()
		));
		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT), 0);
	});
}

#[test]
fn custodian_burn_when_custodian_not_set() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		const TEST_ACCOUNT_CUSTODIAN: <Test as frame_system::Config>::AccountId = 2;
		const TEST_MAX_TOKENS: u128 = 5;
		const TEST_MAX_TOKENS_FOR_ACCOUNT: u64 = 2;

		assert_eq!(Erc721Module::total_supply(), 0);
		assert_eq!(Erc721Module::_max_tokens(), TEST_MAX_TOKENS);
		assert_eq!(Erc721Module::_max_tokens_for_account(), TEST_MAX_TOKENS_FOR_ACCOUNT);
		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT, Vec::<u8>::default()));

		assert_noop!(
			Erc721Module::custodian_burn(
				Origin::signed(TEST_ACCOUNT_CUSTODIAN),
				Vec::<u8>::default().blake2_256().into()
			),
			Error::<Test>::TokenDoesntExistForCustodian
		);
	});
}

fn custodian_burn_when_not_custodian() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		const TEST_ACCOUNT_CUSTODIAN: <Test as frame_system::Config>::AccountId = 2;
		const TEST_MAX_TOKENS: u128 = 5;
		const TEST_MAX_TOKENS_FOR_ACCOUNT: u64 = 2;

		assert_eq!(Erc721Module::total_supply(), 0);
		assert_eq!(Erc721Module::_max_tokens(), TEST_MAX_TOKENS);
		assert_eq!(Erc721Module::_max_tokens_for_account(), TEST_MAX_TOKENS_FOR_ACCOUNT);
		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT, Vec::<u8>::default()));
		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT_CUSTODIAN, vec![1]));

		assert_noop!(
			Erc721Module::custodian_burn(
				Origin::signed(TEST_ACCOUNT_CUSTODIAN),
				Vec::<u8>::default().blake2_256().into()
			),
			Error::<Test>::InvalidCustodian
		);
	});
}

#[test]
fn set_custodian_when_not_owner() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		const TEST_ACCOUNT_2: <Test as frame_system::Config>::AccountId = 2;
		const TEST_ACCOUNT_CUSTODIAN: <Test as frame_system::Config>::AccountId = 3;

		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT, Vec::<u8>::default()));
		assert_noop!(
			Erc721Module::set_custodian(
				Origin::signed(TEST_ACCOUNT_2),
				TEST_ACCOUNT_CUSTODIAN,
				Vec::<u8>::default().blake2_256().into()
			),
			Error::<Test>::InvalidTokenOwner
		);
	});
}

#[test]
fn basic_transfer() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		const TEST_ACCOUNT_2: <Test as frame_system::Config>::AccountId = 2;

		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT, Vec::<u8>::default()));
		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT), 1);
		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT_2), 0);

		assert_ok!(Erc721Module::transfer_from(
			Origin::signed(TEST_ACCOUNT),
			TEST_ACCOUNT_2,
			Vec::<u8>::default().blake2_256().into()
		));

		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT), 0);
		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT_2), 1);
	});
}

#[test]
fn transfer_when_not_owner() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		const TEST_ACCOUNT_2: <Test as frame_system::Config>::AccountId = 2;

		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT, Vec::<u8>::default()));
		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT), 1);
		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT_2), 0);

		assert_noop!(
			Erc721Module::transfer_from(
				Origin::signed(TEST_ACCOUNT_2),
				TEST_ACCOUNT,
				Vec::<u8>::default().blake2_256().into()
			),
			Error::<Test>::InvalidTokenOwner
		);
	});
}

#[test]
fn transfer_when_account_max_reached() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		const TEST_ACCOUNT_2: <Test as frame_system::Config>::AccountId = 2;

		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT, Vec::<u8>::default()));
		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT, vec![1]));
		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT_2, vec![2]));
		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT), 2);
		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT_2), 1);

		assert_noop!(
			Erc721Module::transfer_from(
				Origin::signed(TEST_ACCOUNT_2),
				TEST_ACCOUNT,
				vec![2u8].blake2_256().into()
			),
			Error::<Test>::TokenLimitForAccountExceeded
		);
	});
}

#[test]
fn transfer_when_token_not_owned() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		const TEST_ACCOUNT_2: <Test as frame_system::Config>::AccountId = 2;

		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT_2, Vec::<u8>::default()));
		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT), 0);

		assert_noop!(
			Erc721Module::transfer_from(
				Origin::signed(TEST_ACCOUNT),
				TEST_ACCOUNT,
				Vec::<u8>::default().blake2_256().into()
			),
			Error::<Test>::InvalidTokenOwner
		);
	});
}

#[test]
fn basic_custodian_transfer() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		const TEST_ACCOUNT_2: <Test as frame_system::Config>::AccountId = 2;
		const TEST_ACCOUNT_CUSTODIAN: <Test as frame_system::Config>::AccountId = 3;
		const TEST_MAX_TOKENS: u128 = 5;
		const TEST_MAX_TOKENS_FOR_ACCOUNT: u64 = 2;

		assert_eq!(Erc721Module::total_supply(), 0);
		assert_eq!(Erc721Module::_max_tokens(), TEST_MAX_TOKENS);
		assert_eq!(Erc721Module::_max_tokens_for_account(), TEST_MAX_TOKENS_FOR_ACCOUNT);
		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT, Vec::<u8>::default()));
		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT), 1);

		assert_ok!(Erc721Module::set_custodian(
			Origin::signed(TEST_ACCOUNT),
			TEST_ACCOUNT_CUSTODIAN,
			Vec::<u8>::default().blake2_256().into()
		));

		assert_ok!(Erc721Module::custodian_transfer(
			Origin::signed(TEST_ACCOUNT_CUSTODIAN),
			TEST_ACCOUNT_2,
			Vec::<u8>::default().blake2_256().into()
		));
		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT), 0);
		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT_2), 1);
	});
}

#[test]
fn transfer_then_custodian_transfer() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		const TEST_ACCOUNT_2: <Test as frame_system::Config>::AccountId = 2;
		const TEST_ACCOUNT_CUSTODIAN: <Test as frame_system::Config>::AccountId = 3;

		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT, Vec::<u8>::default()));
		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT), 1);
		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT_2), 0);

		assert_ok!(Erc721Module::set_custodian(
			Origin::signed(TEST_ACCOUNT),
			TEST_ACCOUNT_CUSTODIAN,
			Vec::<u8>::default().blake2_256().into()
		));

		assert_ok!(Erc721Module::transfer_from(
			Origin::signed(TEST_ACCOUNT),
			TEST_ACCOUNT_2,
			Vec::<u8>::default().blake2_256().into()
		));

		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT), 0);
		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT_2), 1);

		assert_noop!(
			Erc721Module::custodian_transfer(
				Origin::signed(TEST_ACCOUNT_CUSTODIAN),
				TEST_ACCOUNT,
				Vec::<u8>::default().blake2_256().into()
			),
			Error::<Test>::TokenDoesntExistForCustodian
		);
	});
}

#[test]
fn burn_then_custodian_burn() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		const TEST_ACCOUNT_CUSTODIAN: <Test as frame_system::Config>::AccountId = 3;

		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT, Vec::<u8>::default()));
		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT), 1);

		assert_ok!(Erc721Module::set_custodian(
			Origin::signed(TEST_ACCOUNT),
			TEST_ACCOUNT_CUSTODIAN,
			Vec::<u8>::default().blake2_256().into()
		));

		assert_ok!(Erc721Module::burn(
			Origin::signed(TEST_ACCOUNT),
			Vec::<u8>::default().blake2_256().into()
		));

		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT), 0);

		assert_noop!(
			Erc721Module::custodian_burn(
				Origin::signed(TEST_ACCOUNT_CUSTODIAN),
				Vec::<u8>::default().blake2_256().into()
			),
			Error::<Test>::TokenDoesntExistForCustodian
		);
	});
}

#[test]
fn burn_then_custodian_transfer() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		const TEST_ACCOUNT_2: <Test as frame_system::Config>::AccountId = 2;
		const TEST_ACCOUNT_CUSTODIAN: <Test as frame_system::Config>::AccountId = 3;

		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT, Vec::<u8>::default()));
		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT), 1);

		assert_ok!(Erc721Module::set_custodian(
			Origin::signed(TEST_ACCOUNT),
			TEST_ACCOUNT_CUSTODIAN,
			Vec::<u8>::default().blake2_256().into()
		));

		assert_ok!(Erc721Module::burn(
			Origin::signed(TEST_ACCOUNT),
			Vec::<u8>::default().blake2_256().into()
		));

		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT), 0);

		assert_noop!(
			Erc721Module::custodian_transfer(
				Origin::signed(TEST_ACCOUNT_CUSTODIAN),
				TEST_ACCOUNT_2,
				Vec::<u8>::default().blake2_256().into()
			),
			Error::<Test>::TokenDoesntExistForCustodian
		);
	});
}

#[test]
fn custodian_transfer_when_not_custodian() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		const TEST_ACCOUNT_2: <Test as frame_system::Config>::AccountId = 2;
		const TEST_ACCOUNT_CUSTODIAN: <Test as frame_system::Config>::AccountId = 3;
		const TEST_ACCOUNT_CUSTODIAN_2: <Test as frame_system::Config>::AccountId = 4;

		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT, Vec::<u8>::default()));
		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT), 1);

		assert_ok!(Erc721Module::set_custodian(
			Origin::signed(TEST_ACCOUNT),
			TEST_ACCOUNT_CUSTODIAN,
			Vec::<u8>::default().blake2_256().into()
		));

		assert_noop!(
			Erc721Module::custodian_transfer(
				Origin::signed(TEST_ACCOUNT_CUSTODIAN_2),
				TEST_ACCOUNT_2,
				Vec::<u8>::default().blake2_256().into()
			),
			Error::<Test>::InvalidCustodian
		);
	});
}

#[test]
fn custodian_transfer_when_account_max_reached() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		const TEST_ACCOUNT_2: <Test as frame_system::Config>::AccountId = 2;
		const TEST_ACCOUNT_CUSTODIAN: <Test as frame_system::Config>::AccountId = 3;

		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT, Vec::<u8>::default()));
		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT, vec![1]));
		assert_ok!(Erc721Module::mint(Origin::root(), TEST_ACCOUNT_2, vec![2]));
		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT), 2);
		assert_eq!(Erc721Module::token_count_for_account(TEST_ACCOUNT_2), 1);

		assert_ok!(Erc721Module::set_custodian(
			Origin::signed(TEST_ACCOUNT_2),
			TEST_ACCOUNT_CUSTODIAN,
			vec![2u8].blake2_256().into()
		));

		assert_noop!(
			Erc721Module::custodian_transfer(
				Origin::signed(TEST_ACCOUNT_CUSTODIAN),
				TEST_ACCOUNT,
				vec![2u8].blake2_256().into()
			),
			Error::<Test>::TokenLimitForAccountExceeded
		);
	});
}
