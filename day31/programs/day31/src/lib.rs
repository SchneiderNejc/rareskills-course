use anchor_lang::prelude::*;

declare_id!("DZAhARBh8xs14dEYi6mQ6S79LPYGEghgqvAbCvvCgVsV");

#[program]
pub mod day31 {
    use super::*;

	pub fn foo(ctx: Context<Foo>) -> Result<()> {
		// we don't do anything with the account SomeAccount
		Ok(())
		}
}

#[derive(Accounts)]
pub struct Foo<'info> {
	some_account: Account<'info, SomeAccount>, // When using Account type, must have callable init.
}

#[account]
pub struct SomeAccount {}
