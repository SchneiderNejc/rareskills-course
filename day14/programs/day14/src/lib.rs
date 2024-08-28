use anchor_lang::prelude::*;

declare_id!("FinKTpyeRA8as4H4jbfKKnbcaj94Lwiz9FJxd391DwKk");

#[program]
pub mod day14 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // @dev Signer doesen't have tu be mutable.
        let the_signer1: &mut Signer = &mut ctx.accounts.signer1;

        msg!("The signer1: {:?}", *the_signer1.key);
        Ok(())
    }

    pub fn two_signers(ctx: Context<Initialize>) -> Result<()> {
        // @dev Signer doesen't have tu be mutable.
        let the_signer1: &mut Signer = &mut ctx.accounts.signer1;
        let the_signer2: &mut Signer = &mut ctx.accounts.signer2;

        msg!("The signer1: {:?}", *the_signer1.key);
        msg!("The signer2: {:?}", *the_signer2.key);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub signer1: Signer<'info>,
    pub signer2: Signer<'info>,
}