//! Example Soroban token contract used as a fuzzing/verification target.
//! Implements a minimal fungible token with balance conservation invariants.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

#[contracttype]
pub enum DataKey {
    Balance(Address),
    TotalSupply,
    Admin,
}

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    /// Initialize the token with a total supply minted to `admin`.
    pub fn initialize(env: Env, admin: Address, total_supply: i128) {
        assert!(!env.storage().instance().has(&DataKey::Admin), "already initialized");
        assert!(total_supply > 0, "supply must be positive");

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TotalSupply, &total_supply);
        env.storage().persistent().set(&DataKey::Balance(admin), &total_supply);
    }

    /// Transfer `amount` from `from` to `to`.
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        assert!(amount > 0, "amount must be positive");

        let from_balance = Self::balance(env.clone(), from.clone());
        assert!(from_balance >= amount, "insufficient balance");

        let to_balance = Self::balance(env.clone(), to.clone());

        env.storage()
            .persistent()
            .set(&DataKey::Balance(from), &(from_balance - amount));
        env.storage()
            .persistent()
            .set(&DataKey::Balance(to), &(to_balance + amount));
    }

    /// Return the balance of `account`.
    pub fn balance(env: Env, account: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(account))
            .unwrap_or(0)
    }

    /// Return total supply (invariant: sum of all balances == total_supply).
    pub fn total_supply(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::Env;

    #[test]
    fn test_transfer_conserves_balance() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, TokenContract);
        let client = TokenContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let user = Address::generate(&env);

        client.initialize(&admin, &1_000_000);

        let supply_before = client.total_supply();
        client.transfer(&admin, &user, &250_000);

        assert_eq!(client.balance(&admin), 750_000);
        assert_eq!(client.balance(&user), 250_000);
        assert_eq!(client.total_supply(), supply_before); // conservation check
    }
}
