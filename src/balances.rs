use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet {
	//Why BTreeMap and not HashMap?
	balances: BTreeMap<String, u128>,
}

impl Pallet {
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
		// We use mute because we will be modifying the balances in the test, and we need to be able to call set_balance.
		let mut balances = super::Pallet::new();
		
    	assert_eq!(balances.balance(&"alice".to_string()), 0);
    	balances.set_balance(&"alice".to_string(), 100);
    	assert_eq!(balances.balance(&"alice".to_string()), 100);
    	assert_eq!(balances.balance(&"bob".to_string()), 0);
	}
}