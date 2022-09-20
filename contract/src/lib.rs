/*
 * Test smart contract for checking near calculation limits
 *
 */

use std::iter;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{
    collections::UnorderedMap,
    env,
    json_types::{U128, U64},
    log, near_bindgen, AccountId, BorshStorageKey, Timestamp,
};

const EPOCH: u128 = 30 * 12;
const ONE_DAY_IN_MS: u64 = 1000 * 60 * 60 * 24;
const TWO_YEARS_IN_MS: u64 = ONE_DAY_IN_MS * 365 * 2;

#[derive(Debug, BorshStorageKey, BorshDeserialize, BorshSerialize, PartialEq, Eq)]
pub enum StorageKey {
    Users,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
struct UserAccount {
    pub order: U128,
    pub withdraw_time: U64,
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub(crate) users: UnorderedMap<AccountId, UserAccount>,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            users: UnorderedMap::new(StorageKey::Users),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    pub(crate) fn add_user(&mut self, user_num: u64) {
        let user_account = AccountId::new_unchecked(user_num.to_string());
        let user_account_struct = UserAccount {
            order: U128::from(user_num as u128),
            withdraw_time: U64::from(env::block_timestamp_ms() + TWO_YEARS_IN_MS),
        };
        self.users.insert(&user_account, &user_account_struct);
    }

    pub fn add_user_accounts(&mut self, started_num: u64, number_to_add: u64) -> u64 {
        for user_num in started_num..started_num + number_to_add {
            self.add_user(user_num);
        }
        self.users.len()
    }

    pub fn get_user_order(&self, user_num: u64) -> u128 {
        let user_account = AccountId::new_unchecked(user_num.to_string());
        let user_account = self
            .users
            .get(&user_account)
            .expect("User account don't found");
        for (_, user_account) in self.users.iter() {
            log!("{:?}", user_account);
        }
        user_account.order.into()
    }

    pub(crate) fn get_users_num(&self) -> u64 {
        self.users.len()
    }

    /// Public method - accepts a number of ve_orders to calculate,
    /// returns calculated ve_order
    pub fn calc_ve_order_sum(&self) -> u128 {
        log!("calculating {} ve_orders", self.users.len());
        let mut ve_order_sum: u128 = 0;
        let cur_time = env::block_timestamp_ms();
        for (_, user_account) in &self.users {
            // let remaining_days = (user_account.withdraw_time - cur_time) / ONE_DAY_IN_MS;
            // let order_amount: u128 = user_account.order.into();
            // ve_order_sum += order_amount * remaining_days as u128 / EPOCH;
        }

        log!("ve_order_sum = {}", ve_order_sum);
        ve_order_sum
    }

    /// Public method - accepts a number of ve_orders to calculate,
    /// returns calculated ve_order
    pub fn calc_ve_order_sum_simple(&self, num: i32) -> u128 {
        log!("calculating {} ve_orders", num);
        let order: u128 = 1000;
        let mut ve_order_sum: u128 = 0;
        for remaining_days in 1000u128..1000u128 + num as u128 {
            ve_order_sum += order * remaining_days / EPOCH;
        }
        log!("ve_order_sum = {}", ve_order_sum);
        ve_order_sum
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_add_users() {
        let mut contract = Contract::default();
        contract.add_user(100);
        contract.add_user(100);
        contract.add_user_accounts(101, 100);
        assert_eq!(contract.get_users_num(), 101);
    }

    #[test]
    fn run_simple_calc() {
        let contract = Contract::default();
        let ve_order_sum = contract.calc_ve_order_sum_simple(50_000_000);
        println!("ve_order_sum = {}", ve_order_sum);
    }

    #[test]
    fn run_get_user_order() {
        let mut contract = Contract::default();
        contract.add_user_accounts(101, 10);
        let order = contract.get_user_order(101);
        println!("order = {}", order as u128);
    }

    #[test]
    fn run_calc() {
        let mut contract = Contract::default();
        contract.add_user_accounts(101, 10);
        let ve_order_sum = contract.calc_ve_order_sum();
        println!("ve_order_sum = {}", ve_order_sum);
    }
}
