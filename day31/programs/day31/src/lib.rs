use anchor_lang::prelude::*;

declare_id!("DZAhARBh8xs14dEYi6mQ6S79LPYGEghgqvAbCvvCgVsV");

#[program]
pub mod day31 {
    use super::*;

	pub fn hello(ctx: Context<Hello>) -> Result<()> {
		let lamports = ctx.accounts.signer.lamports();
		let address = &ctx.accounts
			.signer
			.signer_key().unwrap();
		msg!(
			"hello {:?} you have {} lamports",
			address,
			lamports
		);
		Ok(())
    }
}

#[derive(Accounts)]
pub struct Hello<'info> {
	pub signer: Signer<'info>,
}