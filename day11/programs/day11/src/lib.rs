use anchor_lang::prelude::*;

declare_id!("6XS1mbtFU5T9yeWPLG1x51zyXxEuSqDu9ZaM8v8ZDRGJ");

#[program]
pub mod day11 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let clock: Clock = Clock::get()?;
        msg!(
            "Block timestamp: {}",
            // Get block.timestamp
            clock.unix_timestamp,
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
