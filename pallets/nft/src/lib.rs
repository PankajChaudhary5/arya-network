#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

// Todo: Remove Warnings
// Todo: Add benchmarking
// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

// Todo: Research need of `try_get()`

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::DispatchResult, pallet_prelude::*, sp_runtime::traits::Hash, sp_std::vec::Vec,
		traits::StorageVersion,
	};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Runtime Event Defenition
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// Max number of tokens of this type that can exist
		type TokenLimit: Get<u128>;
		/// Max number of tokens of this type that one account may posess
		type TokenLimitForAccount: Get<u64>;
		/// Token creator who is able to mint new instances of this Token
		type TokenCreator: EnsureOrigin<Self::Origin>;
	}

	// Todo: What is the Storage version?? Look this up
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	/* Todo: What do these proc macro attributes do? */
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// Errors.
	#[pallet::error]
	pub enum Error<T> {
		/// Attempted to mint a duplicate Token
		DuplicateToken,
		/// Attempted to burn a Token which doesnt exist or Query an incorrect TokenID
		TokenDoesntExist,
		/// Someone who is not the owner of the token attempts to transfer or destroy it
		InvalidTokenOwner,
		/// Someone who is not the custodian attempting to transfer or destroy token
		InvalidCustodian,
		/// Attempted to transfer or mint a token for an account which is over their TokenLimit
		/// for this particular TokenId
		TokenLimitForAccountExceeded,
		/// Attempted to mint Token when the maximum for this Token type is already reached.
		TokenLimitExceeded,
		/// This custodian does not have any ownership of this token.
		TokenDoesntExistForCustodian,
	}

	/// Custom Alias for a token id
	pub type TokenId<T> = <T as frame_system::Config>::Hash;

	/// Bytes which are hashed to produce a TokenId
	pub type BytesToHash = Vec<u8>;

	// Events.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Token is created and distributed to the account
		Minted {
			token: TokenId<T>,
			owner: T::AccountId,
		},

		/// Token is destroyed
		Burned {
			owner: T::AccountId,
			token: TokenId<T>,
		},

		/// @dev This emits when the approved address for an NFT is changed or
		///  reaffirmed. The zero address indicates there is no approved address.
		///  When a Transfer event emits, this also indicates that the approved
		///  address for that NFT (if any) is reset to none.
		Approval {
			owner: T::AccountId,
			approved: T::AccountId,
			token: TokenId<T>,
		},

		/// Ownership of the token is transferred to the specified Account
		Transferred {
			from: T::AccountId,
			to: T::AccountId,
			token: TokenId<T>,
		},

		/// Custodial Transfer of a token
		CustodialTransfer {
			custodian: T::AccountId,
			from: T::AccountId,
			to: T::AccountId,
			token: TokenId<T>,
		},

		/// Custodial Burn of a token
		CustodialBurn {
			custodian: T::AccountId,
			owner: T::AccountId,
			token: TokenId<T>,
		},

		/// Current Custodian of a token
		Custodian {
			custodian: T::AccountId,
			token: TokenId<T>,
		},

		/// @dev This emits when an operator is enabled or disabled for an owner.
		///  The operator can manage all NFTs of the owner.
		ApprovalForAll {
			owner: T::AccountId,
			operator: T::AccountId,
			approved: bool,
		},

		// Emits Number of Tokens of a particular Id for an Account
		TokenCountFor {
			owner: T::AccountId,
			token_count: u64,
		},

		TokenOwner {
			token: TokenId<T>,
			owner: T::AccountId,
		},
	}

	//Todo: Check whether ValueQuery or OptionQuery is better..

	#[pallet::storage]
	#[pallet::getter(fn total_supply)]
	/// Total number of tokens in existence.
	pub type TotalSupply<T> = StorageValue<_, u128, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn total_burned)]
	/// Total number of tokens destroyed
	pub type TotalBurned<T> = StorageValue<_, u128, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn token_count_for_account)]
	pub type TokenCountForAccount<T: Config> =
		StorageMap<_, Blake2_128, T::AccountId, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn account_for_token)]
	/// A mapping from TokenId to Account
	pub type AccountForToken<T: Config> = StorageMap<_, Blake2_128, TokenId<T>, T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn tokens_for_account)]
	/// A mapping from a Account to a list of TokenIds of this type which are owned by this Account.
	pub type TokensForAccount<T: Config> =
		StorageMap<_, Blake2_128, T::AccountId, Vec<TokenId<T>>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn custodian_for_token)]
	/// A mapping from token to a custodian account.
	pub type CustodianForToken<T: Config> = StorageMap<_, Blake2_128, TokenId<T>, T::AccountId>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		balances: Vec<(T::AccountId, Vec<BytesToHash>)>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { balances: vec![] } // Todo: fill with with (`Alice account_id`, `Some ref to MutantApes`)
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			for (account, bytes_to_hash) in self.balances.iter() {
				for byte in bytes_to_hash {
					if let Err(e) = Pallet::<T>::_mint(account, byte.clone()) {
						panic!("Error with genesis mint: {:?}", e);
					}
				}
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().reads(1))]
		pub fn balance_of(origin: OriginFor<T>, owner: T::AccountId) -> DispatchResult {
			ensure_signed(origin)?;

			let token_count = Self::token_count_for_account(&owner);

			Self::deposit_event(Event::TokenCountFor { owner, token_count });

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads(1))]
		pub fn owner_of(origin: OriginFor<T>, token: TokenId<T>) -> DispatchResult {
			ensure_signed(origin)?;

			let owner = Self::_owner_of(&token)?;

			Self::deposit_event(Event::TokenOwner { owner, token });
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(8,6))]
		pub fn transfer_from(
			origin: OriginFor<T>,
			to: T::AccountId,
			token: TokenId<T>,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;
			let token_owner = Self::_owner_of(&token)?;
			ensure!(from == token_owner, Error::<T>::InvalidTokenOwner);

			Self::_transfer(&from, &to, &token)?;

			match Self::_custodian_of(&token) {
				Ok(_) => Self::_remove_custodian(&token)?,
				Err(_) => {},
			}

			Self::deposit_event(Event::Transferred { from, to, token });
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(5,4))]
		pub fn mint(
			origin: OriginFor<T>,
			for_: T::AccountId,
			bytes_to_be_hashed: BytesToHash,
		) -> DispatchResult {
			T::TokenCreator::ensure_origin(origin)?;
			let token = Self::_mint(&for_, bytes_to_be_hashed.clone())?;
			Self::deposit_event(Event::Minted { token, owner: for_ });
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(7,6))]
		pub fn burn(origin: OriginFor<T>, token: TokenId<T>) -> DispatchResult {
			let for_ = ensure_signed(origin)?;
			let token_owner = Self::_owner_of(&token)?;
			ensure!(for_ == token_owner, Error::<T>::InvalidTokenOwner);

			Self::_burn(&for_, &token)?;

			match Self::_custodian_of(&token) {
				Ok(_) => Self::_remove_custodian(&token)?,
				Err(_) => {},
			}

			Self::deposit_event(Event::Burned { owner: for_, token });
			Ok(())
		}

		/// ERC721 getApproved => custodian_of
		#[pallet::weight(10_000 + T::DbWeight::get().reads(1))]
		pub fn custodian_of(origin: OriginFor<T>, token: TokenId<T>) -> DispatchResult {
			ensure_signed(origin)?;

			let custodian = Self::_custodian_of(&token)?;

			Self::deposit_event(Event::Custodian { custodian, token });
			Ok(())
		}

		/// ERC721 approve => set_custodian
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn set_custodian(
			origin: OriginFor<T>,
			custodian: T::AccountId,
			token: TokenId<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let owner = Self::_owner_of(&token)?;
			ensure!(who == owner, Error::<T>::InvalidTokenOwner);
			Self::_set_custodian(&custodian, &token)?;
			Self::deposit_event(Event::Approval { owner, approved: custodian, token });
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(8,6))]
		pub fn custodian_transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			token: TokenId<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let custodian = Self::_custodian_of(&token)?;
			let from = Self::_owner_of(&token)?;

			ensure!(who == custodian, Error::<T>::InvalidCustodian);
			Self::_transfer(&from, &to, &token)?;
			Self::_remove_custodian(&token)?;

			Self::deposit_event(Event::CustodialTransfer { custodian, from, to, token });
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(7,6))]
		pub fn custodian_burn(origin: OriginFor<T>, token: TokenId<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let custodian = Self::_custodian_of(&token)?;
			ensure!(who == custodian, Error::<T>::InvalidCustodian);
			let for_ = Self::_owner_of(&token)?;
			Self::_burn(&for_, &token)?;
			Self::_remove_custodian(&token)?;
			Self::deposit_event(Event::<T>::CustodialBurn { custodian, owner: for_, token });
			Ok(())
		}

		// TODO: fn setApprovalForAll

		// TODO: fn isApprovedForAll
	}

	impl<T: Config> Pallet<T> {
		// Intrinsics.. mint, burn, transfer, etc.

		// READ-ONLY Instrics
		pub fn _owner_of(token: &TokenId<T>) -> Result<T::AccountId, DispatchError> {
			let owner = AccountForToken::<T>::get(&token).ok_or(Error::<T>::TokenDoesntExist)?;
			Ok(owner)
		}

		pub fn _custodian_of(token: &TokenId<T>) -> Result<T::AccountId, DispatchError> {
			let custodian = CustodianForToken::<T>::get(&token)
				.ok_or(Error::<T>::TokenDoesntExistForCustodian)?;
			Ok(custodian)
		}

		pub fn _max_tokens() -> u128 {
			T::TokenLimit::get()
		}

		pub fn _max_tokens_for_account() -> u64 {
			T::TokenLimitForAccount::get()
		}

		// READ and WRITE Instrinsics MUST BE PRIVATE!
		fn _mint(
			for_: &T::AccountId,
			bytes_to_be_hashed: BytesToHash,
		) -> Result<TokenId<T>, DispatchError> {
			let token_id = T::Hashing::hash_of(&bytes_to_be_hashed);

			ensure!(!AccountForToken::<T>::contains_key(&token_id), Error::<T>::DuplicateToken);
			ensure!(Self::total_supply() < Self::_max_tokens(), Error::<T>::TokenLimitExceeded);
			ensure!(
				Self::token_count_for_account(&for_) < Self::_max_tokens_for_account(),
				Error::<T>::TokenLimitForAccountExceeded
			);

			TotalSupply::<T>::mutate(|count| *count = count.saturating_add(1));
			TokenCountForAccount::<T>::mutate(for_, |count| *count = count.saturating_add(1));
			TokensForAccount::<T>::mutate(for_, |tokens| {
				match tokens.binary_search(&token_id) {
					/* The token_id should not be found because it is unique and new
					   This allows for an arbitrary order to be established and BS to be
					   used! - credit to Dan Forbes for this nice clever trick!
					*/
					Ok(_) => {},
					Err(pos) => tokens.insert(pos, token_id),
				}
			});
			AccountForToken::<T>::insert(token_id, for_);

			Ok(token_id)
		}

		fn _burn(for_: &T::AccountId, token: &TokenId<T>) -> DispatchResult {
			let for_tokens = Self::tokens_for_account(&for_);
			let pos = for_tokens.binary_search(&token).map_err(|_| Error::<T>::TokenDoesntExist)?;
			TokensForAccount::<T>::mutate(for_, |tokens| tokens.remove(pos));

			TokenCountForAccount::<T>::mutate(for_, |count| *count = count.saturating_sub(1));
			TotalSupply::<T>::mutate(|count| *count = count.saturating_sub(1));
			TotalBurned::<T>::mutate(|count| *count = count.saturating_add(1));
			AccountForToken::<T>::remove(token);

			Ok(())
		}

		fn _transfer(from: &T::AccountId, to: &T::AccountId, token: &TokenId<T>) -> DispatchResult {
			ensure!(
				Self::token_count_for_account(&to) < T::TokenLimitForAccount::get(),
				Error::<T>::TokenLimitForAccountExceeded
			);

			let from_tokens = Self::tokens_for_account(&from);
			let pos =
				from_tokens.binary_search(&token).map_err(|_| Error::<T>::TokenDoesntExist)?;
			TokensForAccount::<T>::mutate(from, |tokens| tokens.remove(pos));

			TokenCountForAccount::<T>::mutate(from, |count| *count = count.saturating_sub(1));
			TokensForAccount::<T>::mutate(to, |tokens| {
				match tokens.binary_search(&token) {
					/* The token_id should not be found because it is unique and new
					   This allows for an arbitrary order to be established and BS to be
					   used! - credit to Dan Forbes for this nice clever trick!
					*/
					Ok(_) => {},
					Err(pos) => tokens.insert(pos, token.clone()),
				}
			});
			TokenCountForAccount::<T>::mutate(to, |count| *count = count.saturating_add(1));
			AccountForToken::<T>::insert(&token, &to);

			Ok(())
		}

		fn _set_custodian(custodian: &T::AccountId, token: &TokenId<T>) -> DispatchResult {
			CustodianForToken::<T>::insert(&token, &custodian);
			Ok(())
		}

		fn _remove_custodian(token: &TokenId<T>) -> DispatchResult {
			CustodianForToken::<T>::remove(token);
			Ok(())
		}
	}
}
