use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("2wmxy1ysvSbM5DPH3s4TYdXLLsw3zUhDgnjAwszpZR2M");

#[program]
pub mod day16 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
