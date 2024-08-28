use anchor_lang::prelude::*;

declare_id!("7fVJW6jrafX5ywXJzctzJwbT12S7h3dz5k7syX5SK4SR");

#[program]
pub mod day13 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        emit!(MyEvent { value: 42 });
        emit!(MySecondEvent { value: 3, message: "hello world".to_string() });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[event]
pub struct MyEvent {
    pub value: u64,
}

#[event]
pub struct MySecondEvent {
		pub value: u64,
    pub message: String,
}
