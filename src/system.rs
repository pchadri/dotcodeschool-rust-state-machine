use num::traits::{CheckedAdd, One, Zero};
/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
use std::{collections::BTreeMap, ops::AddAssign};

//type AccountID = String; moved to main.rs because of generics
//type BlockNumber = u32; moved to main.rs because of generics
//type Nonce = u32; moved to main.rs because of generics

pub trait Config {
	type BlockNumber: CheckedAdd + Zero + Copy + One + AddAssign;
	type AccountID: Ord + Clone;
	type Nonce: CheckedAdd + Zero + Copy + One + AddAssign + Clone;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// The current block number.
	block_number: T::BlockNumber,
	/// A map from an account to their nonce.
	nonce: BTreeMap<T::AccountID, T::Nonce>,
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> T::BlockNumber {
		/* TODO: Return the current block number. */
		self.block_number
	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		/* TODO: Increment the current block number by one. */
		self.block_number += T::BlockNumber::one();
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &T::AccountID) {
		/* TODO: Get the current nonce of `who`, and increment it by one. */
		let current_nonce = self.nonce.get(who).cloned().unwrap_or(T::Nonce::zero());
		self.nonce.insert(who.clone(), current_nonce + T::Nonce::one());
	}
}

#[cfg(test)]
mod test {
	use super::*;

	//A localized implementation of the Config trait to test that our pallet works internally as expected.
	struct TestConfig;
	impl Config for TestConfig {
		type BlockNumber = u32;
		type AccountID = String;
		type Nonce = u32;
	}
	#[test]
	fn localized_init_system() {
		/* TODO: Create a test which checks the following:
			- Increment the current block number.
			- Increment the nonce of `alice`.

			- Check the block number is what we expect.
			- Check the nonce of `alice` is what we expect.
			- Check the nonce of `bob` is what we expect.
		*/

		// Create a new instance of the System Pallet
		let mut system = Pallet::<TestConfig>::new();

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

	#[test]
	//Test of the system pallet using the main.rs runtime as an integrated test
	fn integrated_init_system() {
		/* TODO: Create a test which checks the following:
			- Increment the current block number.
			- Increment the nonce of `alice`.

			- Check the block number is what we expect.
			- Check the nonce of `alice` is what we expect.
			- Check the nonce of `bob` is what we expect.
		*/

		// Create a new instance of the System Pallet
		let mut system = Pallet::<crate::Runtime>::new();

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
