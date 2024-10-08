use anchor_lang::prelude::*;
use std::mem::size_of;


declare_id!("5QpKqiLQkgDmkJCQETbN8Un1QDKRsmigm5wNsQy4i3DE");

#[program]
pub mod day20 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn increase_account_size(ctx: Context<IncreaseAccountSize>) -> Result<()> {
        Ok(())
    }
}


#[derive(Accounts)]
pub struct IncreaseAccountSize<'info> {

    #[account(mut,
							// ***** 1,000 BYTE INCREMENT IS OVER HERE *****
              realloc = size_of::<MyStorage>() + 8 + 1000, // @dev Maximum is 10240.
              realloc::payer = signer,
              realloc::zero = false, // @dev If set to true, account data is erased (set to all zeros).
              seeds = [],
              bump)]
    pub my_storage: Account<'info, MyStorage>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
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
