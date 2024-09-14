use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod day27 {
    use super::*;

    pub fn increment(ctx: Context<Initialize>) -> Result<()> {
        let current_counter = ctx.accounts.my_pda.counter;
        ctx.accounts.my_pda.counter = current_counter + 1;
        Ok(())
    }
}



#[derive(Accounts)]
