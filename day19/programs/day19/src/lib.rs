use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("AfD7NnFyUnHfoS8kvhK5MTvuLqEhVggEKnJmpKuKM4qG");

#[program]
pub mod day19 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, key1: u64, key2: u64) -> Result<()> {
        Ok(())
    }

    pub fn set(ctx: Context<Set>, key1: u64, key2: u64, val: u64) -> Result<()> {
        ctx.accounts.val.value = val;
        Ok(())
    }


}

#[derive(Accounts)]
#[instruction(key1: u64, key2: u64)] // new key args added
pub struct Set<'info> {
    #[account(mut)]
    val: Account<'info, Val>,
}

#[derive(Accounts)]
#[instruction(key: u64)]
pub struct Initialize<'info> {

    #[account(init,
        payer = signer,
        space = size_of::<Val>() + 8,
        seeds=[&key1.to_le_bytes().as_ref(), &key2.to_le_bytes().as_ref()], // 2 seeds
        bump)]
val: Account<'info, Val>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}

#[account]
pub struct Val {
    value: u64,
}
