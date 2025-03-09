#[cfg(test)]
mod tests {
    use solana_sdk::{
        pubkey::Pubkey,
        signature::{Keypair, read_keypair_file, Signer},
        transaction::Transaction,
        system_instruction,
        commitment_config::CommitmentConfig,
    };
    use solana_client::rpc_client::RpcClient;
    use std::str::FromStr;
    use tokio;
    use std::path::PathBuf;

    const DEVNET_RPC_URL: &str = "https://api.devnet.solana.com";

    #[tokio::test]
    async fn test_increase_liquidity() {
        // Define the pool ID
        let pool_id = Pubkey::from_str("Enfoa5Xdtirwa46xxa5LUVcQWe7EUb2pGzTjfvU7EBS1").unwrap();

        // Read the keypair from ~/.solana/owner.json
        let mut keypair_path = dirs::home_dir().expect("Could not find home directory");
        keypair_path.push(".solana/owner.json");

        let user_keypair = read_keypair_file(&keypair_path)
            .expect("Failed to read keypair file at ~/.solana/owner.json");

        // Increase liquidity
        if let Err(err) = increase_liquidity(&pool_id, &user_keypair).await {
            eprintln!("Error: {}", err);
            panic!("Test failed: {}", err);
        }
    }

    async fn increase_liquidity(pool_id: &Pubkey, user_keypair: &Keypair) -> Result<(), Box<dyn std::error::Error>> {
        // Create an RPC client
        let rpc_client = RpcClient::new_with_commitment(DEVNET_RPC_URL.to_string(), CommitmentConfig::confirmed());

        // Fetch pool info (placeholder, replace with actual logic)
        let pool_info = fetch_pool_info(&rpc_client, pool_id).await?;
        println!("Pool Info: {:?}", pool_info);

        // Fetch user's positions in the pool (placeholder, replace with actual logic)
        let user_positions = fetch_user_positions(&rpc_client, &user_keypair.pubkey(), &pool_info.program_id).await?;
        if user_positions.is_empty() {
            return Err("User has no positions in this pool".into());
        }

        let position = &user_positions[0]; // Take the first position for simplicity

        // Calculate liquidity amount (simplified, adjust with actual calculation)
        let input_amount = 0.0001; // SOL
        let liquidity_amount = calculate_liquidity(input_amount, &pool_info);

        // Prepare and send the transaction
        let recent_blockhash = rpc_client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &[
                system_instruction::transfer(
                    &user_keypair.pubkey(),
                    pool_id,
                    liquidity_amount,
                ),
            ],
            Some(&user_keypair.pubkey()),
            &[user_keypair],
            recent_blockhash,
        );

        // Send the transaction
        let signature = rpc_client.send_and_confirm_transaction(&transaction)?;
        println!("Transaction sent: https://explorer.solana.com/tx/{}?cluster=devnet", signature);

        Ok(())
    }

    async fn fetch_pool_info(rpc_client: &RpcClient, pool_id: &Pubkey) -> Result<PoolInfo, Box<dyn std::error::Error>> {
        // Fetch the pool account data
        let account_data = rpc_client.get_account_data(pool_id)?;

        // Parse the account data into a PoolInfo struct (placeholder, replace with actual parsing logic)
        let pool_info = PoolInfo {
            program_id: Pubkey::default(), // Replace with actual program ID
            // Add other fields as needed
        };

        Ok(pool_info)
    }

    async fn fetch_user_positions(rpc_client: &RpcClient, user_pubkey: &Pubkey, program_id: &Pubkey) -> Result<Vec<Position>, Box<dyn std::error::Error>> {
        // Fetch the user's positions using get_program_accounts
        let filters = vec![
            solana_client::rpc_filter::RpcFilterType::Memcmp(
                solana_client::rpc_filter::Memcmp::new(8, user_pubkey.to_string()),
            ),
        ];

        let accounts = rpc_client.get_program_accounts_with_config(
            program_id,
            solana_client::rpc_client::GetProgramAccountsConfig {
                filters: Some(filters),
                account_config: solana_client::rpc_client::RpcAccountInfoConfig {
                    encoding: Some(solana_client::rpc_client::UiAccountEncoding::Base64),
                    ..Default::default()
                },
                ..Default::default()
            },
        )?;

        // Parse the accounts into a Vec<Position> (placeholder, replace with actual parsing logic)
        let positions = accounts
            .into_iter()
            .map(|(_, account)| {
                Position {
                    // Parse account data into Position fields
                }
            })
            .collect();

        Ok(positions)
    }

    fn calculate_liquidity(amount: f64, pool_info: &PoolInfo) -> u64 {
        // Simplified calculation: convert SOL to lamports
        (amount * 1_000_000_000.0) as u64
    }

    // Define your data structures
    #[derive(Debug)]
    struct PoolInfo {
        program_id: Pubkey,
        // Add other fields as needed
    }

    #[derive(Debug)]
    struct Position {
        // Define position fields as needed
    }
}