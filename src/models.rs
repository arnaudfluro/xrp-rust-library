// ... (existing code) ...

use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::provider::Provider;

/// Represents the amount of a non-XRP token (issued asset).
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct DeliverMax {
    currency: String,
    value: String,
    issuer: String,
}

/// Represents a Payment transaction on the XRP Ledger (before signing).
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    #[serde(rename = "TransactionType")]
    transaction_type: String, // e.g., "Payment"
    #[serde(rename = "Account")]
    account: String, // Sender's XRP address
    #[serde(rename = "Destination")]
    destination: String, // Recipient's XRP address
    #[serde(rename = "DeliverMax")]
    deliver_max: DeliverMax, // Nested Amount struct for issued assets
    #[serde(rename = "Fee")]
    fee: String, // Transaction fee in drops (e.g., "12")
    #[serde(rename = "Sequence")]
    sequence: u64, // Account's current sequence number (Nonce)
    #[serde(rename = "LastLedgerSequence", skip_serializing_if = "Option::is_none")]
    last_ledger_sequence: Option<u32>, // Optional: Last valid ledger for this transaction
}

impl Transaction {
    /// Build a transaction following xrp transaction
    /// Auto-fill the required field (e.g: sequence, fee, current ledger)
    pub async fn build_transaction(provider: &Provider, user1_address: String, user2_address: String, issuer_address: String, currency_code: String, amount: u64) -> Self {
        let current_ledger = provider.get_current_ledger_number().await.unwrap() as u32;
        let current_sequence = provider.get_sequence(user1_address.clone()).await.unwrap().sequence();
        let current_fee = provider.get_fee().unwrap().to_string();
        Transaction {
            transaction_type: "Payment".to_string(),
            account: user1_address,
            destination: user2_address,
            deliver_max: DeliverMax {
                currency: currency_code,
                value: amount.to_string(),
                issuer: issuer_address,
            },
            fee: current_fee,
            sequence: current_sequence + 1,
            last_ledger_sequence: Some(current_ledger + 3),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct  TransactionJsonDetails {
    #[serde(rename = "Account")]
    pub account: String,
    #[serde(rename = "DeliverMax")]
    pub deliver_max: DeliverMax,
    #[serde(rename = "Destination")]
    pub destination: String,
    #[serde(rename = "Fee")]
    pub fee: String,
    #[serde(rename = "Flags")]
    pub flags: u32,
    #[serde(rename = "Sequence")]
    pub sequence: u32,
    #[serde(rename = "SigningPubKey")]
    pub signing_pub_key: String,
    #[serde(rename = "TransactionType")]
    pub transaction_type: String,
    #[serde(rename = "TxnSignature")]
    pub txn_signature: String,
    pub hash: String,
}
