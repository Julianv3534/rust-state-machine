use std::collections::BTreeMap;
use num::traits::{CheckedAdd, Zero};

type AccountId = String;
type BlockNumber = u32;
type Nonce = u32;

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + CheckedAdd + From<u8> + Copy;
    type Nonce: Zero + CheckedAdd + From<u8> + Copy;
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
        self.block_number = self
            .block_number
            .checked_add(&T::BlockNumber::from(1u8))
            .expect("Block number overflow");
    }

    pub fn account_nonce(&self, account: &T::AccountId) -> T::Nonce {
        *self.nonce.get(account).unwrap_or(&T::Nonce::zero())
    }

    pub fn inc_account_nonce(&mut self, account: T::AccountId) {
        let nonce = self.nonce.entry(account).or_insert(T::Nonce::zero());
        *nonce = nonce
            .checked_add(&T::Nonce::from(1u8))
            .expect("Nonce overflow");
    }
}

#[cfg(test)]
mod test {
    use super::{Config, Pallet};

    struct TestConfig;

    impl Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

	#[test]
	fn init_system() {
        let mut system = Pallet::<TestConfig>::new();
		system.inc_block_number();
		system.inc_account_nonce("alice".to_string());

		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce.get("alice"), Some(&1));
		assert_eq!(system.nonce.get("bob"), None);
	}
}