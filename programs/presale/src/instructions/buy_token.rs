use {
    anchor_lang::{prelude::*, system_program, Lamports},
    anchor_spl::token_2022,
    anchor_spl::{associated_token, token},
};

use solana_program::clock::Clock;

use crate::constants::PRESALE_VAULT;
use crate::constants::{PRESALE_SEED, USER_SEED};
use crate::errors::PresaleError;
use crate::state::PresaleInfo;
use crate::state::SuperPresaleInfo;
use crate::state::UserInfo;

//  token_amount: u64

// pub fn calculate_fee(amount: u64) -> Result<u64> {
//     // Check for potential overflow
//     let fee = amount
//         .checked_mul(FEE_BASIS_POINTS)
//         .ok_or("error multiplying")?
//         .checked_div(BASIS_POINTS_DENOMINATOR)
//         .ok_or("error dividing")?;

//     Ok(fee)
// }

pub fn buy_token(ctx: Context<BuyToken>, quote_amount: u64) -> Result<()> {
    const BASIS_POINTS_DENOMINATOR: u64 = 10000;
    let presale_info = &mut ctx.accounts.presale_info;
    let user_info = &mut ctx.accounts.user_info;
    let presale_vault = &mut ctx.accounts.presale_vault;
    let super_presale = &ctx.accounts.super_presale_info;
    let cur_timestamp = u64::try_from(Clock::get()?.unix_timestamp).unwrap();
    let admin_fee_amount: u64 = (quote_amount * super_presale.fee) / 10000;
    let isBot: bool = ctx.accounts.buyer.key() == super_presale.bot;

    // let admin_fee_amount = quote_amount
    //     .checked_mul(super_presale.fee)?
    //     // .ok_or("error multiplying")?
    //     .checked_div(BASIS_POINTS_DENOMINATOR)?;
    // // .ok_or("error dividing")?;

    let presale_amount: u64 = quote_amount - admin_fee_amount;

    msg!("quote amount: {}", quote_amount);
    msg!("fee: {}", super_presale.fee);
    msg!("presale admin amount: {}", admin_fee_amount);
    msg!("presale amount: {}", presale_amount);

    msg!("price per token: {}", presale_info.price_per_token);
    let token_amount: u64 =
        ((quote_amount * 1000) / presale_info.price_per_token) * (1000000000 / 1000);
    // get time and compare with start and end time
    if presale_info.start_time > cur_timestamp * 1000 {
        msg!("current time: {}", cur_timestamp);
        msg!("start time: {}", presale_info.start_time);
        return Err(PresaleError::PresaleNotStarted.into());
    }

    if presale_info.end_time < cur_timestamp * 1000 {
        msg!("start time: {}", presale_info.start_time);
        msg!("end time: {}", presale_info.end_time);
        msg!("current time: {}", cur_timestamp);
        return Err(PresaleError::PresaleEnded.into());
    }
    // msg!("start time: {}", presale_info.start_time);
    // msg!("end time: {}", presale_info.end_time);
    // msg!("current time: {}", cur_timestamp * 1000);

    msg!("token amount: {}", token_amount);
    msg!(
        "rest token amount in presale: {}",
        presale_info.deposit_token_amount - presale_info.sold_token_amount
    );
    msg!(
        "max token amount per address: {}",
        presale_info.max_token_amount_per_address
    );

    // compare the rest with the token_amount
    if token_amount > presale_info.deposit_token_amount - presale_info.sold_token_amount {
        msg!("token amount: {}", token_amount);
        msg!(
            "rest token amount in presale: {}",
            presale_info.deposit_token_amount - presale_info.sold_token_amount
        );
        return Err(PresaleError::InsufficientFund.into());
    }

    // limit the token_amount per address
    if presale_info.max_token_amount_per_address < (user_info.buy_token_amount + token_amount) {
        msg!(
            "max token amount per address: {}",
            presale_info.max_token_amount_per_address
        );
        msg!(
            "token amount to buy: {}",
            user_info.buy_token_amount + token_amount
        );
        return Err(PresaleError::InsufficientFund.into());
    }

    // limit the presale to hardcap
    if presale_info.is_hard_capped == true {
        return Err(PresaleError::HardCapped.into());
    }

    if isBot {
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.buyer.to_account_info(),
                    to: ctx.accounts.super_admin.to_account_info(),
                },
            ),
            quote_amount,
        );
    } else {
        // send SOL to contract and update the user info
        user_info.buy_time = cur_timestamp;
        user_info.buy_quote_amount = user_info.buy_quote_amount + quote_amount;
        user_info.buy_token_amount = user_info.buy_token_amount + token_amount;

        presale_info.sold_token_amount = presale_info.sold_token_amount + token_amount;

        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.buyer.to_account_info(),
                    to: ctx.accounts.super_admin.to_account_info(),
                },
            ),
            admin_fee_amount,
        );

        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.buyer.to_account_info(),
                    to: ctx.accounts.presale_authority.to_account_info(),
                },
            ),
            presale_amount,
        )?;
    }

    msg!("Presale tokens transferred successfully.");
    msg!("token amount: {}", token_amount);
    msg!(
        "rest token amount in presale: {}",
        presale_info.deposit_token_amount - presale_info.sold_token_amount
    );
    msg!(
        "max token amount per address: {}",
        presale_info.max_token_amount_per_address
    );
    msg!("token_amount: {}", user_info.buy_token_amount);

    // show softcap status
    //changed form presale_vault
    if presale_info.get_lamports() > presale_info.softcap_amount {
        presale_info.is_soft_capped = true;
        msg!("Presale is softcapped");
    }
    //changed form presale_vault
    msg!("get_lamports: {}", presale_info.get_lamports());
    // show hardcap status
    if presale_info.get_lamports() > presale_info.hardcap_amount {
        presale_info.is_hard_capped = true;
        msg!("Presale is hardcapped");
    }

    Ok(())
}

#[derive(Accounts)]
pub struct BuyToken<'info> {
    #[account(
        mut,
        seeds = [PRESALE_SEED, presale_authority.to_account_info().key().as_ref()],
        bump
    )]
    pub presale_info: Box<Account<'info, PresaleInfo>>,

    /// CHECK: This is not dangerous
    #[account(
        mut,
        constraint = presale_info.authority == presale_authority.to_account_info().key()
    )]
    pub presale_authority: AccountInfo<'info>,

    #[account(mut)]
    pub super_admin: AccountInfo<'info>,

    #[account(
        mut,
        seeds= [PRESALE_SEED],
        bump
    )]
    pub super_presale_info: Box<Account<'info, SuperPresaleInfo>>,

    #[account(
        init_if_needed,
        payer = buyer,
        space = 8 + std::mem::size_of::<UserInfo>(),
        seeds = [USER_SEED, buyer.key().as_ref(), presale_info.authority.key().as_ref()],
        bump
    )]
    pub user_info: Box<Account<'info, UserInfo>>,

    /// CHECK: This is not dangerous
    #[account(
        mut,
        seeds = [PRESALE_VAULT, presale_authority.to_account_info().key().as_ref()],
        bump
    )]
    pub presale_vault: AccountInfo<'info>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token_2022::Token2022>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}
