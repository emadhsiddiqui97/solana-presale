use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("GCEBpvH5xJByeJqW1XX2kxWJFG8SAttk9KQSpEQhVdry");

#[program]
pub mod palm_presale {
    use super::*;

    pub fn create_presale(
        ctx: Context<CreatePresale>,
        // token_mint_address: Pubkey,
        // quote_token_mint_address: Pubkey,
        softcap_amount: u64,
        hardcap_amount: u64,
        max_token_amount_per_address: u64,
        price_per_token: u64,
        start_time: u64,
        end_time: u64,
        // identifier: u8
    ) -> Result<()> {
        return create_presale::create_presale(
            ctx,
            // token_mint_address,
            // quote_token_mint_address,
            softcap_amount,
            hardcap_amount,
            max_token_amount_per_address,
            price_per_token,
            start_time,
            end_time,
            // identifier,
        );
    }

    pub fn update_presale(
        ctx: Context<UpdatePresale>,
        max_token_amount_per_address: u64,
        price_per_token: u64,
        softcap_amount: u64,
        hardcap_amount: u64,
        start_time: u64,
        end_time: u64,
    ) -> Result<()> {
        return update_presale::update_presale(
            ctx,
            max_token_amount_per_address,
            price_per_token,
            softcap_amount,
            hardcap_amount,
            start_time,
            end_time,
        );
    }

    pub fn deposit_token(ctx: Context<DepositToken>, amount: u64) -> Result<()> {
        return deposit_token::deposit_token(ctx, amount);
    }

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
        return start_presale::start_presale(
            ctx,
            start_time,
            end_time,
            token_mint_address,
            softcap_amount,
            hardcap_amount,
            max_token_amount_per_address,
            price_per_token,
        );
    }
    //  token_amount: u64
    pub fn buy_token(ctx: Context<BuyToken>, quote_amount: u64) -> Result<()> {
        return buy_token::buy_token(ctx, quote_amount);
    }

    pub fn claim_token(ctx: Context<ClaimToken>, bump: u8) -> Result<()> {
        return claim_token::claim_token(ctx, bump);
    }

    pub fn withdraw_sol(ctx: Context<WithdrawSol>, amount: u64, bump: u8) -> Result<()> {
        return withdraw_sol::withdraw_sol(ctx, amount, bump);
    }

    pub fn withdraw_token(ctx: Context<WithdrawToken>, amount: u64, bump: u8) -> Result<()> {
        return withdraw_token::withdraw_token(ctx, amount, bump);
    }
    pub fn update_hardcap(ctx: Context<UpdateHardcap>) -> Result<()> {
        return update_hardcap::update_hardcap(ctx);
    }
    pub fn claim_token_test(ctx: Context<ClaimTokenTest>) -> Result<()> {
        return claim_token_test::claim_token_test(ctx);
    }
    // pub fn get_presale_info(ctx: Context<GetPresaleInfo>) -> Result<()> {
    //     return get_presale_info(ctx);
    // }

    pub fn create_presale_super_admin(ctx: Context<CreateSuperPresale>, fee: u64) -> Result<()> {
        return create_super_presale::create_super_presale(ctx, fee);
    }
    pub fn update_fees(ctx: Context<UpdateSuperPresale>, fee: u64) -> Result<()> {
        return update_fees::update_fees(ctx, fee);
    }
}

#[derive(Accounts)]
pub struct Initialize {}
