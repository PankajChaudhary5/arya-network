use crate::{mock::*, Error, FeelessInfo};
use frame_support::{assert_noop, assert_ok};

#[test]
fn make_transaction_feeless() {
	new_test_ext().execute_with(|| {
		const DEST_ACCOUNT: u64 = 3;
		const VALUE: u64 = 1000;
		let call_transfer = Box::new(call_transfer(DEST_ACCOUNT, VALUE));

		assert_ok!(FeelessModule::add_super_user(Origin::root(), 1, 1));

		// Making feeless transaction call.
		assert_ok!(FeelessModule::make_feeless(Origin::signed(1), call_transfer));
	});
}

#[test]
fn feeless_transaction_limit_exceeds() {
	new_test_ext().execute_with(|| {
		const DEST_ACCOUNT: u64 = 3;
		const VALUE: u64 = 1000;
		let call_transfer = Box::new(call_transfer(DEST_ACCOUNT, VALUE));

		assert_ok!(FeelessModule::add_super_user(Origin::root(), 1, 1));

		// Feeless transaction attempt.
		assert_ok!(FeelessModule::make_feeless(Origin::signed(1), call_transfer.clone()));

		// Ensure the expected error is thrown when the feeless limit exceeds.
		assert_noop!(
			FeelessModule::make_feeless(Origin::signed(1), call_transfer),
			Error::<Test>::ExceedMaxCalls
		);
	});
}

#[test]
fn feeless_tracker() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		const DEST_ACCOUNT: u64 = 3;
		const VALUE: u64 = 1000;
		let call_transfer = Box::new(call_transfer(DEST_ACCOUNT, VALUE));

		// Adding user as super user.
		assert_ok!(FeelessModule::add_super_user(Origin::root(), 1, 1));

		// Making a feeless transaction.
		assert_ok!(FeelessModule::make_feeless(Origin::signed(TEST_ACCOUNT), call_transfer));

		// Read pallet storage and assert an expected result.
		assert_eq!(
			FeelessModule::tracker(TEST_ACCOUNT),
			FeelessInfo { last_user_session: 0, user_calls: 1 }
		);
	});
}

#[test]
fn add_super_user() {
	new_test_ext().execute_with(|| {
		// Adding a super user.
		assert_ok!(FeelessModule::add_super_user(Origin::root(), 1, 1));
	});
}

#[test]
fn already_super_user() {
	new_test_ext().execute_with(|| {
		// Adding a super user.
		assert_ok!(FeelessModule::add_super_user(Origin::root(), 1, 1));

		// Ensure the expected error is thrown when member is already a super user.
		assert_noop!(
			FeelessModule::add_super_user(Origin::root(), 1, 1),
			Error::<Test>::AlreadySuperUser
		);
	});
}

#[test]
fn adding_super_user_when_not_root() {
	const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
	new_test_ext().execute_with(|| {
		assert_noop!(
			FeelessModule::add_super_user(Origin::signed(TEST_ACCOUNT), 1, 1),
			sp_runtime::DispatchError::BadOrigin
		);
	});
}

#[test]
fn removing_super_user_when_not_root() {
	const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
	new_test_ext().execute_with(|| {
		assert_noop!(
			FeelessModule::remove_super_user(Origin::signed(TEST_ACCOUNT), 1),
			sp_runtime::DispatchError::BadOrigin
		);
	});
}

#[test]
fn remove_super_user() {
	new_test_ext().execute_with(|| {
		// Adding a super user.
		assert_ok!(FeelessModule::add_super_user(Origin::root(), 1, 1));

		// Removing super user.
		assert_ok!(FeelessModule::remove_super_user(Origin::root(), 1));
	});
}

#[test]
fn not_super_user() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when member not a super user.
		assert_noop!(
			FeelessModule::remove_super_user(Origin::root(), 1),
			Error::<Test>::NotSuperUser
		);
	});
}
