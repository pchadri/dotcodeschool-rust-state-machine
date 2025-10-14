use crate::types::{AccountID, Balance, BlockNumber, Nonce};

mod balances;
mod system;

mod types {
	// Define some common types we'll use in our pallets and runtime.
	pub type AccountID = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<types::BlockNumber, types::AccountID, types::Nonce>,
	balances: balances::Pallet<types::AccountID, types::Balance>,
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		Self {
			system: system::Pallet::<BlockNumber, AccountID, Nonce>::new(),
			balances: balances::Pallet::<AccountID, Balance>::new(),
		}
	}
}

fn main() {
	/* TODO: Create a mutable variable `runtime`, which is a new instance of `Runtime`. */
	let mut runtime = Runtime::new();
	let alice: AccountID = "alice".to_string();
	let bob: AccountID = "bob".to_string();
	let charlie: AccountID = "charlie".to_string();

	/* TODO: Set the balance of `alice` to 100, allowing us to execute other transactions. */
	runtime.balances.set_balance(&alice, 100);

	// start emulating a block
	/* TODO: Increment the block number in system. */
	runtime.system.inc_block_number();
	/* TODO: Assert the block number is what we expect. */
	assert_eq!(runtime.system.block_number(), 1);

	// first transaction
	/* TODO: Increment the nonce of `alice`. */
	runtime.system.inc_nonce(&alice);
	/* TODO: Execute a transfer from `alice` to `bob` for 30 tokens.
		- The transfer _could_ return an error. We should use `map_err` to print
		  the error if there is one.
		- We should capture the result of the transfer in an unused variable like `_res`.
	*/
	let mut _results: Vec<Result<(), ()>> = Vec::new();
	//runtime.balances.transfer("alice".to_string(), "bob".to_string(), 30).map_err(|e| println!("Transfer error: {}", e)).unwrap();

	_results.push(
		runtime
			.balances
			.transfer(&alice, &bob, 30)
			.map_err(|e| println!("Transfer error: {}", e)),
	);

	// second transaction
	/* TODO: Increment the nonce of `alice` again. */
	runtime.system.inc_nonce(&alice);
	/* TODO: Execute another balance transfer, this time from `alice` to `charlie` for 20. */
	_results.push(
		runtime
			.balances
			.transfer(&alice, &charlie, 20)
			.map_err(|e| println!("Transfer error: {}", e)),
	);
}
