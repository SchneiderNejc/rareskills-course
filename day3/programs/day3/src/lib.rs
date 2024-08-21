use anchor_lang::prelude::*;

declare_id!("J4JFzPXckHctGsYtDR1gPueCoaXycNCJ3tei533XPB2");

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

    pub fn mul(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let result = a * b;
        msg!("Multipled is {}", result);
        Ok(())
    }

    pub fn div(ctx: Context<Initialize>, a: f64, b: f64) -> Result<()> {
        let result = a / b;
        msg!("Divided is {}", result);
        Ok(())
    }

    pub fn modulus(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let result = a % b;
        msg!("Modulus is {}", result);
        Ok(())
    }


}

#[derive(Accounts)]
pub struct Initialize {}
