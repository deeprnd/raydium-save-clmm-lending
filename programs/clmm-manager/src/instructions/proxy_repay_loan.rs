use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use anchor_spl::token_interface::TokenAccount;
use solend_wrapper::cpi::accounts::Repay as SolendRepay;
use solend_wrapper::cpi::repay as solend_repay;

#[derive(Accounts)]
pub struct Repay<'info> {
    pub user: Signer<'info>,
    pub solend_program: Program<'info, solend_wrapper::program::SolendWrapper>,
    pub reserve: AccountInfo<'info>,
    pub reserve_liquidity_supply: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub user_liquidity_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub collateral_account: InterfaceAccount<'info, TokenAccount>,
    pub clock: Sysvar<'info, Clock>,
    pub token_program: Program<'info, Token>,
}

// Repay borrowed tokens to Solend
pub fn repay(ctx: Context<Repay>, amount: u64) -> Result<()> {
    let cpi_accounts = SolendRepay {
        user: ctx.accounts.user.to_account_info(),
        reserve: ctx.accounts.reserve.to_account_info(),
        reserve_liquidity_supply: ctx.accounts.reserve_liquidity_supply.to_account_info(),
        user_liquidity_account: ctx.accounts.user_liquidity_account.to_account_info(),
        collateral_account: ctx.accounts.collateral_account.to_account_info(),
        clock: ctx.accounts.clock.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(ctx.accounts.solend_program.to_account_info(), cpi_accounts);
    solend_repay(cpi_ctx, amount)?;
    Ok(())
}