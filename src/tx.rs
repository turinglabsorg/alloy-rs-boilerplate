#![allow(unused_imports)]
use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use dotenv::dotenv;
use eyre::Result;
use std::env;

#[allow(dead_code)]
pub async fn tx() -> Result<()> {
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
    // Get latest block number.
    let latest_block = provider.get_block_number().await?;
    // Print the block number.
    println!("Latest block number: {latest_block}");
    println!("Wallet address: {:?}", wallet.default_signer().address());
    // Get the nonce of the wallet
    let nonce = provider
        .get_transaction_count(wallet.default_signer().address())
        .await?;
    println!("Nonce: {:?}", nonce);
    // Create a transaction request
    let tx = TransactionRequest::default()
        .with_nonce(nonce)
        .with_from(wallet.default_signer().address())
        .with_to(address!("0000000000000000000000000000000000000000"))
        .with_value(U256::from(1));
    // Send the transaction and wait for inclusion.
    let tx_hash = provider.send_transaction(tx).await?;
    println!("Transaction sent: {:?}", tx_hash.tx_hash());
    let mined = tx_hash.watch().await?;
    // Wait for the transaction to be mined
    let receipt = provider.get_transaction_receipt(mined).await?;
    println!("Transaction mined: {:?}", receipt);
    Ok(())
}
