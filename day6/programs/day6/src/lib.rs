use anchor_lang::prelude::*;

declare_id!("5xJNXoJhRUCixjzuzEbvP1qK3H4TxHx9cCcuRVMimWQm");

#[program]
pub mod day6 {
    use super::*;

    pub fn age_checker(ctx: Context<Initialize>,
        age: u64) -> Result<()> {

        let result = if age >= 18 {"You are 18 years old or above"} else { "You are below 18 years old" };
        msg!("{:?}", result);
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize {}
