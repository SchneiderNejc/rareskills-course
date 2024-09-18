use anchor_lang::prelude::*;

// The program definition for Day33
use day33::program::Day33;

// the account where Day33 is storing the sum
use day33::BobData;

// Import BobAddOp from the correct module
use day33::cpi::accounts::BobAddOp;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod alice {
    use super::*;

    pub fn ask_day33_to_add(ctx: Context<AliceOp>, a: u64, b: u64) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.day33_program.to_account_info(),
            BobAddOp {
                bob_data_account: ctx.accounts.bob_data_account.to_account_info(),
            }
        );

        // Correctly reference the CPI function
        let res = day33::cpi::add_and_store(cpi_ctx, a, b);

        // return an error if the CPI failed
        if res.is_ok() {
            return Ok(());
        } else {
            return err!(Errors::CPIToBobFailed);
        }
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