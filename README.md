# juniorDevTaskXrpl

This project provides a Rust framework for interacting with the XRP Ledger (XRPL), focusing on building, signing, submitting, and verifying transactions, particularly for custom token payments.

## Table of Contents

*   [Features](#features)
*   [Dependencies](#dependencies)
*   [Usage](#usage)
    *   [Provider Initialization](#provider-initialization)
    *   [Building a Transaction](#building-a-transaction)
    *   [Signing and Submitting a Transaction](#signing-and-submitting-a-transaction)
    *   [Verifying a Transaction](#verifying-a-transaction)
*   [Running Tests](#running-tests)
*   [Prerequisites for Testing](#prerequisites-for-testing)

## Features

*   **Provider**: Manages connections to an XRPL node and handles various XRPL RPC calls (e.g., getting ledger info, account sequence, transaction fees).
*   **Transaction Building**: Simplifies the creation of XRPL payment transactions, auto-filling necessary fields like `sequence`, `fee`, and `last_ledger_sequence`.
*   **Transaction Management**: Supports signing, submitting, and verifying transactions on the XRPL.

## Dependencies

The project leverages the following key Rust crates:

*   `tokio`: An asynchronous runtime for building network applications.
*   `serde` & `serde_json`: For serializing and deserializing data to/from JSON, which is essential for XRPL RPC interactions.
*   `anyhow`: A flexible concrete Error type built on `std::error::Error`.

## Usage

The core interaction with the XRPL happens through the `Provider` and `Transaction` structs, as demonstrated in `src/provider.rs` and `src/models.rs`.

### Provider Initialization

First, create an instance of the `Provider` by specifying the XRPL node endpoint.

```rust src/provider.rs
pub fn new(endpoint: String) -> Self {
    // ... existing code ...
}
```

```rust
// Example
let provider = Provider::new("https://testnet.xrpl-labs.com/".to_string());
```

### Building a Transaction

Use the `Transaction::build_transaction` asynchronous method to construct a payment transaction. This method automatically fetches the current ledger, account sequence, and transaction fee from the XRPL.

```rust src/models.rs
impl Transaction {
    pub async fn build_transaction(provider: &Provider, user1_address: String, user2_address: String, issuer_address: String, currency_code: String, amount: u64) -> Self {
        // ... existing code ...
    }
}
```

```rust
// Example
let transaction = Transaction::build_transaction(
    &provider,
    user1_address,
    user2_address,
    issuer_address,
    currency_code,
    1000 // amount
).await;
```

### Signing and Submitting a Transaction

Once a `Transaction` is built, it needs to be signed with the sender's private key and then submitted to the XRPL.

```rust src/provider.rs
impl Provider {
    pub async fn sign_txn(&self, transaction: Transaction, private_key: String) -> anyhow::Result<SignedTransactionRpcResponse> {
        // ... existing code ...
    }

    pub async fn submit_transaction(&self, txn_blob: String) -> anyhow::Result<String> {
        // ... existing code ...
    }
}
```

```rust
// Example
let signed_response = provider.sign_txn(transaction, user1_private_key).await.unwrap();
let tx_hash = provider.submit_transaction(signed_response.result.tx_blob).await.unwrap();
println!("Transaction submitted with hash: {}", tx_hash);
```

### Verifying a Transaction

After submission, you can verify the transaction status using its hash. It's often necessary to wait for a few ledgers to close for the transaction to be validated.

```rust src/provider.rs
impl Provider {
    pub async fn verify_txn(&self, txn_hash: String) -> anyhow::Result<String> {
        // ... existing code ...
    }
}
```

```rust
// Example (from tests/transaction_tests.rs)
use tokio::time; // Make sure to have `use tokio::time;` for `time::Duration`

let nine_sec = time::Duration::from_millis(9000); // Wait approximately 3 ledgers
tokio::time::sleep(nine_sec).await;
let response = provider.verify_txn(tx_hash).await.unwrap();
println!("Verification response: {}", response);
```

## Running Tests

To run the integration tests, use Cargo's test command. The test `test_send_and_verify_token_flow` located in `tests/transaction_tests.rs` demonstrates the full lifecycle of a token payment transaction.

```bash
cargo test
```

## Prerequisites for Testing

The `test_send_and_verify_token_flow` test in `tests/transaction_tests.rs` assumes certain conditions are met on the XRPL testnet:

*   **Issuer Account**: An XRPL account must be set up as an issuer for the custom token (e.g., `juniorXRPLTest`).
*   **Token Creation**: The custom token must have been issued by the issuer account.
*   **Trustlines**: Both `user1_address` and `user2_address` must have trustlines established with the `issuer_address` for the `juniorXRPLTest` currency. Without these trustlines, transactions involving the custom token will fail.