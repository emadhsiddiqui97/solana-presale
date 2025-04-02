use anchor_lang::prelude::*;

use crate::constants::PRESALE_SEED;
use crate::state::PresaleInfo;
use crate::state::SuperPresaleInfo;

// Edit the details for a presale
pub fn create_presale(
    ctx: Context<CreatePresale>,
    // token_mint_address: Pubkey,
    softcap_amount: u64,
    hardcap_amount: u64,
    max_token_amount_per_address: u64,
    price_per_token: u64,
    start_time: u64,
    end_time: u64,
) -> Result<()> {
    let presale_info = &mut ctx.accounts.presale_info;
    let super_admin = &ctx.accounts.authority;
    let presale_authority = &ctx.accounts.presale_authority;
    let super_presale = &mut ctx.accounts.super_presale;

    super_presale.presale_count += 1;

    // presale_info.token_mint_address = token_mint_address;
    presale_info.softcap_amount = softcap_amount;
    presale_info.hardcap_amount = hardcap_amount;
    presale_info.deposit_token_amount = 0;
    presale_info.sold_token_amount = 0;
    presale_info.start_time = start_time;
    presale_info.end_time = end_time;
    presale_info.max_token_amount_per_address = max_token_amount_per_address;
    presale_info.price_per_token = price_per_token;
    presale_info.is_live = false;
    presale_info.authority = presale_authority.key();
    presale_info.is_soft_capped = false;
    presale_info.is_hard_capped = false;

    msg!(
        "Presale has created for token: {}",
        presale_info.token_mint_address
    );

    Ok(())
}

#[derive(Accounts)]
pub struct CreatePresale<'info> {
    #[account(
        init,
        seeds = [PRESALE_SEED, presale_authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<PresaleInfo>(),
    )]
    pub presale_info: Box<Account<'info, PresaleInfo>>,

    #[account(mut)]
    pub presale_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub super_presale: Box<Account<'info, SuperPresaleInfo>>,

    #[account(
        mut,
        constraint = authority.key() == super_presale.authority
    )]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
