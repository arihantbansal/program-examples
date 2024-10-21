use anchor_lang::prelude::*;

declare_id!("DXTHoWDkvvpRsz1eaDbxdkZKZXure5XfLUbTGabg2VTg");

#[program]
pub mod token_minter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
