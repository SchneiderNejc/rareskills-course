use anchor_lang::prelude::*;

declare_id!("6jFD32aG6PwaYdLUgswa445xL7Lg3uKsLbApJv52CuNG");

#[program]
pub mod day6 {
    use super::*;

    pub fn age_checker(ctx: Context<Initialize>,
        age: u64) -> Result<()> {
    match age {
        1 => {
            // Code block executed if age equals 1
            msg!("The age is 1");
        },
        2 | 3 => {
            // Code block executed if age equals 2 or 3
            msg!("The age is either 2 or 3");
        },
        4..=6 => {
            // Code block executed if age is in the
            // range 4 to 6 (inclusive)
            msg!("The age is between 4 and 6");
        },
        _ => {
            // Code block executed for any other age
            msg!("The age is something else");
        }
        }
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize {}
