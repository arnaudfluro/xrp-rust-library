
//! A Rust library for interacting with the Ripple Testnet.

use anyhow::anyhow;
use crate::models::Transaction;
use crate::requests::RpcMethod::{AccountInfo, LedgerCurrent, Sign, Submit, Tx};
use crate::requests::{command_post_with_auth_header, AccountInfoParams, RequestHttp, SignParam, VerifyTxParam};
use crate::responses::{AccountInfoRpcResponse, CurrentLedgerRpcResponse, SignedTransactionRpcResponse, SubmitRpcResponse, VerificationRpcResponse};

pub struct Provider {
    pub endpoint_rpc: String
}

impl Provider {
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint_rpc: endpoint
        }
    }

    /// Get the current ledger
    pub async fn get_current_ledger_number(&self) -> anyhow::Result<u64>{
        let request_http = RequestHttp::new({}, LedgerCurrent);
        let request_http_json = serde_json::to_string(&request_http)?;
        let response_body = command_post_with_auth_header(&self.endpoint_rpc, request_http_json).await?;
        let current_ledger_response = CurrentLedgerRpcResponse::deserialize_current_ledger_response(response_body.as_ref())?;
        Ok(current_ledger_response.ledger_index())
    }

    /// Sign the transaction and return a SignedTransactionRpcResponse (contains txn_hash and txn_blob)
    pub async fn sign_txn(&self, transaction: Transaction, private_key: String) -> anyhow::Result<SignedTransactionRpcResponse> {
        let sign_param = SignParam {
            offline: false,
            secret: private_key,
            tx_json: transaction,
            fee_mult_max: 10000
        };
        let request_http = RequestHttp::new(sign_param, Sign);
        let request_http_json = serde_json::to_string(&request_http)?;
        let response_body = command_post_with_auth_header(&self.endpoint_rpc, request_http_json).await?;
        SignedTransactionRpcResponse::deserialize_signed_txn_response(response_body.as_ref())
    }

    /// Permit to sign, submit a transaction on the xrp blockchain.
    /// Return a txn_hash
    pub async fn submit_transaction(&self, txn_blob: String) -> anyhow::Result<String> {

        let request_http = RequestHttp::new(txn_blob, Submit);
        let request_http_json = serde_json::to_string(&request_http)?;
        let response_body = command_post_with_auth_header(&self.endpoint_rpc, request_http_json).await?;
        let response = SubmitRpcResponse::deserialize_submit_response(&response_body)?;

        //TODO: use trace framework to log efficiently
        println!("--- Transaction Submission Result ---");
        println!("Status: {}", response.result.status);
        println!("Engine Result: {} -> {}", response.result.engine_result, response.result.engine_result_message);

        Ok(response.transaction_hash().to_string())
    }

    pub fn get_fee(&self) -> anyhow::Result<u64> {
        //Default for the moment 13 drops
        Ok(13_u64)
    }

    /// Get the sequence of an account thanks to his public key
    pub async fn get_sequence(&self, account: String) -> anyhow::Result<AccountInfoRpcResponse> {

        let account_info_param = AccountInfoParams::new(account);

        let request_http = RequestHttp::new(account_info_param, AccountInfo);
        let request_http_json = serde_json::to_string(&request_http)?;
        let response_body = command_post_with_auth_header(&self.endpoint_rpc, request_http_json).await?;
        AccountInfoRpcResponse::deserialize_account_info_response(response_body.as_ref())
    }

    pub async fn verify_txn(&self, txn_hash: String) -> anyhow::Result<String> {
        let verify_txn_param = VerifyTxParam::new(txn_hash);

        let request_http = RequestHttp::new(verify_txn_param, Tx);
        let request_http_json = serde_json::to_string(&request_http)?;
        let response_body = command_post_with_auth_header(&self.endpoint_rpc, request_http_json).await?;
        let response = VerificationRpcResponse::deserialize_verification_response(&response_body)?;

        println!("--- Transaction Verification Result ---");
        let result_code = response.transaction_result();

        match (response.is_validated(), result_code) {
            (true, "tesSUCCESS") => {
                let verify_message = format!("Transaction SUCCESSFUL and VALIDATED in ledger index: {}",response.ledger_index());
                println!(
                    "{}", verify_message
                );
                Ok(verify_message)

            }
            (true, code) => {
                let verify_message = format!("Transaction VALIDATED in ledger index {} but FAILED with code: {}",response.ledger_index(),code);
                println!(
                    "{}", verify_message
                );
                Err(anyhow!(verify_message))
            }
            (false, code) => {
                let verify_message = format!("Transaction NOT YET VALIDATED. The preliminary result is: {}",code);
                println!(
                    "{}", verify_message
                );
                Err(anyhow!(verify_message))
            }
        }
    }

}