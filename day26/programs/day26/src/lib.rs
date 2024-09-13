use anchor_lang::prelude::*;
use std::mem::size_of;
use anchor_lang::system_program;
use std::str::FromStr;


declare_id!("4RgrgZq6zuSggZtsAyYKgRhkuXDBa2VFvb8Bw7uQK5Wg");

#[program]
pub mod day26 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let initialized_pda = &mut ctx.accounts.pda;
        Ok(())
    }

    pub fn donate(ctx: Context<Donate>, amount: u64) -> Result<()> {
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.signer.to_account_info().clone(),
                to: ctx.accounts.pda.to_account_info().clone(),
            },
        );

        system_program::transfer(cpi_context, amount)?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        // in anchor 0.29 or higher, use the following syntax:
        // ctx.accounts.pda.sub_lamports(amount)?;
        // ctx.accounts.signer.add_lamports(amount)?;


        **ctx.accounts.pda.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.signer.to_account_info().try_borrow_mut_lamports()? += amount;
        Ok(())
    }
}



#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init, payer = signer, space=size_of::<Pda>() + 8, seeds=[], bump)]
    pub pda: Account<'info, Pda>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub pda: Account<'info, Pda>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, address = Pubkey::from_str("5jmigjgt77kAfKsHri3MHpMMFPo6UuiAMF19VdDfrrTj").unwrap())]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub pda: Account<'info, Pda>,
}

#[account]
pub struct Pda {}