// This is our main Runtime.
// It accumulates all of the different pallets we want to use.

// Import the types we defined in the types module below so that we can use them without prefixing them with `crate::types::`.
use crate::types::{AccountID, Balance, BlockNumber, Nonce};

//import the pallets so we can use them in our runtime.
mod balances;
mod system;
mod support;

// Define the concrete	types we'll use in our runtime.
mod types {
	// Define some common types we'll use in our pallets and runtime.
	pub type AccountID = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
	pub type Extrinsic = crate::support::Extrinsic<AccountID, crate::RuntimeCall>;
	pub type Header = crate::support::Header<BlockNumber>;
	pub type Block = crate::support::Block<Header, Extrinsic>;
}

pub enum RuntimeCall {
				// TODO: Not implemented yet.	
}

#[derive(Debug)]
//	The main runtime struct which holds instances of each pallet. 
//<Self> is used to refer to the runtime itself when passing it as a generic parameter to each pallet.
// It means each pallet will refer to the	runtime's implementation of the Config trait to get its associated types.
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
}

// Implement the config traits defined in each pallet for our runtime, specifically, the associated types.
//	Here we link the associated types for each config to the concrete values we gave in our mod types that we defined at the beginning then imported using crate.
impl system::Config for Runtime {
	type BlockNumber = BlockNumber;
	type AccountID = AccountID;
	type Nonce = Nonce;
}

//Because	balances::Config extends system::Config, we only need to implement the additional associated type Balance here.
impl balances::Config for Runtime {
	type Balance = Balance;
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		Self { system: system::Pallet::<Self>::new(), balances: balances::Pallet::<Self>::new() }
	}

	// Execute a block of extrinsics. Increments the block number.
	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		/* TODO:
			- Increment the system's block number.
			- Check that the block number of the incoming block matches the current block number,
			  or return an error.
			- Iterate over the extrinsics in the block...
				- Increment the nonce of the caller.
				- Dispatch the extrinsic using the `caller` and the `call` contained in the extrinsic.
				- Handle errors from `dispatch` same as we did for individual calls: printing any
				  error and capturing the result.
				- You can extend the error message to include information like the block number and
				  extrinsic number.
		*/
		Ok(())
	}
}

impl crate::support::Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::AccountId;
	type Call = RuntimeCall;
	// Dispatch a call on behalf of a caller. Increments the caller's nonce.
	//
	// Dispatch allows us to identify which underlying module call we want to execute.
	// Note that we extract the `caller` from the extrinsic, and use that information
	// to determine who we are executing the call on behalf of.
	fn dispatch(
		&mut self,
		caller: Self::Caller,
		runtime_call: Self::Call,
	) -> support::DispatchResult {
		unimplemented!();
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
