use anchor_lang::prelude::*;
use crate::constants::PRESALE_SEED;
use crate::constants::PRESALE_VAULT;
use crate::state::PresaleInfo;
use solana_program::clock::Clock;
// use crate::state::PresaleInfo;
// use crate::state::UserInfo;

// Edit the details for presale
pub fn start_presale(
    ctx: Context<StartPresale>,
    start_time: u64,
    end_time: u64,
    token_mint_address: Pubkey,
    softcap_amount: u64,
    hardcap_amount: u64,
    max_token_amount_per_address: u64,
    price_per_token: u64,
) -> Result<()> {
    let cur_timestamp = u64::try_from(Clock::get()?.unix_timestamp).unwrap();
    msg!("start_time: {}, end_time:{},", start_time, end_time);
    // let current_timestamp = clock.unix_timestamp;
    // msg!("Current Timestamp: {}", current_timestamp);
    let presale = &mut ctx.accounts.presale_info;

    // Set the presale details
    presale.token_mint_address = token_mint_address;
    presale.is_live = true;
    presale.start_time = start_time;
    presale.end_time = end_time;

    presale.softcap_amount = softcap_amount;
    presale.hardcap_amount = hardcap_amount;
    presale.deposit_token_amount = 0;
    presale.sold_token_amount = 0;
    presale.max_token_amount_per_address = max_token_amount_per_address;
    presale.price_per_token = price_per_token;

    // if softcap_amount > 0 {
    //     presale.is_soft_capped = true;
    // } else {
    //     presale.is_soft_capped = false;
    // }

    presale.is_soft_capped = false;
    presale.is_hard_capped = false;
    // if hardcap_amount > 0 {
    //     presale.is_hard_capped = true;
    // } else {
    //     presale.is_hard_capped = false;
    // }

    msg!("presale lamports: {}", presale.get_lamports());
    msg!("current time: {}", cur_timestamp * 1000);
    msg!("hard cap: {}", presale.hardcap_amount);
    msg!("soft cap: {}", presale.softcap_amount);
    msg!(
        "Presale has started for token: {} at the time: {}",
        presale.token_mint_address,
        start_time
    );
    Ok(())
}

#[derive(Accounts)]
pub struct StartPresale<'info> {
    #[account(
        mut,
        seeds = [PRESALE_SEED, authority.key().as_ref()],
        bump
    )]
    pub presale_info: Box<Account<'info, PresaleInfo>>,

    #[account(
        mut,
        constraint = authority.key() == presale_info.authority
    )]
    pub authority: Signer<'info>,
}
