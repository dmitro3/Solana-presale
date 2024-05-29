use anchor_lang::prelude::*;
use crate::states::PresaleInfo;
use crate::constants::{ PRESALE_SEED };

pub fn create_presale(
    ctx: Context<CreatePresale>,
    token_mint_address: Pubkey,
    sol_token_mint_address: Pubkey,
    usdt_token_mint_address: Pubkey,
    usdc_token_mint_address: Pubkey,
    jup_token_mint_address: Pubkey,
    softcap_amount: u64,
    hardcap_amount: u64,
    max_token_amount_per_address: u64,
    price_per_token: u64,
    start_time: u64,
    end_time: u64,
    vesting_schedule: u64,
    identifier: u8
) -> Result<()> {
    let presale_info = &mut ctx.accounts.presale_info;
    let authority = &ctx.accounts.authority;

    // Set the presale details to the parameters given
    presale_info.is_live = false;
    presale_info.token_mint_address = token_mint_address;
    presale_info.sol_token = sol_token_mint_address;
    presale_info.usdc_token = usdc_token_mint_address;
    presale_info.usdt_token = usdt_token_mint_address;
    presale_info.jup_token = jup_token_mint_address;
    presale_info.deposit_token_amount = 0;
    presale_info.deposit_usdt_token_amount = 0;
    presale_info.deposit_usdc_token_amount = 0;
    presale_info.deposit_jup_token_amount = 0;
    presale_info.sold_token_amount = 0;
    presale_info.start_time = start_time;
    presale_info.end_time = end_time;
    presale_info.softcap_amount = softcap_amount;
    presale_info.hardcap_amount = hardcap_amount;
    presale_info.max_token_amount_per_address = max_token_amount_per_address;
    presale_info.price_per_token = price_per_token;
    presale_info.identifier = identifier;
    presale_info.vesting_schedule = vesting_schedule;
    presale_info.authority = authority.key();
    presale_info.authority1 = authority.key();
    // presale_info.bump = *ctx.bumps.get().unwrap();
    msg!("Presale has created for token: {}", presale_info.token_mint_address);
    Ok(())
}

#[derive(Accounts)]
#[allow(clippy::too_many_arguments)]
#[instruction(
    token_mint_address: Pubkey,
    sol_token: Pubkey,
    usdc_token: Pubkey,
    usdt_token: Pubkey,
    jup_token: Pubkey,
    softcap_amount: u64,
    hardcap_amount: u64,
    start_time: u64,
    end_time: u64,
    max_token_amount_per_address: u64,
    price_per_token: u64,
    vesting_schedule: u64,
    identifier: u8,
)]
pub struct CreatePresale<'info> {
    // Initialize the presale_info accont
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<PresaleInfo>(),
        seeds = [PRESALE_SEED.as_ref(), authority.key().as_ref(), [identifier].as_ref()],
        bump
    )]
    pub presale_info: Box<Account<'info, PresaleInfo>>,

    // Set the authority to the transaction signer
    #[account(mut)]
    authority: Signer<'info>,

    // Must be included when initialzing an account
    pub system_program: Program<'info, System>,
}
