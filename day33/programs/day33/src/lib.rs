use anchor_lang::prelude::*;
use std::mem::size_of;


declare_id!("DTwEUK34nyL1yQ7DJRd7rFEPwYGfqQ8ph7mRh762uFMY");

#[program]
pub mod day33 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Data Account Initialized: {}", ctx.accounts.bob_data_account.key());

        Ok(())
    }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = size_of::<BobData>() + 8)]
    pub bob_data_account: Account<'info, BobData>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
#[account]
pub struct BobData {
    pub result: u64,
}