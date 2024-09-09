use anchor_lang::prelude::*;
use anchor_lang::system_program;


declare_id!("oPcAUxju1qYTkWngUDXYh25eFXkHkDLzatnk17ePKCy");

#[program]
pub mod day23 {
    use super::*;

    // @notice Send sol to a single recipient.
    pub fn send_sol(ctx: Context<SendSol>, amount: u64) -> Result<()> {

        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),

            system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.recipient.to_account_info(),
            }
        );

        let res = system_program::transfer(cpi_context, amount);

        if res.is_ok() {
            return Ok(());
        } else {
            return err!(Errors::TransferFailed);
        }
    }

    // @notice Split amount and send sol to two recipients.
    pub fn split_send_sol(ctx: Context<SplitSendSol>, amount: u64) -> Result<()> {

        let half_amount = amount / 2;

        let cpi_context1 = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),

            system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.recipient1.to_account_info()
            }
        );
        let mut res = system_program::transfer(cpi_context1, half_amount)?;

        // Transfer to recipient2
        let cpi_context2 = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.recipient2.to_account_info(),
            },
        );
        res = system_program::transfer(cpi_context2, half_amount)?;


        Ok(())
    }

    // @notice Split amount and sol to several recipients.
    // 'a, 'b, 'c are Rust lifetimes, ignore them for now
    pub fn split_sol<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, SplitSol<'info>>,
        amount: u64,
    ) -> Result<()> {

        let amount_each_gets = amount / ctx.remaining_accounts.len() as u64;
        let system_program = &ctx.accounts.system_program;

				// note the keyword `remaining_accounts`
        for recipient in ctx.remaining_accounts {
            let cpi_accounts = system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: recipient.to_account_info(),
            };
            let cpi_program = system_program.to_account_info();
            let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

            let res = system_program::transfer(cpi_context, amount_each_gets);
            if !res.is_ok() {
                return err!(Errors::TransferFailed);
            }
        }

        Ok(())
    }
}

#[error_code]
pub enum Errors {
    #[msg("transfer failed")]
    TransferFailed,
}

#[derive(Accounts)]
pub struct SendSol<'info> {
    /// CHECK: we do not read or write the data of this account
    #[account(mut)]
    recipient: UncheckedAccount<'info>,

    system_program: Program<'info, System>,

    #[account(mut)]
    signer: Signer<'info>,
}

// @todo Check if declaration of accounts was good.
#[derive(Accounts)]
pub struct SplitSendSol<'info> {

    #[account(mut)]
    /// CHECK: Splitting bill to 2 recipients.
    recipient1: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: Second recipient.
    recipient2: UncheckedAccount<'info>,

    system_program: Program<'info, System>,

    #[account(mut)]
    signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SplitSol<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}
