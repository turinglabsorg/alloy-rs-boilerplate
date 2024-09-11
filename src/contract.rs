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

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    IWETH9,
    "abi/IWETH9.json"
);

#[allow(dead_code)]
pub async fn contract() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    // Set up the HTTP transport which is consumed by the RPC client.
    let rpc_url = "http://0.0.0.0:8545".parse()?;
    // Read private key from .env file
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set in .env file");
    // Create a signer from the private key
    let signer: PrivateKeySigner = private_key.parse().expect("should parse private key");
    let wallet = EthereumWallet::from(signer);
    // Create a provider with provided wallet
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet.clone())
        .on_http(rpc_url);
    // Create a contract instance of WETH9 on Optimism
    let contract = IWETH9::new(
        address!("4200000000000000000000000000000000000006"),
        provider,
    );
    // Call the contract, retrieve the total supply.
    let total_supply = contract.totalSupply().call().await?._0;
    println!("Total supply: {:?}", total_supply);
    // Call the contract, retrieve the balance of the wallet
    let balance = contract.balanceOf(wallet.default_signer().address()).call().await?._0;
    println!("Balance: {:?} WETH, deposit 1 ETH", balance);
    // Wrap 1 WETH
    let value = U256::from(1);
    let tx = contract.deposit().value(value);
    let receipt = tx.send().await?;
    println!("Wrapped 1 ETH at {:?}", receipt.tx_hash());
    // Wait for the transaction to be mined
    println!("Waiting for confirmation...");
    receipt.watch().await?;
    // Call the contract, retrieve the balance of the wallet
    let balance = contract.balanceOf(wallet.default_signer().address()).call().await?._0;
    println!("Balance: {:?} WETH", balance);
    Ok(())
}
