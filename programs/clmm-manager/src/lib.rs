use anchor_lang::prelude::*;

declare_id!("E9iJcnzprVCjrJiSVfPKVEqLvSN2jTn9qwKXJFhEUnrm");

#[program]
pub mod clmm_manager {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
