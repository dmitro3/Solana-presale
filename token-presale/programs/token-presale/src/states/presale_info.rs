use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct PresaleInfo {
    // Mint address of the presale token
    pub token_mint_address: Pubkey,

    // Mint address of the quote token
    pub sol_token: Pubkey,
    pub usdc_token: Pubkey,
    pub usdt_token: Pubkey,
    pub jup_token: Pubkey,

    // Softcap
    pub softcap_amount: u64,

    // Hardcap
    pub hardcap_amount: u64,

    // Total amount of presale tokens available in the presale, that is quote token
    pub deposit_token_amount: u64,
    pub deposit_usdt_token_amount: u64,
    pub deposit_usdc_token_amount: u64,
    pub deposit_jup_token_amount: u64,

    // Total amount of presale tokens sold during the presale, that is base token
    pub sold_token_amount: u64,

    // Start time of presale
    pub start_time: u64,

    // end time of presale
    pub end_time: u64,

    // Maximum token amound of presale token that an address can purchase
    pub max_token_amount_per_address: u64,

    // Quote token per presale token
    pub price_per_token: u64,

    // Presale is buable
    pub is_live: bool,

    // Vesting schedule
    pub vesting_schedule: u64,

    // Identifier,
    pub identifier: u8,

    // Authority of the presale
    pub authority: Pubkey,
    pub authority1: Pubkey,

    // Bump used when creating the PDA
    pub bump: u8,
}
