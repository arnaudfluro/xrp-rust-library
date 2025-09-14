// tests/transaction_tests.rs

use juniorDevTaskXrpl::models::Transaction;
use juniorDevTaskXrpl::provider::Provider;
use std::time;
use tokio;

#[tokio::test]
async fn test_send_and_verify_token_flow() {
    // Prerequisites need to setup an issuer account, create a token, //
    // and set the trustlines with the user1 and 2 //

    // Wallet address
    let user1_address = "rwmLkyTwfPe8ZnBY8NDi1HRSze6Z6EPR9M".to_string();
    let user1_private_key = "sEdThW676x5bzLAAc8ku5kU8ZRWZNbN".to_string();
    let user2_address = "rcjxkuh2ksXe2Rn2D4693fmAojLCUdVEW".to_string();
    let user2_private_key = "sEdTXpqu34pEZL9duPWTtYybvMHxerc".to_string();
    let issuer_address = "rJZnfH9Hbbv2XmwrAeB1pEAbEZEr1sgNtX".to_string();
    let currency_code = "juniorXRPLTest".to_string();

    //Provider
    let provider = Provider::new("https://testnet.xrpl-labs.com/".to_string());

    //Build Transaction
    let transaction = Transaction::build_transaction(
        &provider,
        user1_address,
        user2_address,
        issuer_address,
        currency_code,
        1000
    ).await;


    // Sign tx
    let signed_response = provider.sign_txn(transaction, user1_private_key).await.unwrap();

    //Submit transaction
    let tx_hash = provider.submit_transaction(signed_response.result.tx_blob).await.unwrap();

    //Verify transaction (Waiting current_ledger >= ledger_submitted + 3)
    let nine_sec = time::Duration::from_millis(9000); //approximately 3 ledger
    tokio::time::sleep(nine_sec).await;
    let response = provider.verify_txn(tx_hash).await.unwrap();

    assert_eq!(response[0..51].to_string(), "Transaction SUCCESSFUL and VALIDATED in ledger index");
}
