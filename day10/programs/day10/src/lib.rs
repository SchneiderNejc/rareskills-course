use anchor_lang::prelude::*;

declare_id!("5L48GcaWe3LhhUjgnifDFS9L9eDQQyPmXYLdtYeKjirC");

#[program]
pub mod day10 {
    use super::*;

    pub fn add_two_numbers(_ctx: Context<Initialize>, x: u64, y: u64) -> Result<()> {
        // Call `add` function in calculate.rs
        let result = calculate::add(x, y);

        msg!("{} + {} = {}", x, y, result);
        Ok(())
    }
}

// @note mod caluclate deosen't have to be a separate file.
/* mod calculate {
    pub fn add(x: u64, y: u64) -> u64 {
		// Return the summation of x and y
        x + y
    }
} */

#[derive(Accounts)]
pub struct Initialize {}