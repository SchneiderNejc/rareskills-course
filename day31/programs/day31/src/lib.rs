use anchor_lang::prelude::*;

declare_id!("DZAhARBh8xs14dEYi6mQ6S79LPYGEghgqvAbCvvCgVsV");

#[program]
pub mod day31 {
    use super::*;

	pub fn foo(ctx: Context<Foo>) -> Result<()> {
		let data = &ctx.accounts.some_account.try_borrow_data()?;
		msg!("{:?}", data);
		Ok(())
	}
}

#[derive(Accounts)]
pub struct Foo<'info> {
	/// CHECK: we are just printing the data
	some_account: AccountInfo<'info>,
}