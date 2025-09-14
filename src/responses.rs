use std::process::{Command, Output};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use crate::models::TransactionJsonDetails;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SignedTransactionResult {
    pub status: String,
    pub tx_blob: String,
    pub tx_json: TransactionJsonDetails,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SignedTransactionRpcResponse {
    pub result: SignedTransactionResult,
}

impl SignedTransactionRpcResponse {
    pub fn deserialize_signed_txn_response(response_str: &str) -> anyhow::Result<Self> {
        serde_json::from_str(response_str)
            .map_err(|e| anyhow!("Error while deserializing signed transaction response: {}. JSON: '{}'", e, response_str))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LedgerResult {
    pub ledger_current_index: u64,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentLedgerRpcResponse {
    pub result: LedgerResult,
}

impl CurrentLedgerRpcResponse {
    pub fn deserialize_current_ledger_response(response_str: &str) -> anyhow::Result<Self> {
        serde_json::from_str(response_str)
            .map_err(|e| anyhow!("Error while deserializing current ledger response: {}. JSON: '{}'", e, response_str))
    }

    pub fn ledger_index(&self) -> u64 {
        self.result.ledger_current_index
    }
}


//TODO: Implement error response
async fn command_get(enpoint_rpc: String) -> Output {
    Command::new("curl")
        .arg("-X")
        .arg("GET")
        .arg(enpoint_rpc)  // URL du service JSON-RPC
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("--max-time")
        .arg("10") // 10s timeout
        .output()
        .expect("failed to execute process")
}

#[derive(Deserialize, Debug)]
struct AccountData {
    #[serde(rename = "Sequence")]
    sequence: u64,
}

#[derive(Deserialize, Debug)]
struct AccountInfoResult {
    account_data: AccountData,
}

#[derive(Deserialize, Debug)]
pub struct AccountInfoRpcResponse {
    result: AccountInfoResult,
}

impl AccountInfoRpcResponse {
    pub fn deserialize_account_info_response(response_str: &str) -> anyhow::Result<Self> {
        serde_json::from_str(response_str)
            .map_err(|e| anyhow!("Error while deserializing account info response: {}. JSON: '{}'", e, response_str))
    }

    pub fn sequence(&self) -> u64 {
        self.result.account_data.sequence
    }
}

#[derive(Deserialize, Debug)]
pub struct TxJsonForSubmit {
    #[serde(rename = "hash")]
    hash: String,
}

#[derive(Deserialize, Debug)]
pub struct SubmitResult {
    pub status: String,
    pub engine_result: String,
    pub engine_result_message: String,
    applied: bool,
    tx_json: TxJsonForSubmit,
}

#[derive(Deserialize, Debug)]
pub struct SubmitRpcResponse {
    pub result: SubmitResult,
    validated_ledger_index: Option<u64>,
}

impl SubmitRpcResponse {
    pub fn deserialize_submit_response(response_str: &str) -> anyhow::Result<Self> {
        serde_json::from_str(response_str)
            .map_err(|e| anyhow!("Error while deserializing submit response: {}. JSON: '{}'", e, response_str))
    }

    pub fn transaction_hash(&self) -> &str {
        &self.result.tx_json.hash
    }
}

#[derive(Deserialize, Debug)]
struct Meta {
    #[serde(rename = "TransactionResult")]
    transaction_result: String,
}

#[derive(Deserialize, Debug)]
struct VerificationResult {
    validated: bool,
    ledger_index: u64,
    hash: String,
    meta: Meta,
}

#[derive(Deserialize, Debug)]
pub struct VerificationRpcResponse {
    result: VerificationResult,
}

impl VerificationRpcResponse {
    pub fn deserialize_verification_response(response_str: &str) -> anyhow::Result<Self> {
        serde_json::from_str(response_str)
            .map_err(|e| anyhow!("Error while deserializing verification response: {}. JSON: '{}'", e, response_str))
    }

    pub fn is_validated(&self) -> bool {
        self.result.validated
    }

    pub fn ledger_index(&self) -> u64 {
        self.result.ledger_index
    }

    pub fn transaction_result(&self) -> &str {
        &self.result.meta.transaction_result
    }
}
