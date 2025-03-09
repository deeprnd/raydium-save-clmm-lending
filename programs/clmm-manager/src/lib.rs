use anchor_lang::prelude::*;

mod errors;
pub mod instructions;

use instructions::*;

declare_id!("441hQCsxuqCJumMNPdGaZxBzTtuLbadAaH3hcnTufPZR"); // devnet

#[program]
pub mod clmm_manager {
    use super::*;

    pub fn open_position<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, OpenCLLMPosition<'info>>,
        tick_lower_index: i32,
        tick_upper_index: i32,
        tick_array_lower_start_index: i32,
        tick_array_upper_start_index: i32,
        liquidity: u128,
        amount_0_max: u64,
        amount_1_max: u64,
        with_matedata: bool,
        base_flag: Option<bool>,
    ) -> Result<()> {
        instructions::proxy_open_position(
            ctx,
            tick_lower_index,
            tick_upper_index,
            tick_array_lower_start_index,
            tick_array_upper_start_index,
            liquidity,
            amount_0_max,
            amount_1_max,
            with_matedata,
            base_flag,
        )
    }
    pub fn close_position<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, CloseCLLMPosition<'info>>,
    ) -> Result<()> {
        instructions::proxy_close_position(ctx)
    }

    pub fn increase_liquidity<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, IncreaseCLLMLiquidity<'info>>,
        liquidity: u128,
        amount_0_max: u64,
        amount_1_max: u64,
        base_flag: Option<bool>,
    ) -> Result<()> {
        instructions::proxy_increase_liquidity(
            ctx,
            liquidity,
            amount_0_max,
            amount_1_max,
            base_flag,
        )
    }
    pub fn decrease_liquidity<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, DecreaseCLLMLiquidity<'info>>,
        liquidity: u128,
        amount_0_min: u64,
        amount_1_min: u64,
    ) -> Result<()> {
        instructions::proxy_decrease_liquidity(ctx, liquidity, amount_0_min, amount_1_min)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::proxy_deposit_loan::deposit(ctx, amount)
    }

    pub fn borrow(ctx: Context<Borrow>, amount: u64) -> Result<()> {
        instructions::proxy_borrow_loan::borrow(ctx, amount)
    }

    pub fn repay_loan(ctx: Context<Repay>, amount: u64) -> Result<()> {
        instructions::proxy_repay_loan::repay(ctx, amount)
    }
}
