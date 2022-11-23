// common types and constants used in both pallet tests and runtime
#![cfg_attr(not(feature = "std"), no_std)]
pub mod constants {

	pub type Balance = u128;

	pub const TOKEN_DECIMALS: u32 = 12;
	const TOKEN_BASE: u128 = 10;
	pub const ARYA: Balance = TOKEN_BASE.pow(TOKEN_DECIMALS);
	pub const MILLIARYA: Balance = ARYA / 1000;
	pub const MICROARYA: Balance = MILLIARYA / 1000;
	pub const GIGAARYA: Balance = MICROARYA / 1000;

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 15 * MICROARYA + (bytes as Balance) * 6 * MICROARYA
	}
}
