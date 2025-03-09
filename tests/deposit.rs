use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::program_pack::Pack;
use anchor_spl::token::{self, Token, TokenAccount};
use solana_sdk::{
    signature::Keypair,
    signer::Signer,
    transaction::Transaction,
    system_program,
    pubkey::Pubkey,
    commitment_config::CommitmentConfig,
};
use solana_client::rpc_client::RpcClient;
use std::str::FromStr;

// Replace with your program IDs
const CLMM_MANAGER_PROGRAM_ID: &str = "441hQCsxuqCJumMNPdGaZxBzTtuLbadAaH3hcnTufPZR";
const SOLEND_PROGRAM_ID: &str = "ALend7Ketfx5bxh6ghsCDXAoDrhvEmsXT3cynB6aPLgx";

#[tokio::test]
#[ignore]
async fn test_deposit_on_devnet() {
    // Connect to Devnet
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // Load the program IDs
    let program_id = Pubkey::from_str(CLMM_MANAGER_PROGRAM_ID).unwrap();
    let solend_program_id = Pubkey::from_str(SOLEND_PROGRAM_ID).unwrap();

    // Create a user account
    let user = Keypair::new();
    let user_pubkey = user.pubkey();

    // Airdrop SOL to the user account (for fees and deposit)
    let airdrop_amount = 1_000_000_000; // 1 SOL in lamports
    client
        .request_airdrop(&user_pubkey, airdrop_amount)
        .await
        .unwrap();

    // Wait for the airdrop to be confirmed
    client
        .confirm_transaction_with_commitment(
            &client
                .get_latest_blockhash()
                .await
                .unwrap(),
            CommitmentConfig::confirmed(),
        )
        .await
        .unwrap();

    let reserve = Keypair::new();
    let reserve_pubkey = reserve.pubkey();
    let liquidity_supply = Keypair::new();
    let liquidity_supply_pubkey = liquidity_supply.pubkey();

    let user_lp_token_account = Keypair::new();
    let user_lp_token_account_pubkey = user_lp_token_account.pubkey();

    // Create a transaction to deposit 1/10 SOL
    let deposit_amount = 1_000_100_000 /  10;

    let instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(user_pubkey, true),
            AccountMeta::new_readonly(solend_program_id, false),
            AccountMeta::new(reserve_pubkey, false),
            AccountMeta::new(liquidity_supply_pubkey, false),
            AccountMeta::new(user_lp_token_account_pubkey, false),
            AccountMeta::new_readonly(system_program::id(), false),
            AccountMeta::new_readonly(token::ID, false),
        ],
        data: clmm_manager::instruction::Deposit { amount: deposit_amount }.data(),
    };

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&user_pubkey),
        &[&user],
        client
            .get_latest_blockhash()
            .await
            .unwrap(),
    );

    // Send the transaction
    client
        .send_and_confirm_transaction(&transaction)
        .await
        .unwrap();

    // Verify the user's LP token balance
    let user_lp_token_account_data = client
        .get_account_data(&user_lp_token_account_pubkey)
        .await
        .unwrap();
    let user_lp_token_balance = TokenAccount::unpack(&user_lp_token_account_data)
        .unwrap()
        .amount;

    assert!(user_lp_token_balance > 0, "User did not receive LP tokens");
}