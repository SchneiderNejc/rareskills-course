use anchor_lang::prelude::*;

declare_id!("FK7sdzQhjoetR2oFDdLEyWfkechXFAfueUmy1L51Ldb4");

#[program]
pub mod day12 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // ------------------- Clock sysvar -------------------
        // Get the Clock sysvar
        let clock = Clock::get()?;

        // Log the details of the Clock sysvar
        msg!("clock: {:?}", clock);

        // ------------------- EpochSchedule sysvar -------------------
        // Get the EpochSchedule sysvar
        let epoch_schedule = EpochSchedule::get()?;

        // Log the details of the EpochSchedule sysvar
        msg!("EpochSchedule: {:?}", epoch_schedule);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
