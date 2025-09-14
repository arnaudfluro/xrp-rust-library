use std::ffi::CString;
use std::process::{Command, Output};
use serde::{Deserialize, Serialize};
use crate::models::Transaction;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
/// Tx: verigy the transaction
/// Submit: to submit a signed transaction
/// Sign: Sign a transaction
/// LedgerCurrent: Permit to get the current block
/// AcountInfo: info related to the account (e.g: sequence)
pub enum RpcMethod {
    Tx,
    Submit,
    Sign,
    LedgerCurrent,
    AccountInfo
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionParam {
    #[serde(rename = "transaction")]
    txn_hash: String,
    binary: bool,
}

#[derive(Debug, Serialize)]
pub struct AccountInfoParams {
    pub account: String,
    pub ledger_index: String,
    pub queue: bool,
}

impl AccountInfoParams {
    pub fn new(account: String) -> Self {
        Self {
            account,
            ledger_index: "current".to_string(),
            queue: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub struct RequestHttp<T> {
    method: RpcMethod,
    params: [T; 1],
}

impl<T> RequestHttp<T> {
    pub(crate) fn new(request_params: T, method_name: RpcMethod) -> Self {
        Self {
            method: method_name,
            params: [request_params],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SubmitParam {
    pub tx_blob: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SignParam {
    pub(crate) offline: bool,
    pub(crate) secret: String,
    pub(crate) tx_json: Transaction,
    pub(crate) fee_mult_max: u64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct VerifyTxParam {
    transaction: String,
    binary: bool,
    api_version: u8,
}

impl VerifyTxParam {
    pub fn new(txn_hash: String) -> Self {
        Self {
            transaction: txn_hash,
            binary: false,
            api_version: 2,
        }
    }
}

pub async fn command_post_with_auth_header(enpoint_rpc: &str, data: String) -> anyhow::Result<String> {
    let response = Command::new("curl")
        .arg("-X")
        .arg("POST")
        .arg(enpoint_rpc)  // URL HTTP
        .arg("-H")
        .arg("Content-Type: application/json")
        //.arg("-H")
        // .arg(authorization_header)
        .arg("-d")
        .arg(data)    // JSON body (Standard or Json RPC)
        .arg("--max-time")
        .arg("10") // 10s timeout
        .output()
        .expect("failed to execute process");

    if !response.status.success() {
        // handle http error
        return Err(anyhow::anyhow!(
            "HTTP Error: {} - {}",
            response.status,
            String::from_utf8_lossy(response.stderr.as_slice())
        ));
    }

    let response_body = String::from_utf8_lossy(response.stdout.as_slice());
    Ok(response_body.to_string())
}