use core::ops::AddAssign;
use num::traits::{One, Zero};
use std::collections::BTreeMap;

type AccountId = String;
type BlockNumber = u32;
type Nonce = u32;

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + One + AddAssign + Copy;
	type Nonce: One + AddAssign + Default;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
        Self {block_number: T::BlockNumber::zero(), nonce: BTreeMap::new()}
	}

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    //not using safe math, because assuming a user does one transaction every block,
    //and a new block is generated every 6 seconds,
    //it would take over 800 years for an overflow to occur.

    pub fn inc_block_number(&mut self) {
		self.block_number += T::BlockNumber::one();
	}

    pub fn inc_nonce(&mut self, who: &T::AccountId) {
		*self.nonce.entry(who.clone()).or_default() += T::Nonce::one();
	}
}

#[cfg(test)]
mod test {
    struct TestConfig;
	impl super::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn init_system() {
		let mut system = super::Pallet::<TestConfig>::new();
		system.inc_block_number();
		system.inc_nonce(&"alice".to_string());

		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce.get("alice"), Some(&1));
		assert_eq!(system.nonce.get("bob"), None);
	}
}