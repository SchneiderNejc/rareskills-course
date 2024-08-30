use anchor_lang::prelude::*;

declare_id!("EDNggeqsBokQLBrhpLc3dnhvnnCuPbmXUyZdF9iBPThN");

#[program]
pub mod day15 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
