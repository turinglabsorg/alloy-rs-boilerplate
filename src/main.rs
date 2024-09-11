#![allow(unused_imports)]
use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    sol,
};
use dotenv::dotenv;
use eyre::Result;
use std::env;
mod contract;
mod tx;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IWETH9,
    "abi/IWETH9.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    // Run the tx example
    crate::tx::tx().await?;
    // Run the contract example
    crate::contract::contract().await?;
    Ok(())
}
