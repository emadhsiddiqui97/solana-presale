use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct SuperPresaleInfo {
    pub authority: Pubkey,
    pub fee: u64,
    pub presale_count: u64,
    pub bot: Pubkey,
}
