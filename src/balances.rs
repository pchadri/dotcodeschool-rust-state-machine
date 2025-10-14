use num::traits::{CheckedAdd, CheckedSub, Zero};
/// This is the Balances Module.
/// It is a simple module which keeps track of how much balance each account has in this state
/// machine.
use std::collections::BTreeMap;

//type AccountID = String; moved to main.rs because of generics
//type Balance = u128; moved to main.rs because of generics

#[derive(Debug)]
pub struct Pallet<AccountID, Balance> {
	// A simple storage mapping from accounts (`String`) to their balances (`u128`).
	balances: BTreeMap<AccountID, Balance>,
}

impl<AccountID, Balance> Pallet<AccountID, Balance>
where
	AccountID: Ord + Clone,
	Balance: CheckedAdd + CheckedSub + Zero + Clone,
{
	/// Create a new instance of the balances module.
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	/// Set the balance of an account `who` to some `amount`.
	pub fn set_balance(&mut self, who: &AccountID, amount: Balance) {
		self.balances.insert(who.clone(), amount);
	}

	/// Get the balance of an account `who`.
	/// If the account has no stored balance, we return zero.
	pub fn balance(&self, who: &AccountID) -> Balance {
		self.balances.get(who).cloned().unwrap_or(Balance::zero())
	}

	/// Transfer `amount` from one account to another.
	/// This function verifies that `from` has at least `amount` balance to transfer,
	/// and that no mathematical overflows occur.
	pub fn transfer(
		&mut self,
		caller: &AccountID,
		to: &AccountID,
		amount: Balance,
	) -> Result<(), &'static str> {
		/* TODO:
			- Get the balance of account `caller`.
			- Get the balance of account `to`.

			- Use safe math to calculate a `new_caller_balance`.
			- Use safe math to calculate a `new_to_balance`.

			- Insert the new balance of `caller`.
			- Insert the new balance of `to`.
		*/

		// Get the balance of account `caller`.
		let current_caller_balance = self.balance(caller);
		// Get the balance of account `to`.
		let current_to_balance = self.balance(to);

		// Check if caller has enough balance to transfer
		let new_caller_balance = current_caller_balance
			.checked_sub(&amount)
			.ok_or("Insufficient balance for transfer")?;

		// Check for overflow when adding amount to recipient's balance
		let new_to_balance = current_to_balance
			.checked_add(&amount)
			.ok_or("Balance overflow for recipient")?;

		// Insert the new balance of `caller`.
		//self.balances.insert(caller.clone(), new_caller_balance);  Deprecated because of set_balance function
		self.set_balance(caller, new_caller_balance);
		// Insert the new balance of `to`.
		//self.balances.insert(to.clone(), new_to_balance); Deprecated because of set_balance function
		self.set_balance(to, new_to_balance);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn init_balances() {
		let mut balances = super::Pallet::<String, u128>::new();

		assert_eq!(balances.balance(&"alice".to_string()), 0);
		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}

	#[test]
	fn transfer_balance() {
		/* TODO: Create a test that checks the following:
			- That `alice` cannot transfer funds she does not have.
			- That `alice` can successfully transfer funds to `bob`.
			- That the balance of `alice` and `bob` is correctly updated.
		*/

		// Initialize the balances module
		let mut balances = super::Pallet::<String, u128>::new();

		// Set initial balance for Alice
		balances.set_balance(&"alice".to_string(), 100);

		// Ensure Bob starts with zero balance
		//assert_eq!(balances.balance(&"bob".to_string()), 0); Deprecated in favor of just setting balance to 0
		balances.set_balance(&"bob".to_string(), 0);

		// Attempt to transfer more than Alice's balance
		assert_eq!(
			balances.transfer("alice".to_string(), "bob".to_string(), 150),
			Err("Insufficient balance for transfer")
		);

		// Transfer a valid amount from Alice to Bob
		assert_eq!(balances.transfer("alice".to_string(), "bob".to_string(), 50), Ok(()));

		// Check final balances
		assert_eq!(balances.balance(&"alice".to_string()), 50);
		assert_eq!(balances.balance(&"bob".to_string()), 50);
	}
}
