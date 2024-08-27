use anchor_lang::prelude::*;

declare_id!("FK7sdzQhjoetR2oFDdLEyWfkechXFAfueUmy1L51Ldb4");

#[program]
pub mod day12 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
