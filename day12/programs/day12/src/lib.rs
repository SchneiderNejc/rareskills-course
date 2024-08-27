use anchor_lang::prelude::*;

declare_id!("2Kcrje5YbJVskthgEJVqzDcbAAu8CL2PuvoJiM6xpDJr");

#[program]
pub mod day12 {
    use super::*;
    // Following import is for the public-address-specific sysvars
    // use anchor_lang::solana_program::sysvar::{instructions, fees::Fees, recent_blockhashes::RecentBlockhashes};

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Clock sysvar
        let clock = Clock::get()?;
        msg!("clock: {:?}", clock);

        // EpochSchedule sysvar
        let epoch_schedule = EpochSchedule::get()?;
        msg!("EpochSchedule: {:?}", epoch_schedule);

        // Rent sysvar
        let rent_var = Rent::get()?;
        msg!("Rent {:?}", rent_var);

        // ------------------- Accessing Sysvars Using Sysvar Public Address -------------------

        // Sysvars we didn't test due changing pub struct Initialize
        // StakeHistory, RecentBlockhashes, Fees, Instruction, LastRestartSlot

/*      In the current version of Anchor, it is not feasible to access certain sysvars.
        These sysvars include EpochRewards, SlotHistory, and SlotHashes.
        When attempting to access these sysvars, it results to an error. */

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
