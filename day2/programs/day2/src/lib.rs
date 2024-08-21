use anchor_lang::prelude::*;

declare_id!("EQiStVkyD9fxMF2q2D3izbnSZEfRT8vwuRTLmyxymx6b");

#[program]
pub mod day2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, a: u64, b: u64, message: String) -> Result<()> {
        msg!("You said {:?}", message);
        msg!("You sent {} and {}", a, b);
        Ok(())
    }

    pub fn array(ctx: Context<Initialize>, arr: Vec<u64>) -> Result<()> {
        msg!("Your array {:?}", arr);
        Ok(())
    }

    pub fn add(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let result: u64 = a + b;
        msg!("Result of addition: {}", result);
        Ok(())
    }

    pub fn subtract(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let result: u64 = a - b;
        msg!("Result of subtraction: {}", result);
        Ok(())
    }

    pub fn multiply(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let result: u64 = a * b;
        msg!("Result of multiplication: {}", result);
        Ok(())
    }

    pub fn divide(ctx: Context<Initialize>, a: f64, b: f64) -> Result<()> {
        let result: f64 = a / b;
        msg!("Result of division: {}", result);
        Ok(())
    }

    pub fn sqrt(ctx: Context<Initialize>, a: f64) -> Result<()> {
        msg!("Result of square root: {}",  a.sqrt());
        Ok(())
    }

    pub fn log10(ctx: Context<Initialize>, a: f64) -> Result<()> {
        msg!("Result of lof 10: {}", a.log10());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
