use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Vesting {
    // Vesting is initialized
    pub is_initialized: bool,

    // User address
    pub user: Pubkey,

    // Vesting release time
    pub release_time: i64,

    // Vesting amount
    pub amount: u64,
}
