use anchor_lang::prelude::*;

declare_id!("26V3qX3sQsxxTyvL555M4J6a6FUBhqvwAWvuiswVApjZ");

#[program]
pub mod day21 {
    use super::*;

    pub fn read_balance(ctx: Context<ReadBalance>) -> Result<()> {
        let balance = ctx.accounts.acct.to_account_info().lamports();

        msg!("balance in Lamports is {}", balance);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ReadBalance<'info> {
    /// CHECK: although we read this account's balance, we don't do anything with the information
    pub acct: UncheckedAccount<'info>,
}