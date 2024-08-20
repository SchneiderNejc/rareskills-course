use anchor_lang::prelude::*;

declare_id!("D8f47hT2NXBUJjU1hGERmnwA3z9zj7Jgy2WhnLm1a3zt");

#[program]
pub mod got {
    use super::*;

    pub fn initialize2(ctx: Context[<Initialize>) -> Result<()> {
        msg!("WE ARE KINGS");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
