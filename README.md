# Alloy Rust Boilerplate

This project is a boilerplate for interacting with the blockchain using Rust and Alloy crate. It leverages the `alloy-rs` crate to connect to the network, send transactions, and interact with smart contracts.

## Prerequisites

Before running the project, ensure you have the following installed:

- Rust (nightly or stable)
- Cargo
- A running node, we suggest using [Foundry's Anvil](https://book.getfoundry.sh/anvil/) to run a local Ethereum network.

## Configure `.env` file
In the root of the project, create a `.env` file and set the following environment variables:
```
PRIVATE_KEY=<your-private-key>
```

## Run the project
To run the project, use the following command:
```
cargo run
```

## Development
To run the project in development mode, use the following command:
```
cargo dev
```