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

// ------------------- INTERNAL FUNC -------------------
/* pub mod day10 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        // Call the internal_function from within its parent module
        some_internal_function::internal_function();
        Ok(())
    }

    pub mod some_internal_function {
        pub fn internal_function() {
            // Internal function logic...
        }
    }
}

// @note does not use 'pub' keyword
mod do_something {
    // Import day10 module
    use crate::day10;

    pub fn some_func_here() {
        // Call the internal_function from outside its parent module
        day10::some_internal_function::internal_function();

        // Do something else...
    }
} */


// ------------------- PRIVATE FUNC -------------------
/* pub mod day10 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        // Call the private_function from within its parent module
        some_function_function::private_function();

        Ok(())
    }

    pub mod some_function_function {
        // @note uses pub(in crate::day10)
        pub(in crate::day10) fn private_function() {
            // Private function logic...
        }
    }
} */



#[derive(Accounts)]
pub struct Initialize {}