mod balances;
mod system;

mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
}

use types::{AccountId, Balance, BlockNumber, Nonce};

#[derive(Debug)]
struct RuntimeConfig;

impl system::Config for RuntimeConfig {
	type AccountId = AccountId;
	type BlockNumber = BlockNumber;
	type Nonce = Nonce;
}

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<RuntimeConfig>,
	balances: balances::Pallet<AccountId, Balance>,	
}

impl Runtime {
	pub fn new() -> Self {
		Self {
			system: system::Pallet::new(),
			balances: balances::Pallet::new(),
		}
	}
}

fn main() {
	let mut runtime = Runtime::new();
	runtime.balances.set_balance("alice".to_string(), 100);

	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number(), 1);

	// first transaction
	runtime.system.inc_account_nonce("alice".to_string());
	let _res = runtime.balances.transfer("alice".to_string(), "bob".to_string(), 30).map_err(|e| eprintln!("{e}"));

	// second transaction
	runtime.system.inc_account_nonce("alice".to_string());
	let _res = runtime.balances.transfer("alice".to_string(), "charlie".to_string(), 20).map_err(|e| eprintln!("{e}"));
	
	println!("{:#?}", runtime);
}

