use {
    anchor_lang::{prelude::*, system_program},
    anchor_spl::{associated_token, token},
};

use solana_program::clock::Clock;

use crate::constants::PRESALE_VAULT;
use crate::constants::{PRESALE_SEED, USER_SEED};
use crate::errors::PresaleError;
use crate::state::PresaleInfo;
use crate::state::UserInfo;

pub fn update_hardcap(ctx: Context<GetPresaleInfo>) -> Result<()> {
    let presale_info = &mut ctx.accounts.presale_info;
    let user_info = &mut ctx.accounts.user_info;

    // msg!("token_mint_address: {}", presale_info.token_mint_address);
    // msg!("softcap_amount: {}", presale_info.softcap_amount);
    // msg!("hardcap_amount: {}", presale_info.hardcap_amount);
    msg!(
        "deposit_token_amount: {}",
        presale_info.deposit_token_amount
    );
    msg!("sold_token_amount: {}", presale_info.sold_token_amount);
    // msg!("start_time: {}", presale_info.start_time);
    // msg!("end_time: {}", presale_info.end_time);
    msg!(
        "max_token_amount_per_address: {}",
        presale_info.max_token_amount_per_address
    );
    msg!("price_per_token: {}", presale_info.price_per_token);
    // msg!("is_live: {}", presale_info.is_live);
    // msg!("authority: {}", presale_info.authority);
    // msg!("is_soft_capped: {}", presale_info.is_soft_capped);
    // msg!("is_hard_capped: {}", presale_info.is_hard_capped);
    msg!("buy_quote_amount: {}", user_info.buy_quote_amount);
    msg!("buy_token_amount: {}", user_info.buy_token_amount);
    msg!("buy_time: {}", user_info.buy_time);
    msg!("claim_time: {}", user_info.claim_time);

    Ok(())
}

#[derive(Accounts)]
pub struct GetPresaleInfo<'info> {
    #[account(
        mut,
        seeds = [PRESALE_SEED],
        bump
    )]
    pub presale_info: Box<Account<'info, PresaleInfo>>,
    // #[account(mut)]
    // pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [USER_SEED],
        bump
    )]
    pub user_info: Box<Account<'info, UserInfo>>,
}
