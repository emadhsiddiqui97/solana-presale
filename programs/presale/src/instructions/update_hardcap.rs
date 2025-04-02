use {
    anchor_lang::{prelude::*, system_program},
    anchor_spl::{
        token,
        associated_token,
    },
};

use solana_program::clock::Clock;

use crate::constants::PRESALE_VAULT;
use crate::state::PresaleInfo;
use crate::state::UserInfo;
use crate::constants::{PRESALE_SEED, USER_SEED};
use crate::errors::PresaleError;


pub fn update_hardcap(ctx: Context<UpdateHardcap>)->Result<()>{
let presale_info = &mut ctx.accounts.presale_info;
    presale_info.is_hard_capped = false;
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateHardcap<'info> {
    #[account(
        mut,
        seeds = [PRESALE_SEED, authority.key().as_ref()],
        bump
    )]
    pub presale_info: Box<Account<'info, PresaleInfo>>,

    #[account(mut)]
    pub authority: Signer<'info>,
}
