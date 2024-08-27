use anchor_lang::prelude::*;

declare_id!("FK7sdzQhjoetR2oFDdLEyWfkechXFAfueUmy1L51Ldb4");

#[program]
pub mod day12 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Get the Clock sysvar
        let clock = Clock::get()?;

        msg!(
            "clock: {:?}",
            // Retrieve all the details of the Clock sysvar
            clock
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
