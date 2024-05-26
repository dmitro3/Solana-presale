use anchor_lang::prelude::*;

declare_id!("CHKqP8wim3BczkSrsqh6ZKThLYTbEGA5nxDnEbXuZGwB");

#[program]
pub mod token_presale {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
