//! Example Soroban token contract used as a fuzzing/verification target.
//! Implements a minimal fungible token with balance conservation invariants.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

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
        assert!(
            !env.storage().instance().has(&DataKey::Admin),
            "already initialized"
        );
        assert!(total_supply > 0, "supply must be positive");
        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::TotalSupply, &total_supply);
        env.storage()
            .persistent()
            .set(&DataKey::Balance(admin), &total_supply);
    }

    /// Transfer `amount` from `from` to `to`.
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        assert!(amount > 0, "amount must be positive");

        let from_balance = Self::balance(env.clone(), from.clone());
        assert!(from_balance >= amount, "insufficient balance");

        if from == to {
            return;
        }
        let to_balance = Self::balance(env.clone(), to.clone());
        let updated_to = to_balance.checked_add(amount).expect("balance overflow");

        env.storage()
            .persistent()
            .set(&DataKey::Balance(from), &(from_balance - amount));
        env.storage()
            .persistent()
            .set(&DataKey::Balance(to), &updated_to);
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

        let contract_id = env.register(TokenContract, ());
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

#[cfg(test)]
mod security_tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn self_transfer_and_sequences_conserve_supply() {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(TokenContract, ());
        let c = TokenContractClient::new(&env, &id);
        let a = Address::generate(&env);
        let b = Address::generate(&env);
        c.initialize(&a, &1000);
        c.transfer(&a, &a, &1000);
        assert_eq!(c.balance(&a), 1000);
        for amount in 1..100 {
            c.transfer(&a, &b, &amount);
            assert_eq!(c.balance(&a) + c.balance(&b), c.total_supply());
            c.transfer(&b, &a, &amount);
            assert_eq!(c.balance(&a), 1000);
        }
        assert!(c.try_transfer(&a, &b, &1001).is_err());
        assert!(c.try_transfer(&a, &b, &0).is_err());
        assert!(c.try_transfer(&a, &b, &-1).is_err());
        assert!(c.try_initialize(&b, &1000).is_err());
        assert_eq!(c.balance(&a), 1000);
        assert_eq!(c.balance(&b), 0);
    }

    #[test]
    fn initialization_and_transfer_require_authorization() {
        let env = Env::default();
        let id = env.register(TokenContract, ());
        let c = TokenContractClient::new(&env, &id);
        let a = Address::generate(&env);
        let b = Address::generate(&env);
        assert!(c.try_initialize(&a, &1000).is_err());
        c.mock_all_auths().initialize(&a, &1000);
        assert!(c.try_transfer(&a, &b, &1).is_err());
        assert_eq!(c.balance(&a), 1000);
    }
}
