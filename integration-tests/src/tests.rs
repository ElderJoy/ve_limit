use near_units::parse_near;
use rand::{distributions::Alphanumeric, Rng};
use serde_json::json;
use std::{env, fs};
use workspaces::prelude::*;
use workspaces::{network::Sandbox, Account, Contract, Worker};

fn generate_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let wasm_arg: &str = &(env::args().nth(1).unwrap());
    let wasm_filepath = fs::canonicalize(env::current_dir()?.join(wasm_arg))?;

    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(wasm_filepath)?;
    let order_stake = worker.dev_deploy(&wasm).await?;

    // create accounts
    let account = worker.dev_create_account().await?;
    let alice = account
        .create_subaccount(&worker, "alice")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    // begin tests
    // test_calc_ve_order_sum_simple(&alice, &order_stake, &worker, 1_200_000).await?;

    // test_add_users(&alice, &order_stake, &worker, 10, 10).await?;
    // test_get_user_order(&alice, &order_stake, &worker, 19).await?;
    // test_calc_ve_order_sum(&alice, &order_stake, &worker).await?;

    // Add a lot of users
    for users_num in 1..3 {
        test_add_users(&alice, &order_stake, &worker, users_num * 500, 500).await?;
    }
    test_calc_ve_order_sum(&alice, &order_stake, &worker).await?;

    Ok(())
}

async fn test_add_users(
    user: &Account,
    order_stake: &Contract,
    worker: &Worker<Sandbox>,
    started_num: u64,
    number_to_add: u64,
) -> anyhow::Result<()> {
    let rnd_str = generate_string(63);
    let users_num: u128 = user
        .call(&worker, order_stake.id(), "add_user_accounts")
        .args_json(
            json!({ "started_num": started_num, "number_to_add": number_to_add, "rnd_str":  &rnd_str}),
        )?
        .gas(300_000_000_000_000)
        .transact()
        .await?
        .json()?;

    // assert_eq!(message, "Howdy".to_string());
    println!("      users_num = {}", users_num);
    Ok(())
}

async fn test_get_user_order(
    user: &Account,
    order_stake: &Contract,
    worker: &Worker<Sandbox>,
    num: u64,
) -> anyhow::Result<()> {
    let order: u128 = user
        .call(&worker, order_stake.id(), "get_user_order")
        .args_json(json!({ "user_num": num }))?
        .gas(300_000_000_000_000)
        .transact()
        .await?
        .json()?;

    // assert_eq!(message, "Howdy".to_string());
    println!("      order = {}", order);
    println!("      Passed ✅ test_get_user_order");
    Ok(())
}

async fn test_calc_ve_order_sum(
    user: &Account,
    order_stake: &Contract,
    worker: &Worker<Sandbox>,
) -> anyhow::Result<()> {
    let ve_order_sum: u128 = user
        .call(&worker, order_stake.id(), "calc_ve_order_sum")
        .gas(300_000_000_000_000)
        .transact()
        .await?
        .json()?;

    // assert_eq!(message, "Howdy".to_string());
    println!("      ve_order_sum = {}", ve_order_sum);
    println!("      Passed ✅ test_calc_ve_order_sum");
    Ok(())
}

async fn test_calc_ve_order_sum_simple(
    user: &Account,
    order_stake: &Contract,
    worker: &Worker<Sandbox>,
    num: i32,
) -> anyhow::Result<()> {
    let ve_order_sum: u128 = user
        .call(&worker, order_stake.id(), "calc_ve_order_sum_simple")
        .args_json(json!({ "num": num }))?
        .gas(300_000_000_000_000)
        .transact()
        .await?
        .json()?;

    // assert_eq!(message, "Howdy".to_string());
    println!("      ve_order_sum = {}", ve_order_sum);
    println!("      Passed ✅ test_calc_ve_order_sum_simple");
    Ok(())
}
