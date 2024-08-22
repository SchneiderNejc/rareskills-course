use anchor_lang::prelude::*;

declare_id!("BLRa76aKxcHHdugF3P1hoqZgcAEHn8HdbYvkSHqH8Zo8");

#[program]
pub mod day5 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Welcome to solana.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}