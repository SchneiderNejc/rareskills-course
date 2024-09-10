use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("9bQDwFx9JWGk7ASYK2x1vj3iZpbL61pUXGNfVxZbZZ1G");

const STARTING_POINTS: u32 = 10;

#[program]
pub mod day24 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.player.points = STARTING_POINTS;
        ctx.accounts.player.authority = ctx.accounts.signer.key();
        Ok(())
    }

    pub fn transfer_points(ctx: Context<TransferPoints>,
                           amount: u32) -> Result<()> {
        require!(ctx.accounts.from.authority == ctx.accounts.signer.key(),
								 Errors::SignerIsNotAuthority);
        require!(ctx.accounts.from.points >= amount,
                 Errors::InsufficientPoints);

        ctx.accounts.from.points -= amount;
        ctx.accounts.to.points += amount;
        Ok(())
    }
}







#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,
              payer = fren,
              space=size_of::<MyStorage>() + 8,
              seeds = [],
              bump)]
    pub my_storage: Account<'info, MyStorage>,

    #[account(mut)]
    pub fren: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateValue<'info> {
    #[account(mut, seeds = [], bump)]
    pub my_storage: Account<'info, MyStorage>,

	// THIS FIELD MUST BE INCLUDED
    #[account(mut)]
    pub fren: Signer<'info>,
}

#[account]
pub struct MyStorage {
    x: u64,
}
