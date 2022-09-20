use near_units::parse_near;
use serde_json::json;
use std::{env, fs};
use workspaces::prelude::*;
use workspaces::{network::Sandbox, Account, Contract, Worker};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let wasm_arg: &str = &(env::args().nth(1).unwrap());
    let wasm_filepath = fs::canonicalize(env::current_dir()?.join(wasm_arg))?;

    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(wasm_filepath)?;
    let contract = worker.dev_deploy(&wasm).await?;

    // create accounts
    let account = worker.dev_create_account().await?;
    let alice = account
        .create_subaccount(&worker, "alice")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    test_add_indexes(&alice, &contract, &worker, 0, 100).await?;
    test_get_index(&alice, &contract, &worker, 99).await?;
    test_get_index_sum(&alice, &contract, &worker).await?;

    // begin tests
    // test_calc_ve_order_sum_simple(&alice, &contract, &worker, 1_200_000).await?;

    // test_add_users(&alice, &contract, &worker, 0, 10).await?;
    // test_get_user_order(&alice, &contract, &worker, 9).await?;
    // test_calc_ve_order_sum(&alice, &contract, &worker).await?;

    // Add a lot of users
    // for users_num in 0..20 {
    //     test_add_users(&alice, &contract, &worker, users_num * 500, 500).await?;
    // }
    Ok(())
}

//#################################################################
async fn test_add_users(
    user: &Account,
    contract: &Contract,
    worker: &Worker<Sandbox>,
    started_num: u64,
    number_to_add: u64,
) -> anyhow::Result<()> {
    let users_num: u128 = user
        .call(&worker, contract.id(), "add_user_accounts")
        .args_json(json!({ "started_num": started_num, "number_to_add": number_to_add }))?
        .gas(300_000_000_000_000)
        .transact()
        .await?
        .json()?;

    // assert_eq!(message, "Howdy".to_string());
    println!("      users_num = {}", users_num);
    Ok(())
}

async fn test_get_index(
    user: &Account,
    contract: &Contract,
    worker: &Worker<Sandbox>,
    num: u64,
) -> anyhow::Result<()> {
    let test_index: u128 = user
        .call(&worker, contract.id(), "get_index")
        .args_json(json!({ "ind_num": num }))?
        .gas(300_000_000_000_000)
        .transact()
        .await?
        .json()?;

    // assert_eq!(message, "Howdy".to_string());
    println!("      test_index = {}", test_index);
    println!("      Passed ✅ test_get_inex");
    Ok(())
}

async fn test_get_index_sum(
    user: &Account,
    contract: &Contract,
    worker: &Worker<Sandbox>,
) -> anyhow::Result<()> {
    let index_sum: u128 = user
        .call(&worker, contract.id(), "get_index_sum")
        .gas(300_000_000_000_000)
        .transact()
        .await?
        .json()?;

    // assert_eq!(message, "Howdy".to_string());
    println!("      index_sum = {}", index_sum);
    println!("      Passed ✅ test_get_index_sum");
    Ok(())
}
//#################################################################

async fn test_add_indexes(
    user: &Account,
    contract: &Contract,
    worker: &Worker<Sandbox>,
    started_num: u64,
    number_to_add: u64,
) -> anyhow::Result<()> {
    let users_num: u128 = user
        .call(&worker, contract.id(), "add_indexes")
        .args_json(json!({ "started_num": started_num, "number_to_add": number_to_add }))?
        .gas(300_000_000_000_000)
        .transact()
        .await?
        .json()?;

    // assert_eq!(message, "Howdy".to_string());
    println!("      indexes_num = {}", users_num);
    Ok(())
}

async fn test_get_user_order(
    user: &Account,
    contract: &Contract,
    worker: &Worker<Sandbox>,
    num: u64,
) -> anyhow::Result<()> {
    let order: u128 = user
        .call(&worker, contract.id(), "get_user_order")
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
    contract: &Contract,
    worker: &Worker<Sandbox>,
) -> anyhow::Result<()> {
    let ve_order_sum: u128 = user
        .call(&worker, contract.id(), "calc_ve_order_sum")
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
    contract: &Contract,
    worker: &Worker<Sandbox>,
    num: i32,
) -> anyhow::Result<()> {
    let ve_order_sum: u128 = user
        .call(&worker, contract.id(), "calc_ve_order_sum_simple")
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
