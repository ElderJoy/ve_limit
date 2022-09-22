/*
 * Test smart contract for checking near calculation limits
 *
 */

use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedMap,
    env, log, near_bindgen, require, AccountId, BlockHeight, BorshStorageKey, Timestamp,
};

const ONE_DAY_IN_MS: u64 = 1000 * 60 * 60 * 24;
const TWO_YEARS_IN_MS: u64 = ONE_DAY_IN_MS * 365 * 2;
const EPOCH: u64 = 30; // epoch in days
const EPOCH_IN_MS: u64 = ONE_DAY_IN_MS * EPOCH;
const TWELWE_EPOCHS: u64 = EPOCH * 12;
const MAX_EPOCH_NUM: u8 = 12 * 4;

#[derive(Debug, BorshStorageKey, BorshDeserialize, BorshSerialize, PartialEq, Eq)]
pub enum StorageKey {
    Users,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, PartialEq, Eq)]
struct Point {
    pub bias: i128,
    pub slope: i128, // # -dweight / dt
    pub ts: Timestamp,
    pub blk: BlockHeight,
}

#[derive(Debug, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
struct LockedBalance {
    pub amount: u128,
    pub end: Timestamp,
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub(crate) start_time: Timestamp,
    pub(crate) users: UnorderedMap<AccountId, LockedBalance>,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            start_time: env::block_timestamp_ms(),
            users: UnorderedMap::new(StorageKey::Users),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    pub fn get_unlock_time(&self, from: Timestamp, num_epoch: u8) -> Timestamp {
        require!(num_epoch > 0, "Number of epochs should be more than zero");
        require!(
            num_epoch < MAX_EPOCH_NUM,
            format!("Number of epochs should be no more than {}", MAX_EPOCH_NUM)
        );

        let num_epoch_from_start = (env::block_timestamp_ms() - self.start_time) / EPOCH_IN_MS;
        let next_epoch_ts = self.start_time + num_epoch_from_start * EPOCH_IN_MS + EPOCH_IN_MS;
        next_epoch_ts + num_epoch as u64 * EPOCH_IN_MS
    }

    pub(crate) fn add_user(&mut self, user_num: u64, user_account: &AccountId) {
        let user_account_struct = LockedBalance {
            amount: user_num as u128,
            end: env::block_timestamp_ms() + TWO_YEARS_IN_MS,
        };
        self.users.insert(&user_account, &user_account_struct);
    }

    pub fn add_user_accounts(
        &mut self,
        started_num: u64,
        number_to_add: u64,
        rnd_str: &String,
    ) -> u64 {
        for user_num in started_num..started_num + number_to_add {
            let account_str = format!("{:.64}", format!("{}{}", user_num, rnd_str)).to_lowercase();
            let user_account = AccountId::try_from(account_str).unwrap();
            self.add_user(user_num, &user_account);
        }
        self.users.len()
    }

    pub fn get_user_order(&self, user_num: u64) -> u128 {
        let user_account = AccountId::new_unchecked(user_num.to_string());
        let user_account = self
            .users
            .get(&user_account)
            .expect("User account don't found");
        user_account.amount
    }

    pub fn get_users_num(&self) -> u64 {
        self.users.len()
    }

    /// Public method - accepts a number of ve_orders to calculate,
    /// returns calculated ve_order
    pub fn calc_ve_order_sum(&self) -> u128 {
        log!("calculating {} ve_orders", self.users.len());
        let mut ve_order_sum: u128 = 0;
        let cur_time = env::block_timestamp_ms();
        for (_, user_account) in &self.users {
            let remaining_days = (user_account.end - cur_time) / ONE_DAY_IN_MS;
            let order_amount: u128 = user_account.amount.into();
            ve_order_sum += order_amount * remaining_days as u128 / TWELWE_EPOCHS as u128;
        }

        log!("ve_order_sum = {}", ve_order_sum);
        ve_order_sum
    }

    /// Public method - accepts a number of ve_orders to calculate,
    /// returns calculated ve_order
    pub fn calc_ve_order_sum_simple(&self, num: i32) -> u128 {
        log!("calculating {} ve_orders", num);
        let amount: u128 = 1000;
        let mut ve_order_sum: u128 = 0;
        for remaining_days in 1000u128..1000u128 + num as u128 {
            ve_order_sum += amount * remaining_days / TWELWE_EPOCHS as u128;
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
    use rand::distributions::Uniform;
    use rand::{distributions::Alphanumeric, Rng};

    use super::*;

    fn generate_string(len: usize) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }

    #[test]
    fn run_add_users() {
        let mut contract = Contract::default();
        let rnd_str = generate_string(63);
        contract.add_user_accounts(101, 100, &rnd_str);
        assert_eq!(contract.get_users_num(), 100);
    }

    #[test]
    fn run_simple_calc() {
        let contract = Contract::default();
        let ve_order_sum = contract.calc_ve_order_sum_simple(50_000_000);
        println!("ve_order_sum = {}", ve_order_sum);
    }

    // #[test]
    // fn run_get_user_order() {
    //     let mut contract = Contract::default();
    //     let rnd_str = generate_string(63);
    //     contract.add_user_accounts(101, 10, &rnd_str);
    //     let amount = contract.get_user_order(101);
    //     println!("amount = {}", amount as u128);
    // }

    #[test]
    fn run_calc() {
        let mut contract = Contract::default();
        let rnd_str = generate_string(63);
        contract.add_user_accounts(101, 10, &rnd_str);
        let ve_order_sum = contract.calc_ve_order_sum();
        println!("ve_order_sum = {}", ve_order_sum);
    }

    #[test]
    fn run_rand_string() {
        let s = generate_string(70);
        println!("{}", s);
    }

    #[test]
    fn run_generate_account_strings() {
        let rnd_str = generate_string(63);
        println!("{}", format!("{:.64}", format!("{}{}", 0, rnd_str)));
        println!("{}", format!("{:.64}", format!("{}{}", 2, rnd_str)));
        println!("{}", format!("{:.64}", format!("{}{}", 10, rnd_str)));
        println!("{}", format!("{:.64}", format!("{}{}", 10000, rnd_str)));
        let s = "0vr6Ygf7dHoMA8Ch5o0BmkhI42N4QtnIeLf8O4pHOQjF9Pwj27IGSRZQe4RL7JQq"
            .to_owned()
            .to_lowercase();
        println!("{}", s);
        let acc = AccountId::try_from(s).unwrap();
    }

    #[test]
    fn run_get_unlock_time() {
        let contract = Contract::default();
        let one_epoch = contract.get_unlock_time(env::block_timestamp_ms(), 1);
    }
}
