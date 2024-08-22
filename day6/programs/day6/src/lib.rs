use anchor_lang::prelude::*;

declare_id!("5xJNXoJhRUCixjzuzEbvP1qK3H4TxHx9cCcuRVMimWQm");

#[program]
pub mod day6 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
