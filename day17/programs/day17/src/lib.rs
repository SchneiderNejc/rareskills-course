use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("AXX2p8v4GzJiKbtMAbv8aTnRJaMFfFwUQFWWdLSj3S66");

#[program]
pub mod day17 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn set(ctx: Context<Set>, new_x: u64) -> Result<()> {
        // Basic syntax
        //ctx.accounts.my_storage.x = new_x;
        // Improved syntax is better for multiple storage operations.
        let my_storage = &mut ctx.accounts.my_storage;
	    my_storage.x = new_x;
        Ok(())
    }

    pub fn print_x(ctx: Context<PrintX>) -> Result<()> {
        let x = ctx.accounts.my_storage.x;
        msg!("The value of x is {}", x);
        Ok(())
    }

    pub fn increment(ctx: Context<Set>) -> Result<()> {
        ctx.accounts.my_storage.x += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Set<'info> {
    #[account(mut, seeds = [], bump)]
    pub my_storage: Account<'info, MyStorage>,
}

#[derive(Accounts)]
pub struct PrintX<'info> {
    pub my_storage: Account<'info, MyStorage>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,
              payer = signer,
              space=size_of::<MyStorage>() + 8,
              seeds = [],
              bump)]
    pub my_storage: Account<'info, MyStorage>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct MyStorage {
    x: u64,
}
