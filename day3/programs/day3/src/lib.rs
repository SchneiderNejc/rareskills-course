use anchor_lang::prelude::*;

declare_id!("AZTSRipUXJrGU1YQsMUMpMcq61k154Axt3H3ZLM6fh65");

#[program]
pub mod day3 {
    use super::*;

    pub fn boaty_mc_boatface(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
