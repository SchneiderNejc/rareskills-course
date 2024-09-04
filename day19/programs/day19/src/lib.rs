use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("AfD7NnFyUnHfoS8kvhK5MTvuLqEhVggEKnJmpKuKM4qG");

#[program]
pub mod day19 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>, key1: u64, key2: u64) -> Result<()> {
        Ok(())
    }

    pub fn initialize_second(ctx: Context<InitializeSecondMapping>, key: u64) -> Result<()> {
        Ok(())
    }

    pub fn set(ctx: Context<Set>, key1: u64, key2: u64, val: u64) -> Result<()> {
        ctx.accounts.val.value = val;
        Ok(())
    }

    pub fn set_second(ctx: Context<SetSecondMapping>, key: u64, val: u64) -> Result<()> {
        ctx.accounts.val.value = val;
        Ok(())
    }

}



#[derive(Accounts)]
#[instruction(key1: u64, key2: u64)]
pub struct Initialize<'info> {
    #[account(init,
        payer = signer,
        space = size_of::<Val>() + 8,
        seeds=[
                b"mapping1",  // Distinct identifier for the first mapping
                &key1.to_le_bytes(),
                &key2.to_le_bytes()
            ],
        bump)]
    val: Account<'info, Val>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(key: u64)]
pub struct InitializeSecondMapping<'info> {
    #[account(init,
        payer = signer,
        space = size_of::<Val>() + 8,
        seeds=[
                b"mapping2",  // Distinct identifier for the second mapping
                &key.to_le_bytes()
            ],
        bump)]
    val: Account<'info, Val>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(key1: u64, key2: u64)]
pub struct Set<'info> {
    #[account(mut, seeds=[
        b"mapping1",
        &key1.to_le_bytes(),
        &key2.to_le_bytes()],
        bump
    )]
    val: Account<'info, Val>,
}

#[derive(Accounts)]
#[instruction(key: u64)]
pub struct SetSecondMapping<'info> {
    #[account(mut, seeds=[b"mapping2", &key.to_le_bytes()], bump)]
    val: Account<'info, Val>,
}

#[account]
pub struct Val {
    value: u64,
}
