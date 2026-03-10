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
}