use anchor_lang::prelude::*;

#[error_code]
pub enum PresaleError {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("Not allowed")]
    NotAllowed,
    #[msg("Math operation overflow")]
    MathOverflow,
    #[msg("Already marked")]
    AlreadyMarked,
    #[msg("Presale not started yet")]
    PresaleNotStarted,
    #[msg("Presale already ended")]
    PresaleEnded,
    #[msg("Token amount mismatch")]
    TokenAmountMismatch,
    #[msg("Insufficient Tokens")]
    InsufficientFund,
    #[msg("Presale not ended yet")]
    PresaleNotEnded,
    #[msg("Presale is wrong hardcapped")]
    HardCapped,
    #[msg("Invalid fees! please use a value between 0 and 100")]
    InvalidFee
}