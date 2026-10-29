//! Demonstration balance ledger. No assets are transferred or held by this contract.
#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
pub enum DataKey {
    Owner,
    Balance(Address),
    Paused,
}

#[contract]
pub struct VaultContract;

#[contractimpl]
impl VaultContract {
    pub fn initialize(env: Env, owner: Address) {
        assert!(
            !env.storage().instance().has(&DataKey::Owner),
            "already initialized"
        );
        owner.require_auth();
        env.storage().instance().set(&DataKey::Owner, &owner);
        env.storage().instance().set(&DataKey::Paused, &false);
    }

    /// Owner can pause deposits; withdrawals remain available.
    pub fn set_paused(env: Env, paused: bool) {
        let owner = Self::owner(&env);
        owner.require_auth();
        env.storage().instance().set(&DataKey::Paused, &paused);
    }

    /// Record a demonstration credit, without transferring tokens.
    pub fn deposit(env: Env, depositor: Address, amount: i128) {
        Self::owner(&env);
        depositor.require_auth();
        let paused: bool = env
            .storage()
            .instance()
            .get(&DataKey::Paused)
            .unwrap_or(false);
        assert!(!paused, "deposits paused");
        assert!(amount > 0, "amount must be positive");
        let current = Self::balance(env.clone(), depositor.clone());
        let updated = current.checked_add(amount).expect("balance overflow");
        env.storage()
            .persistent()
            .set(&DataKey::Balance(depositor), &updated);
    }

    pub fn withdraw(env: Env, depositor: Address, amount: i128) {
        Self::owner(&env);
        depositor.require_auth();
        assert!(amount > 0, "amount must be positive");
        let current = Self::balance(env.clone(), depositor.clone());
        assert!(current >= amount, "insufficient balance");
        env.storage()
            .persistent()
            .set(&DataKey::Balance(depositor), &(current - amount));
    }

    pub fn balance(env: Env, account: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(account))
            .unwrap_or(0)
    }

    fn owner(env: &Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::Owner)
            .expect("not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn pause_preserves_withdrawals_and_failed_calls_preserve_balances() {
        let env = Env::default();
        env.mock_all_auths();
        let id = env.register(VaultContract, ());
        let c = VaultContractClient::new(&env, &id);
        let owner = Address::generate(&env);
        let user = Address::generate(&env);
        assert!(c.try_deposit(&user, &1).is_err());
        c.initialize(&owner);
        assert!(c.try_initialize(&user).is_err());
        c.deposit(&user, &i128::MAX);
        assert!(c.try_deposit(&user, &1).is_err());
        assert_eq!(c.balance(&user), i128::MAX);
        c.set_paused(&true);
        assert!(c.try_deposit(&user, &1).is_err());
        c.withdraw(&user, &i128::MAX);
        assert_eq!(c.balance(&user), 0);
        assert!(c.try_withdraw(&user, &1).is_err());
        assert!(c.try_withdraw(&user, &0).is_err());
        assert!(c.try_deposit(&user, &-1).is_err());
        c.set_paused(&false);
        c.deposit(&user, &10);
        assert_eq!(c.balance(&user), 10);
        assert_eq!(c.balance(&owner), 0);
    }

    #[test]
    fn mutations_require_authorization() {
        let env = Env::default();
        let id = env.register(VaultContract, ());
        let c = VaultContractClient::new(&env, &id);
        let owner = Address::generate(&env);
        let user = Address::generate(&env);
        assert!(c.try_initialize(&owner).is_err());
        c.mock_all_auths().initialize(&owner);
        assert!(c.try_set_paused(&true).is_err());
        assert!(c.try_deposit(&user, &1).is_err());
        c.mock_all_auths().deposit(&user, &10);
        assert!(c.try_withdraw(&user, &1).is_err());
        assert_eq!(c.balance(&user), 10);
    }
}

#[cfg(test)]
mod owner_authorization_tests {
    use super::*;
    use soroban_sdk::{
        testutils::{Address as _, MockAuth, MockAuthInvoke},
        IntoVal,
    };

    #[test]
    fn another_users_authorization_cannot_pause_deposits() {
        let env = Env::default();
        let id = env.register(VaultContract, ());
        let c = VaultContractClient::new(&env, &id);
        let owner = Address::generate(&env);
        let other = Address::generate(&env);
        c.mock_all_auths().initialize(&owner);
        assert!(c
            .mock_auths(&[MockAuth {
                address: &other,
                invoke: &MockAuthInvoke {
                    contract: &id,
                    fn_name: "set_paused",
                    args: (true,).into_val(&env),
                    sub_invokes: &[],
                },
            }])
            .try_set_paused(&true)
            .is_err());
        c.mock_all_auths().deposit(&other, &1);
        assert_eq!(c.balance(&other), 1);
    }
}
