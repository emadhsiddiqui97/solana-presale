use anchor_lang::prelude::*;

use crate::constants::PRESALE_SEED;
use crate::errors::PresaleError;
use crate::state::SuperPresaleInfo;

// Edit the details for a presale
pub fn update_fees(ctx: Context<UpdateSuperPresale>, fees: u64) -> Result<()> {
    let super_presale_info = &mut ctx.accounts.super_presale_info;
    if fees > 10000 {
        return Err(PresaleError::InvalidFee.into());
    }
    super_presale_info.fee = fees;
    super_presale_info.bot = ctx.accounts.bot.to_account_info().key();

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateSuperPresale<'info> {
    // presale_detils account
    #[account(
        mut,
        seeds = [PRESALE_SEED],
        bump
    )]
    pub super_presale_info: Box<Account<'info, SuperPresaleInfo>>,

    // Set the authority to the transaction signer
    #[account(
        mut,
        constraint = authority.key() == super_presale_info.authority
    )]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub bot: AccountInfo<'info>,

    // Must be included when initializing an account
    pub system_program: Program<'info, System>,
}
