use anchor_spl::token_2022::{transfer_checked, Token2022, TransferChecked};
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use {anchor_lang::prelude::*, anchor_spl::associated_token};

use crate::constants::{PRESALE_SEED, USER_SEED};
use crate::errors::PresaleError;
use crate::state::{PresaleInfo, UserInfo};

pub fn claim_token(ctx: Context<ClaimToken>, bump: u8) -> Result<()> {
    let presale_info = &mut ctx.accounts.presale_info;
    let presale_auth = &mut ctx
        .accounts
        .presale_authority
        .to_account_info()
        .key
        .as_ref();

    let cur_timestamp = u64::try_from(Clock::get()?.unix_timestamp).unwrap();

    // get time and compare with start and end time
    if presale_info.end_time > cur_timestamp * 1000 {
        msg!("current time: {}", cur_timestamp);
        msg!("presale end time: {}", presale_info.end_time);
        msg!("Presale not ended yet.");
        return Err(PresaleError::PresaleNotEnded.into());
    }

    let user_info = &mut ctx.accounts.user_info;
    let claim_amount = user_info.buy_token_amount;
    msg!("claim amount: {}", claim_amount);

    // msg!("presale end time: {}", presale_info.end_time);
    // msg!("presale start_time: {}", presale_info.start_time);

    msg!(
        "Transferring presale tokens to buyer {}...",
        &ctx.accounts.buyer.key()
    );
    msg!(
        "Mint: {}",
        &ctx.accounts
            .presale_token_mint_account
            .to_account_info()
            .key()
    );
    msg!(
        "From Token Address: {}",
        &ctx.accounts
            .presale_presale_token_associated_token_account
            .key()
    );
    msg!(
        "To Token Address: {}",
        &ctx.accounts
            .buyer_presale_token_associated_token_account
            .key()
    );
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx
                    .accounts
                    .presale_presale_token_associated_token_account
                    .to_account_info(),
                to: ctx
                    .accounts
                    .buyer_presale_token_associated_token_account
                    .to_account_info(),
                authority: ctx.accounts.presale_info.to_account_info(),
                mint: ctx.accounts.presale_token_mint_account.to_account_info(),
            },
            &[&[PRESALE_SEED, presale_auth, &[bump]][..]],
        ),
        claim_amount,
        9,
    )?;

    user_info.buy_token_amount = 0;
    user_info.claim_time = cur_timestamp;
    msg!("All claimed presale tokens transferred successfully.");

    Ok(())
}

#[derive(Accounts)]
pub struct ClaimToken<'info> {
    // Presale token accounts
    #[account(mut)]
    pub presale_token_mint_account: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = presale_token_mint_account,
        associated_token::authority = buyer,
        associated_token::token_program = token_program
    )]
    pub buyer_presale_token_associated_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = presale_token_mint_account,
        associated_token::authority = presale_info,
        associated_token::token_program = token_program
    )]
    pub presale_presale_token_associated_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [USER_SEED, buyer.key().as_ref(), presale_info.authority.key().as_ref()],
        bump
    )]
    pub user_info: Box<Account<'info, UserInfo>>,

    #[account(
        mut,
        seeds = [PRESALE_SEED, presale_authority.to_account_info().key().as_ref()],
        bump
    )]
    pub presale_info: Box<Account<'info, PresaleInfo>>,

    #[account(mut)]
    pub presale_authority: AccountInfo<'info>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}
