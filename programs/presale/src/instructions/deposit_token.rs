use anchor_lang::system_program;
// use solana_program::program::invoke;

use {
    anchor_lang::prelude::*,
    anchor_spl::{associated_token, token, token_2022, token_interface},
};

// use solana_program::rent::Rent;

use crate::constants::{PRESALE_SEED, PRESALE_VAULT, RENT_MINIMUM};
use crate::state::PresaleInfo;

pub fn deposit_token(ctx: Context<DepositToken>, amount: u64) -> Result<()> {
    let presale_info = &mut ctx.accounts.presale_info;
    msg!("deposit amount: {}", amount);
    // transfer token to the presaleAta
    msg!(
        "Mint: {}",
        &ctx.accounts.mint_account.to_account_info().key()
    );
    msg!(
        "From Token Address: {}",
        &ctx.accounts.from_associated_token_account.key()
    );
    msg!(
        "To Token Address: {}",
        &ctx.accounts.to_associated_token_account.key()
    );
    token_2022::transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token_2022::TransferChecked {
                from: ctx.accounts.from_associated_token_account.to_account_info(),
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.to_associated_token_account.to_account_info(),
                authority: ctx.accounts.from_authority.to_account_info(),
            },
        ),
        amount,
        9,
    )?;

    // transfer Sol to the presaleVault
    msg!(
        "From Wallet Address: {}",
        &ctx.accounts.from_associated_token_account.key()
    );
    msg!(
        "To Wallet Address: {}",
        &ctx.accounts.to_associated_token_account.key()
    );
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.admin.to_account_info(),
                to: ctx.accounts.presale_vault.to_account_info(),
            },
        ),
        RENT_MINIMUM,
    )?;

    presale_info.deposit_token_amount = presale_info.deposit_token_amount + amount;

    msg!("Tokens deposited successfully.");

    Ok(())
}

#[derive(Accounts)]
pub struct DepositToken<'info> {
    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, token_interface::Mint>,

    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = from_authority,
        associated_token::token_program = token_program //needed to create ata for spl-2022
    )]
    pub from_associated_token_account: InterfaceAccount<'info, token_interface::TokenAccount>,

    #[account(constraint = admin.key() == from_authority.key())]
    pub from_authority: Signer<'info>,

    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = mint_account,
        associated_token::authority = presale_info,
        associated_token::token_program = token_program
    )]
    pub to_associated_token_account: InterfaceAccount<'info, token_interface::TokenAccount>,

    #[account(
        // mut,
        init_if_needed,
        // space = 8 + std::mem::size_of::<token_interface::TokenAccount>(),
        space = 0,
        seeds = [PRESALE_VAULT, from_authority.key().as_ref()],
        bump,
        payer = admin,
        // owner = admin.key()
    )]
    pub presale_vault: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [PRESALE_SEED, from_authority.key().as_ref()],
        bump
    )]
    pub presale_info: Box<Account<'info, PresaleInfo>>,

    // #[account(mut)]
    // pub payer: AccountInfo<'info>,
    /// CHECK:
    #[account(
        mut,
        constraint = admin.key() == presale_info.authority.key()
    )]
    pub admin: AccountInfo<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token_2022::Token2022>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}
// init,
//         payer = admin,
//         associated_token::mint = mint_account,
//         associated_token::authority = presale_info,
