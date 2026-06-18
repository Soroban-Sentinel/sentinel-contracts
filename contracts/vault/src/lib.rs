//! Example vault contract — demonstrates access-control and reentrancy-guard patterns.

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
pub enum DataKey {
    Owner,
    Balance(Address),
    Locked, // reentrancy guard
}

#[contract]
pub struct VaultContract;

#[contractimpl]
impl VaultContract {
    pub fn initialize(env: Env, owner: Address) {
        assert!(!env.storage().instance().has(&DataKey::Owner), "already initialized");
        env.storage().instance().set(&DataKey::Owner, &owner);
        env.storage().instance().set(&DataKey::Locked, &false);
    }

    /// Deposit funds into the vault.
    pub fn deposit(env: Env, depositor: Address, amount: i128) {
        depositor.require_auth();
        Self::assert_not_locked(&env);
        assert!(amount > 0, "amount must be positive");

        let current: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(depositor.clone()))
            .unwrap_or(0);
        env.storage()
            .persistent()
            .set(&DataKey::Balance(depositor), &(current + amount));
    }

    /// Withdraw funds — only the depositor may withdraw their own balance.
    pub fn withdraw(env: Env, depositor: Address, amount: i128) {
        depositor.require_auth();
        Self::set_lock(&env, true); // reentrancy guard: lock
        assert!(amount > 0, "amount must be positive");

        let current: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(depositor.clone()))
            .unwrap_or(0);
        assert!(current >= amount, "insufficient balance");

        env.storage()
            .persistent()
            .set(&DataKey::Balance(depositor), &(current - amount));
        Self::set_lock(&env, false); // release
    }

    /// Owner-only emergency drain — access-control invariant target.
    pub fn emergency_drain(env: Env, caller: Address) {
        caller.require_auth();
        let owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        assert!(caller == owner, "only owner");
        // drain logic omitted — placeholder for fuzzer targeting
    }

    pub fn balance(env: Env, account: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(account))
            .unwrap_or(0)
    }

    fn assert_not_locked(env: &Env) {
        let locked: bool = env
            .storage()
            .instance()
            .get(&DataKey::Locked)
            .unwrap_or(false);
        assert!(!locked, "reentrancy detected");
    }

    fn set_lock(env: &Env, value: bool) {
        env.storage().instance().set(&DataKey::Locked, &value);
    }
}
