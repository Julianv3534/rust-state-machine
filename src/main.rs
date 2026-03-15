mod balances;
mod support;
mod system;

use crate::support::Dispatch;

mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
	pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
	pub type Header = crate::support::Header<BlockNumber>;
	pub type Block = crate::support::Block<Header, Extrinsic>;
}

pub enum RuntimeCall {
	BalancesTransfer { to: types::AccountId, amount: types::Balance },
}

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<RuntimeConfig>,
	balances: balances::Pallet<RuntimeConfig>,
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
	type Balance = types::Balance;
}

impl Runtime {
	pub fn new() -> Self {
		Self {
			system: system::Pallet::new(),
			balances: balances::Pallet::new(),
		}
	}

	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		self.system.inc_block_number();
		if block.header.block_number != self.system.block_number() {
			return Err("Block number mismatch.");
		}

		for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
			self.system.inc_account_nonce(caller.clone());
			let _res = self.dispatch(caller, call).map_err(|e| {
				eprintln!(
					"Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
					block.header.block_number, i, e
				)
			});
		}

		Ok(())
	}
}

impl crate::support::Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::AccountId;
	type Call = RuntimeCall;
	// Dispatch a call on behalf of a caller. Increments the caller's nonce.
	//
	// Dispatch allows us to identify which underlying module call we want to execute.
	// Note that we extract the `caller` from the extrinsic, and use that information
	// to determine who we are executing the call on behalf of.
	fn dispatch(
		&mut self,
		caller: Self::Caller,
		runtime_call: Self::Call,
	) -> support::DispatchResult {
		
		match runtime_call {
    	RuntimeCall::BalancesTransfer { to, amount } => {
        self.balances.transfer(caller, to, amount).map_err(|e| { eprintln!("{e}");e })
    		}
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
	
	println!("{runtime:#?}");
}

