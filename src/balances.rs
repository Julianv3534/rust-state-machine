use std::collections::BTreeMap;
use num::traits::{CheckedAdd, CheckedSub, Zero};

pub trait Config: crate::system::Config {
	type Balance: Zero + CheckedAdd + CheckedSub + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	//Why BTreeMap and not HashMap?
	balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: T::AccountId, amount: T::Balance) {
		self.balances.insert(who, amount);
	}

	pub fn balance(&self, who: &T::AccountId) -> T::Balance {
		*self.balances.get(who).unwrap_or(&T::Balance::zero())
	}

	pub fn transfer(
		&mut self,
		caller: T::AccountId,
		to: T::AccountId,
		amount: T::Balance,
	) -> crate::support::DispatchResult {
		let from_balance = self.balance(&caller);
		let new_from_balance = from_balance
	    	.checked_sub(&amount)
    	.ok_or("Not enough funds.")?;

		// Update the balance of the caller
		self.set_balance(caller, new_from_balance);

		// Update the balance of the recipient
		let to_balance = self.balance(&to);
		let new_to_balance = to_balance
		.checked_add(&amount)
		.ok_or("Balance overflow.")?;
		self.set_balance(to, new_to_balance);

		Ok(())
	}
}

pub enum Call<T: Config> {
	Transfer { to: T::AccountId, amount: T::Balance },
}

impl<T: Config> crate::support::Dispatch for Pallet<T> {
	type Caller = T::AccountId;
	type Call = Call<T>;

	fn dispatch(
		&mut self,
		caller: Self::Caller,
		call: Self::Call,
	) -> crate::support::DispatchResult {
		match call {
			Call::Transfer { to, amount } => self.transfer(caller, to, amount),
		};
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::{Config, Pallet};

	struct TestConfig;

	impl Config for TestConfig {
		type Balance = u128;
	}

	impl crate::system::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn init_balances() {
		let mut balances = Pallet::<TestConfig>::new();

		assert_eq!(balances.balance(&"alice".to_string()), 0);
		balances.set_balance("alice".to_string(), 100);
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}

	#[test]
	fn transfer_balance() {
		let mut balances = Pallet::<TestConfig>::new();

		assert_eq!(
			balances.transfer("alice".to_string(), "bob".to_string(), 51),
			Err("Not enough funds.")
		);

		balances.set_balance("alice".to_string(), 100);
		assert_eq!(balances.transfer("alice".to_string(), "bob".to_string(), 51), Ok(()));
		assert_eq!(balances.balance(&"alice".to_string()), 49);
		assert_eq!(balances.balance(&"bob".to_string()), 51);

		assert_eq!(
			balances.transfer("alice".to_string(), "bob".to_string(), 51),
			Err("Not enough funds.")
		);
	}
}