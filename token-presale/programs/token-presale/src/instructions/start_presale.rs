use anchor_lang::prelude::*;

use crate::states::PresaleInfo;
use crate::constants::{ PRESALE_SEED };

// Edit the details for a presale
pub fn start_presale(ctx: Context<StartPresale>, start_time: u64, identifier: u8) -> Result<()> {
    let presale_info = &mut ctx.accounts.presale_info;
    presale_info.start_time = start_time;
    presale_info.identifier = identifier;
    presale_info.is_live = true;
    Ok(())
}

#[derive(Accounts)]
#[instruction(
    start_time: u64,
    identifier: u8
)]
pub struct StartPresale<'info> {
    #[account(
        mut,
        seeds = [PRESALE_SEED, authority.key().as_ref(), [identifier].as_ref()],
        bump = presale_info.bump,
    )]
    pub presale_info: Box<Account<'info, PresaleInfo>>,

    #[account(mut)]
    pub authority: Signer<'info>,
}
