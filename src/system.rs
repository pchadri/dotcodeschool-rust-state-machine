use num::traits::{CheckedAdd, One, Zero};
/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
use std::{collections::BTreeMap, ops::AddAssign};

//type AccountID = String; moved to main.rs because of generics
//type BlockNumber = u32; moved to main.rs because of generics
//type Nonce = u32; moved to main.rs because of generics


#[derive(Debug)]
pub struct Pallet<BlockNumber, AccountID, Nonce> {
	/// The current block number.
	block_number: BlockNumber,
	/// A map from an account to their nonce.
	nonce: BTreeMap<AccountID, Nonce>,
}

impl<BlockNumber, AccountID, Nonce> Pallet<BlockNumber, AccountID, Nonce>
where
	BlockNumber: CheckedAdd + Zero + Copy + One + AddAssign,
	AccountID: Ord + Clone,
	Nonce: CheckedAdd + Zero + Copy + One + AddAssign + Clone,
{
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self { block_number: BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> BlockNumber {
		/* TODO: Return the current block number. */
		self.block_number
	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		/* TODO: Increment the current block number by one. */
		self.block_number += BlockNumber::one();
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &AccountID) {
		/* TODO: Get the current nonce of `who`, and increment it by one. */
		let current_nonce = self.nonce.get(who).cloned().unwrap_or(Nonce::zero());
		self.nonce.insert(who.clone(), current_nonce + Nonce::one());
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn init_system() {
		/* TODO: Create a test which checks the following:
			- Increment the current block number.
			- Increment the nonce of `alice`.

			- Check the block number is what we expect.
			- Check the nonce of `alice` is what we expect.
			- Check the nonce of `bob` is what we expect.
		*/

		// Create a new instance of the System Pallet
		let mut system = super::Pallet::<u32, String, u32>::new();

		//check initial block number is 0
		assert_eq!(system.block_number(), 0);
		// Check that incrementing the current block number works
		system.inc_block_number();
		assert_eq!(system.block_number(), 1);

		// create alice and bob accounts
		let alice = String::from("alice");
		let bob = String::from("bob");
		// Check that incrementing the nonce of alice works
		assert_eq!(system.nonce.get(&alice), None);
		system.inc_nonce(&alice);
		assert_eq!(system.nonce.get(&alice), Some(&1));
		// Check that the nonce of bob is still 0
		assert_eq!(system.nonce.get(&bob), None);
	}
}
