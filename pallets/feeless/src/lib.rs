#![cfg_attr(not(feature = "std"), no_std)]

pub mod info_types;
pub use info_types::FeelessInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use crate::FeelessInfo;
	use frame_support::{
		dispatch::{DispatchResult, DispatchResultWithPostInfo, Dispatchable},
		pallet_prelude::*,
		traits::Get,
		weights::{GetDispatchInfo, Pays},
		Parameter,
	};
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::boxed::Box;

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The call type from the runtime which has all the calls available in your runtime.
		type Call: Parameter + GetDispatchInfo + Dispatchable<Origin = Self::Origin>;

		/// The maximum amount of calls an account can make in a session.
		#[pallet::constant]
		type MaxCalls: Get<u32>;
	}

	#[pallet::error]
	pub enum Error<T> {
		/// An account cannot make more Calls than 'MaxCalls'.
		ExceedMaxCalls,

		/// Already a super user.
		AlreadySuperUser,

		/// Not a super user.
		NotSuperUser,

		/// When the feeless count is more than the max count permited.
		FeelessTxnCountLimitExceeds,
	}

	/// feeless txn count.
	pub type FeelessCount = u32;

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		ExtrinsicResult {
			/// Account id of the sender/transaction maker.
			tx_sender: T::AccountId,

			/// Feeless code result.
			feeless_result: DispatchResult,
		},

		/// Member added as super user.
		MemberAdded,

		/// Member removed from super user.
		MemberRemoved,
	}

	#[pallet::storage]
	#[pallet::getter(fn tracker)]
	/// Track how many calls each user has done for the latest session
	pub(super) type Tracker<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, FeelessInfo<T::BlockNumber>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn superusers)]
	/// List of Super users.
	pub type SuperUsers<T: Config> = StorageMap<_, Blake2_128, T::AccountId, FeelessCount>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight({
		let dispatch_info = call.get_dispatch_info();
		(dispatch_info.weight.saturating_add(10_000), dispatch_info.class, Pays::Yes)
		})]
		pub fn make_feeless(
			origin: OriginFor<T>,
			call: Box<<T as Config>::Call>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin.clone())?;

			// Get the relevant storage data.
			let max_calls: u32 = T::MaxCalls::get();
			let mut feeless_info = Tracker::<T>::get(&sender);

			// Checking whether the sender is a super user or not.
			let feeless_count: FeelessCount =
				SuperUsers::<T>::get(&sender).ok_or(Error::<T>::NotSuperUser)?;

			// The feeless transaction count should be less than MaxCalls permited.
			ensure!(feeless_count <= max_calls, Error::<T>::FeelessTxnCountLimitExceeds);

			// Checking whether the account is eligible for feeless payment.
			ensure!(feeless_info.user_calls < feeless_count, Error::<T>::ExceedMaxCalls);

			// Update the tracker count.
			feeless_info.user_calls = feeless_info.user_calls.saturating_add(1);

			Tracker::<T>::insert(&sender, feeless_info);

			// Dispatch the call.
			let result = call.dispatch(origin);

			Self::deposit_event(Event::ExtrinsicResult {
				tx_sender: sender,
				feeless_result: result.map(|_| ()).map_err(|e| e.error),
			});

			// Making the tx feeless.
			Ok(Pays::No.into())
		}

		/// Add a member to Super User.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn add_super_user(
			origin: OriginFor<T>,
			who: T::AccountId,
			feeless_count: u32,
		) -> DispatchResult {
			ensure_root(origin.clone())?;

			ensure!(!SuperUsers::<T>::contains_key(&who), Error::<T>::AlreadySuperUser);

			SuperUsers::<T>::insert(&who, feeless_count);

			Self::deposit_event(Event::MemberAdded);
			Ok(())
		}

		/// Remove a member from Super User.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn remove_super_user(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin.clone())?;

			ensure!(SuperUsers::<T>::contains_key(&who), Error::<T>::NotSuperUser);

			SuperUsers::<T>::remove(&who);

			Self::deposit_event(Event::MemberRemoved);
			Ok(())
		}
	}
}
