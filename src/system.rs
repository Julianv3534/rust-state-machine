use std::collections::BTreeMap;
use num::traits::{CheckedAdd, Zero};

type AccountId = String;
type BlockNumber = u32;
type Nonce = u32;

#[derive(Debug)]
pub struct Pallet<AccountId, BlockNumber, Nonce> {
    block_number: BlockNumber,
    nonce: BTreeMap<AccountId, Nonce>,
}

impl<AccountId, BlockNumber, Nonce> Pallet<AccountId, BlockNumber, Nonce>
where
    AccountId: Ord,
    BlockNumber: Zero + CheckedAdd + From<u8> + Copy,
    Nonce: Zero + CheckedAdd + From<u8> + Copy,
{
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
        Self {block_number: BlockNumber::zero(), nonce: BTreeMap::new()}
	}

    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    //not using safe math, because assuming a user does one transaction every block,
    //and a new block is generated every 6 seconds,
    //it would take over 800 years for an overflow to occur.

    pub fn inc_block_number(&mut self) {
        self.block_number = self
            .block_number
            .checked_add(&BlockNumber::from(1u8))
            .expect("Block number overflow");
    }

    pub fn account_nonce(&self, account: &AccountId) -> Nonce {
        *self.nonce.get(account).unwrap_or(&Nonce::zero())
    }

    pub fn inc_account_nonce(&mut self, account: AccountId) {
        let nonce = self.nonce.entry(account).or_insert(Nonce::zero());
        *nonce = nonce
            .checked_add(&Nonce::from(1u8))
            .expect("Nonce overflow");
    }
}

#[cfg(test)]
mod test {
	use super::{AccountId, BlockNumber, Nonce, Pallet};

	#[test]
	fn init_system() {
		let mut system = Pallet::<AccountId, BlockNumber, Nonce>::new();
		system.inc_block_number();
		system.inc_account_nonce("alice".to_string());

		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce.get("alice"), Some(&1));
		assert_eq!(system.nonce.get("bob"), None);
	}
}