use anchor_lang::prelude::*;

declare_id!("5xJNXoJhRUCixjzuzEbvP1qK3H4TxHx9cCcuRVMimWQm");

#[program]
pub mod day6 {
    use super::*;

    pub fn age_checker(ctx: Context<Initialize>,
        age: u64) -> Result<()> {
    if age >= 18 {
        msg!("You are 18 years old or above");
    } else {
        msg!("You are below 18 years old");
    }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
