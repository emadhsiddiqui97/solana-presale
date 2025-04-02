use {
    anchor_lang::prelude::*,
    anchor_spl::{associated_token, token},
};

use crate::constants::{PRESALE_SEED, USER_SEED};
// use crate::errors::PresaleError;
use crate::state::{PresaleInfo, UserInfo};

pub fn claim_token_test(ctx: Context<ClaimTokenTest>) -> Result<()> {
    let cur_timestamp = u64::try_from(Clock::get()?.unix_timestamp).unwrap();

    let presale_info = &mut ctx.accounts.presale_info;
    // let user_info = &mut ctx.accounts.user_info;
    // let presale_token_mint_account = &mut ctx.accounts.presale_token_mint_account;
    // let buyer_presale_token_associated_token_account =
    //     &mut ctx.accounts.buyer_presale_token_associated_token_account;
    // let presale_presale_token_associated_token_account =
    // &mut ctx.accounts.presale_presale_token_associated_token_account;
    // let presale_authority = &mut ctx.accounts.presale_authority;
    // let buyer = &mut ctx.accounts.buyer;
    // let rent = &mut ctx.accounts.rent;
    // let system_program = &mut ctx.accounts.system_program;
    // let token_program = &mut ctx.accounts.token_program;
    // let associated_token_program = &mut ctx.accounts.associated_token_program;

    // msg!("in claim token test");
    msg!(
        "presale info endtime: {}",
        ctx.accounts.presale_info.end_time
    );
    msg!(
        "presale info startTime: {}",
        ctx.accounts.presale_info.start_time
    );
    msg!("Current time: {}", cur_timestamp * 1000);
    msg!("presale is live: {}", ctx.accounts.presale_info.is_live);
    msg!("presale authority: {}", ctx.accounts.presale_info.authority);

    // msg!("user buy token amount: {}", ctx.accounts.user_info.buy_token_amount);
    // msg!("user claim token amount: {}", ctx.accounts.user_info.claim_token_amount);
    // msg!("user claim token amount: {}", ctx.accounts.user_info.claim_token_amount);

    // msg!("presale info: {}", ctx.accounts.presale_info);
    // msg!("claim amount: {:?}", format!(ctx.accounts.user_info.claim_amount));
    // msg!("presale_token_mint_account: {:?}", format!(ctx.accounts.presale_token_mint_account));
    // msg!("buyer_presale_token_associated_token_account: {:?}", format!(ctx.accounts.buyer_presale_token_associated_token_account));
    // msg!("presale_presale_token_associated_token_account: {:?}", format!(ctx.accounts.presale_presale_token_associated_token_account));
    // msg!("buyer: {:?}", format!(ctx.accounts.buyer));
    // msg!("rent: {:?}", format!(ctx.accounts.rent));
    // msg!("token_program: {:?}", format!(ctx.accounts.token_program));
    // msg!("associated_token_program: {:?}", format!(ctx.accounts.associated_token_program));

    // msg!(
    //     "Transferring presale tokens to buyer {}...",
    //     &ctx.accounts.buyer.key()
    // );
    // msg!(
    //     "Mint: {}",
    //     &ctx.accounts
    //         .presale_token_mint_account
    //         .to_account_info()
    //         .key()
    // );
    // msg!(
    //     "From Token Address: {}",
    //     &ctx.accounts
    //         .presale_presale_token_associated_token_account
    //         .key()
    // );
    // msg!(
    //     "To Token Address: {}",
    //     &ctx.accounts
    //         .buyer_presale_token_associated_token_account
    //         .key()
    // );
    Ok(())
}

// pub fn transfer_spl_tokens(ctx: Context<TransferSpl>, amount: u64) -> Result<()> {
//         let destination = &ctx.accounts.to_ata;
//         let source = &ctx.accounts.from_ata;
//         let token_program = &ctx.accounts.token_program;
//         let authority = &ctx.accounts.from;
//         // Transfer tokens from taker to initializer
//         let cpi_accounts = SplTransfer {
//             from: source.to_account_info().clone(),
//             to: destination.to_account_info().clone(),
//             authority: authority.to_account_info().clone(),
//         };
//         let cpi_program = token_program.to_account_info();
//         token::transfer(
//             CpiContext::new(cpi_program, cpi_accounts),
//             amount)?;
//         Ok(())
//     }

#[derive(Accounts)]
pub struct ClaimTokenTest<'info> {
    // Presale token accounts
    // #[account(mut)]
    // pub presale_token_mint_account: Box<Account<'info, token::Mint>>,

    // #[account(
    //     init_if_needed,
    //     payer = buyer,
    //     associated_token::mint = presale_token_mint_account,
    //     associated_token::authority = buyer,
    // )]
    // pub buyer_presale_token_associated_token_account: Box<Account<'info, token::TokenAccount>>,

    // #[account(
    //     mut,
    //     associated_token::mint = presale_token_mint_account,
    //     associated_token::authority = presale_info,
    // )]
    // pub presale_presale_token_associated_token_account: Box<Account<'info, token::TokenAccount>>,

    // #[account(
    //     mut,
    //     seeds = [USER_SEED],
    //     bump
    // )]
    // pub user_info: Box<Account<'info, UserInfo>>,
    #[account(
        mut,
        seeds = [PRESALE_SEED],
        bump
    )]
    pub presale_info: Box<Account<'info, PresaleInfo>>,
    // pub presale_authority: SystemAccount<'info>,

    // #[account(mut)]
    // pub buyer: Signer<'info>,

    // pub rent: Sysvar<'info, Rent>,
    // pub system_program: Program<'info, System>,
    // pub token_program: Program<'info, token::Token>,
    // pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}
//
// #[derive(Accounts)]
// pub struct TransferSpl<'info> {
//     pub from: Signer<'info>,
//     #[account(mut)]
//     pub from_ata: Account<'info, TokenAccount>,
//     #[account(mut)]
//     pub to_ata: Account<'info, TokenAccount>,
//     pub token_program: Program<'info, Token>,
// }
