use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod states;
pub mod instructions;

use instructions::*;

declare_id!("9tagFqyNtjjk3z8N3mcfGvKJshpZWEBGB3UDsv53yjRs");

#[program]
pub mod token_presale {
    use super::*;

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
        return create_presale::create_presale(
            ctx,
            token_mint_address,
            sol_token_mint_address,
            usdt_token_mint_address,
            usdc_token_mint_address,
            jup_token_mint_address,
            softcap_amount,
            hardcap_amount,
            max_token_amount_per_address,
            price_per_token,
            start_time,
            end_time,
            vesting_schedule,
            identifier
        );
    }

    pub fn start_presale(
        ctx: Context<StartPresale>,
        start_time: u64,
        identifier: u8
    ) -> Result<()> {
        return start_presale::start_presale(ctx, start_time, identifier);
    }

    pub fn buy_token(ctx: Context<BuyToken>, quote_amount: u64, identifier: u8) -> Result<()> {
        return buy_token::buy_token(ctx, quote_amount, identifier);
    }

    pub fn deposit_token(ctx: Context<DepositToken>, amount: u64, identifier: u8) -> Result<()> {
        return deposit_token::deposit_token(ctx, amount, identifier);
    }

    pub fn claim_token(ctx: Context<ClaimToken>, identifier: u8) -> Result<()> {
        return claim_token::claim_token(ctx, identifier);
    }

    pub fn transfer_tokens(ctx: Context<TransferTokens>, quantity: u64) -> Result<()> {
        return transfer_tokens::transfer_tokens(ctx, quantity);
    }

    pub fn withdraw_sol(ctx: Context<WithdrawSol>, identifier: u8) -> Result<()> {
        return withdraw_sol::withdraw_sol(ctx, identifier);
    }

    pub fn withdraw_token(ctx: Context<WithdrawToken>, identifier: u8) -> Result<()> {
        return withdraw_token::withdraw_token(ctx, identifier);
    }

    pub fn update_auth(ctx: Context<UpdateAuth>, identifier: u8) -> Result<()> {
        return update_auth::update_auth(ctx, identifier);
    }

    pub fn update_presale(
        ctx: Context<UpdatePresale>,
        max_token_amount_per_address: u64,
        price_per_token: u64,
        softcap_amount: u64,
        hardcap_amount: u64,
        start_time: u64,
        end_time: u64,
        identifier: u8
    ) -> Result<()> {
        return update_presale::update_presale(
            ctx,
            max_token_amount_per_address,
            price_per_token,
            softcap_amount,
            hardcap_amount,
            start_time,
            end_time,
            identifier
        );
    }
}
