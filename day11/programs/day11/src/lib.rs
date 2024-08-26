use anchor_lang::{prelude::*, solana_program::sysvar::recent_blockhashes::RecentBlockhashes};

declare_id!("6XS1mbtFU5T9yeWPLG1x51zyXxEuSqDu9ZaM8v8ZDRGJ");

#[program]
pub mod day11 {
    use super::*;
    use chrono::*;

    // ----------------- unix timestamp -----------------
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        let clock: Clock = Clock::get()?;
        msg!(
            "Block timestamp: {}",
            // Get block.timestamp
            clock.unix_timestamp,
        );
        Ok(())
    }

    // ----------------- day of the week -----------------
    pub fn get_day_of_the_week(
        _ctx: Context<Initialize>) -> Result<()> {

        let clock = Clock::get()?;
        let time_stamp = clock.unix_timestamp; // current timestamp

        let date_time = chrono::NaiveDateTime::from_timestamp_opt(time_stamp, 0).unwrap();
        let day_of_the_week = date_time.weekday();

        msg!("Week day is: {}", day_of_the_week);

        Ok(())
    }

    // ----------------- blockhash -----------------
    // @dev uncomment Initialize<'info> to make this work.
/*     pub fn blockhash(ctx: Context<Initialize>) -> Result<()> {
        // RECENT BLOCK HASHES
        let arr = [ctx.accounts.recent_blockhashes.clone()];
        let accounts_iter = &mut arr.iter();
        let sh_sysvar_info = next_account_info(accounts_iter)?;
        let recent_blockhashes = RecentBlockhashes::from_account_info(sh_sysvar_info)?;
        let data = recent_blockhashes.last().unwrap();

        msg!("The recent block hash is: {:?}", data.blockhash);
        Ok(())
    } */
}

#[derive(Accounts)]
pub struct Initialize {}
// @dev using this makes gethash() work, but the otherfunctions fail.
/* pub struct Initialize<'info> {
    /// CHECK: readonly
    pub recent_blockhashes: AccountInfo<'info>
} */
