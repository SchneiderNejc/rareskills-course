use anchor_lang::prelude::*;
use std::mem::size_of;
use anchor_lang::system_program;

declare_id!("4RgrgZq6zuSggZtsAyYKgRhkuXDBa2VFvb8Bw7uQK5Wg");

#[program]
pub mod day26 {
    use super::*;

    pub fn initialize_keypair(ctx: Context<InitializeKeypair>) -> Result<()> {
        Ok(())
    }

    pub fn initialize_pda(ctx: Context<InitializePda>) -> Result<()> {
        Ok(())
    }

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn change_owner(ctx: Context<ChangeOwner>) -> Result<()> {
        let account_info = &mut ctx.accounts.my_storage.to_account_info();

				// assign is the function to transfer ownership
				account_info.assign(&system_program::ID);

				// we must erase all the data in the account or the transfer will fail
        let res = account_info.realloc(0, false);

        if !res.is_ok() {
            return err!(Err::ReallocFailed);
        }
        Ok(())
    }
}



#[error_code]
pub enum Err {
    #[msg("realloc failed")]
    ReallocFailed,
}

#[derive(Accounts)]
pub struct InitializeKeypair<'info> {
    #[account(init, payer = signer, space = 8)]
    keypair: Account<'info, Keypair>,
    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializePda<'info> {
    #[account(init, payer = signer, space = 8, seeds = [], bump)]
    pda: Account<'info, Pda>,
    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {

    #[account(init,
              payer = signer,
              space=size_of::<MyStorage>() + 8,
              seeds = [],
              bump)]
    pub my_storage: Account<'info, MyStorage>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ChangeOwner<'info> {
    #[account(mut)]
    pub my_storage: Account<'info, MyStorage>,
}

#[account]
pub struct MyStorage {
    x: u64,
}

#[account]
pub struct Keypair();

#[account]
pub struct Pda();
