use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("ohh9qHi1dms4GJCRCUamqPNnHXTGhcmoWAmG41UwJ76");

#[program]
pub mod day29 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8, seeds = [], bump)]
    pub my_storage: Account<'info, MyStorage>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MyStorage {}