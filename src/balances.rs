use std::collections::BTreeMap;
use num::traits::{CheckedAdd, CheckedSub, Zero};

type AccountId = String;
type Balance = u128;

pub trait Config {
	type AccountId: Ord + Clone;
	type Balance: Zero + CheckedAdd + CheckedSub + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	//Why BTreeMap and not HashMap?
	balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> 
	where
		T::AccountId: Ord,
		T::Balance: Zero + CheckedSub + CheckedAdd + Copy,
	{
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
	) -> Result<(), &'static str> {
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

#[cfg(test)]
mod tests {
	use super::{Config, Pallet};

	struct TestConfig;

	impl Config for TestConfig {
		type AccountId = String;
		type Balance = u128;
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