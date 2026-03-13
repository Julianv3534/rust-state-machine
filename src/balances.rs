use std::collections::BTreeMap;
use num::traits::{CheckedAdd, CheckedSub, Zero};

#[derive(Debug)]
pub struct Pallet<AccountId, Balance> {
	//Why BTreeMap and not HashMap?
	balances: BTreeMap<AccountId, Balance>,
}

impl<AccountId, Balance> Pallet<AccountId, Balance> 
	where
		AccountId: Ord + Clone,
		Balance: Zero + CheckedSub + CheckedAdd + Copy,
	{
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	// can be done with &str and who.to_string() instead of String and clone, but for simplicity we
	// will use String.
	pub fn set_balance(&mut self, who: &String, amount: u128) {
		self.balances.insert(who.clone(), amount);
	}

	pub fn balance(&self, who: &String) -> u128 {
		*self.balances.get(who).unwrap_or(&0)
	}

	pub fn transfer(
		&mut self,
		caller: String,
		to: String,
		amount: u128,
	) -> Result<(), &'static str> {
		let from_balance = self.balance(&caller);
		let new_from_balance = from_balance
    	.checked_sub(amount)
    	.ok_or("Not enough funds.")?;

		// Update the balance of the caller
		self.set_balance(&caller, new_from_balance);

		// Update the balance of the recipient
		let to_balance = self.balance(&to);
		let new_to_balance = to_balance
		.checked_add(amount)
		.ok_or("Balance overflow.")?;
		self.set_balance(&to, new_to_balance);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn init_balances() {
		let mut balances = super::Pallet::<String, u128>::new();

		assert_eq!(balances.balance(&"alice".to_string()), 0);
		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}

	#[test]
	fn transfer_balance() {
		let mut balances = super::Pallet::<String, u128>::new();

		assert_eq!(
			balances.transfer("alice".to_string(), "bob".to_string(), 51),
			Err("Not enough funds.")
		);

		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.transfer("alice".to_string(), "bob".to_string(), 51), Ok(()));
		assert_eq!(balances.balance(&"alice".to_string()), 49);
		assert_eq!(balances.balance(&"bob".to_string()), 51);

		assert_eq!(
			balances.transfer("alice".to_string(), "bob".to_string(), 51),
			Err("Not enough funds.")
		);
	}
}