use anchor_lang::prelude::*;

declare_id!("8keP6ofyuxTHX9g1s1vQbWbiM4ezDbPBmHV2nusqBTm9");

#[program]
pub mod dapp_sol {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
