use anchor_lang::prelude::*;

declare_id!("8i1QGsm4LCAnF4HuPn4iwqtYHxTFPdVLi1y2HvAsT6Jf");

#[program]
pub mod day3 {
    use super::*;

    pub fn boaty_mc_boatface(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn add(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let result: u64 = a + b;
        msg!("Result of addition: {}", result);
        Ok(())
    }

    pub fn sub(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let difference = a - b;
        msg!("Difference is {}", difference);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
