use anchor_lang::prelude::*;

declare_id!("FinKTpyeRA8as4H4jbfKKnbcaj94Lwiz9FJxd391DwKk");

// @note Replace with your wallet's public key
const OWNER: &str = "6ZJDfSVjffvRYbtpFF33PSeYWNYJnbtMKiwWVt1YRjZW";

#[program]
pub mod day14 {
    use super::*;

    // ------------------ Multiple signers ------------------

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // @dev Signer doesen't have tu be mutable.
        let the_signer1: &mut Signer = &mut ctx.accounts.signer1;

        msg!("The signer1: {:?}", *the_signer1.key);
        Ok(())
    }

    pub fn two_signers(ctx: Context<Initialize>) -> Result<()> {
        let the_signer1: &mut Signer = &mut ctx.accounts.signer1;
        let the_signer2: &mut Signer = &mut ctx.accounts.signer2;

        msg!("The signer1: {:?}", *the_signer1.key);
        msg!("The signer2: {:?}", *the_signer2.key);

        Ok(())
    }

    pub fn three_signers(ctx: Context<Initialize>) -> Result<()> {
        let the_signer1: &mut Signer = &mut ctx.accounts.signer1;
        let the_signer2: &mut Signer = &mut ctx.accounts.signer2;
        let the_signer3: &mut Signer = &mut ctx.accounts.signer3;

        msg!("The signer1: {:?}", *the_signer1.key);
        msg!("The signer2: {:?}", *the_signer2.key);
        msg!("The signer3: {:?}", *the_signer3.key);

        Ok(())
    }

    // ------------------ Only Owner ------------------

    #[access_control(check(&ctx))]
    pub fn only_owner(ctx: Context<OnlyOwner>) -> Result<()> {
        // Function logic...

        msg!("Holla, I'm the owner.");
        Ok(())
    }
}

fn check(ctx: &Context<OnlyOwner>) -> Result<()> {
    // Check if signer === owner
    require_keys_eq!(
        ctx.accounts.signer_account.key(),
        OWNER.parse::<Pubkey>().unwrap(),
        OnlyOwnerError::NotOwner
    );

    Ok(())
}

// Signer related structs
#[derive(Accounts)]
pub struct Initialize<'info> {
    pub signer1: Signer<'info>,
    pub signer2: Signer<'info>,
    pub signer3: Signer<'info>,
}

// Only owner related structs
#[derive(Accounts)]
pub struct OnlyOwner<'info> {
    signer_account: Signer<'info>,
}

// An enum for custom error codes
#[error_code]
pub enum OnlyOwnerError {
    #[msg("Only owner can call this function!")]
    NotOwner,
}