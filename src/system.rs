use std::collections::BTreeMap;

pub struct Pallet {
    block_number: u32,
    nonce: BTreeMap<String, u32>,
}

impl Pallet {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
        Self {block_number: 0, nonce: BTreeMap::new()}
	}

    fn block_number(&self) -> u32 {
        self.block_number
    }

    //not using safe math, because assuming a user does one transaction every block,
    //and a new block is generated every 6 seconds,
    //it would take over 800 years for an overflow to occur.

    fn inc_block_number(&mut self) {
        self.block_number += 1;
    }

    fn account_nonce(&self, account: &str) -> u32 {
        *self.nonce.get(account).unwrap_or(&0)
    }

    fn inc_account_nonce(&mut self, account: &str) {
        let nonce = self.nonce.entry(account.to_string()).or_insert(0);
        *nonce += 1;
    }
}