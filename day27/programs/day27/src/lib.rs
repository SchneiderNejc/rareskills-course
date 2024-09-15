use anchor_lang::prelude::*;
use std::mem::size_of;
use anchor_lang::system_program;


declare_id!("3PPpBczWaqqitCZHYWrFvGyXJevNTB21rWTfL5ncF3GK");

#[program]
pub mod day27 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn erase(ctx: Context<Erase>) -> Result<()> {
        ctx.accounts.my_pda.realloc(0, false)?;
        Ok(())
    }
}



#[derive(Accounts)]
pub struct Erase<'info> {
		/// CHECK: We are going to erase the account
    #[account(mut)]
    pub my_pda: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8, seeds = [], bump)]
    pub my_pda: Account<'info, MyPDA>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct MyPDA {}