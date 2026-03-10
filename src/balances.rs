use std::collections::BTreeMap;

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