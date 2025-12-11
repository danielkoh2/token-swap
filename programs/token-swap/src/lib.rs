use anchor_lang::prelude::*;

declare_id!("8StNymmkJzA8oixeRCoafEj5sPtnqzBaYVfaGkBYSB3F");

#[program]
pub mod token_swap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
