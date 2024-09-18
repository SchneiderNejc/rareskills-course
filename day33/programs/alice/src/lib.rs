use anchor_lang::prelude::*;

// The program definition for Day33
use day33::program::Day33;

// the account where Day33 is storing the sum
use day33::BobData;

// Import BobAddOp from the correct module
use day33::cpi::accounts::BobAddOp;

declare_id!("5PpNQHKWXq5qprfmgtBJgEfSyY4yVW6yaXJW9RozqC1F");

#[program]
pub mod alice {
    use super::*;

    pub fn ask_day33_to_add(ctx: Context<AliceOp>, a: u64, b: u64) -> Result<()> {
        // Calls the `bob_add_operation` function in bob program
        let res = day33::cpi::add_and_store(ctx.accounts.add_function_ctx(), a, b);

        if res.is_ok() {
            return Ok(());
        } else {
            return err!(Errors::CPIToBobFailed);
        }
    }
}

impl<'info> AliceOp<'info> {
    pub fn add_function_ctx(&self) -> CpiContext<'_, '_, '_, 'info, BobAddOp<'info>> {
        // The bob program we are interacting with
        let cpi_program = self.day33_program.to_account_info();

        // Passing the necessary account(s) to the `BobAddOp` account struct in Bob program
        let cpi_account = BobAddOp {
            bob_data_account: self.bob_data_account.to_account_info(),
        };

        // Creates a `CpiContext` object using the new method
        CpiContext::new(cpi_program, cpi_account)
    }
}

#[error_code]
pub enum Errors {
    #[msg("cpi to day33 failed")]
    CPIToBobFailed,
}

#[derive(Accounts)]
pub struct AliceOp<'info> {
    #[account(mut)]
    pub bob_data_account: Account<'info, BobData>,

    pub day33_program: Program<'info, Day33>,
}