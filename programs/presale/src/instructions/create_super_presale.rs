use anchor_lang::prelude::*;

use crate::constants::PRESALE_SEED;
use crate::errors::PresaleError;
use crate::state::SuperPresaleInfo;

// Edit the details for a presale
pub fn create_super_presale(ctx: Context<CreateSuperPresale>, fee: u64) -> Result<()> {
    let super_presale = &mut ctx.accounts.super_presale_info;
    let authority = &ctx.accounts.authority;
    let bot = &ctx.accounts.bot;

    if fee > 10000 {
        return Err(PresaleError::InvalidFee.into());
    }
    super_presale.fee = fee;
    super_presale.authority = authority.key();
    super_presale.presale_count = 0;
    super_presale.bot = bot.to_account_info().key();
    Ok(())
}

#[derive(Accounts)]
pub struct CreateSuperPresale<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds=[PRESALE_SEED],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<SuperPresaleInfo>(),
    )]
    pub super_presale_info: Box<Account<'info, SuperPresaleInfo>>,

    pub bot: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}
