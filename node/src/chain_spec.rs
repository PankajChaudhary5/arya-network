use hex_literal::hex;
use node_template_runtime::{
	AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig, Signature, SudoConfig,
	SystemConfig, WASM_BINARY,
};
use sc_service::ChainType;
use runtime_common::constants::{Balance, ARYA, TOKEN_DECIMALS};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

const TOKEN_SYMBOL: &str = "ARYA";
const SS_58_FORMAT: u32 = 51;

/// Total supply of token is 100_000_000.
/// Initially we are distributing the total supply to the multiple accounts which is representing
/// its category pool which we will update in later part of development.
const SEED_SUPPLY: Balance = ARYA * 50_000_000;
const INITIAL_SALE: Balance = ARYA * 50_000_000;

/// Generate a default spec for the live chain (Prod).
pub fn arya_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Live wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Arya Live",
		// ID
		"prod",
		ChainType::Live,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					(
						hex!["88fd77d706e168d78713a6a927c1ddfae367b081fb2829b119bbcc6db9af401d"]
							.into(),
						SEED_SUPPLY,
					),
					(
						hex!["04063fc1cbba917ced6c45091bf631de6a4db584dd55c1d67431661a5d57a575"]
							.into(),
						INITIAL_SALE,
					),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					(get_account_id_from_seed::<sr25519::Public>("Alice"), 2000000),
					(get_account_id_from_seed::<sr25519::Public>("Bob"), 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Charlie"), 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Dave"), 10000000),
					(get_account_id_from_seed::<sr25519::Public>("Eve"), 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Ferdie"), 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Alice//stash"), 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Bob//stash"), 10000000),
					(get_account_id_from_seed::<sr25519::Public>("Charlie//stash"), 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Dave//stash"), 1000),
					(get_account_id_from_seed::<sr25519::Public>("Eve//stash"), 1000),
					(get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"), 1000),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					(get_account_id_from_seed::<sr25519::Public>("Alice"), 2000000),
					(get_account_id_from_seed::<sr25519::Public>("Bob"), 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Charlie"), 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Dave"), 10000000),
					(get_account_id_from_seed::<sr25519::Public>("Eve"), 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Ferdie"), 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Alice//stash"), 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Bob//stash"), 10000000),
					(get_account_id_from_seed::<sr25519::Public>("Charlie//stash"), 1000000),
					(get_account_id_from_seed::<sr25519::Public>("Dave//stash"), 1000),
					(get_account_id_from_seed::<sr25519::Public>("Eve//stash"), 1000),
					(get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"), 1000),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		None,
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<(AccountId, Balance)>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts,
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
	}
}