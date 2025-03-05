use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

declare_id!("Cj2icEd2Zb8dzZQ6eEyUdTm5n86hBxD2KNU9qJtS9pSR");

#[program]
pub mod solend_wrapper {
    use super::*;

    // Deposit instruction
    pub fn deposit(_ctx: Context<Deposit>, _amount: u64) -> Result<()> {
        // Solend deposit logic
        Ok(())
    }

    // Borrow instruction
    pub fn borrow(_ctx: Context<Borrow>, _amount: u64) -> Result<()> {
        // Solend borrow logic
        Ok(())
    }

    // Repay instruction
    pub fn repay(_ctx: Context<Repay>, _amount: u64) -> Result<()> {
        // Solend repay logic
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    pub user: Signer<'info>,
    /// CHECK: This is interface
    pub reserve: AccountInfo<'info>,
    pub reserve_liquidity_supply: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_liquidity_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub collateral_account: Account<'info, TokenAccount>,
    pub clock: Sysvar<'info, Clock>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Borrow<'info> {
    pub user: Signer<'info>,
    /// CHECK: This is interface
    pub reserve: AccountInfo<'info>,
    pub reserve_liquidity_supply: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_liquidity_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub collateral_account: Account<'info, TokenAccount>,
    pub clock: Sysvar<'info, Clock>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Repay<'info> {
    pub user: Signer<'info>,
    /// CHECK: This is interface
    pub reserve: AccountInfo<'info>,
    pub reserve_liquidity_supply: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_liquidity_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub collateral_account: Account<'info, TokenAccount>,
    pub clock: Sysvar<'info, Clock>,
    pub token_program: Program<'info, Token>,
}