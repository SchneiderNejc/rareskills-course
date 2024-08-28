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
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer1: Signer<'info>,
}
