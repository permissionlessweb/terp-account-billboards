use cosmwasm_std::{Coin, Instantiate2AddressError, StdError, Timestamp, Uint128};
use cw_controllers::{AdminError, HookError};
use cw_ownable::OwnershipError;
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    PaymentError(#[from] PaymentError),

    #[error("{0}")]
    Admin(#[from] AdminError),

    #[error("{0}")]
    OwnershipError(#[from] OwnershipError),

    #[error("{0}")]
    Instantiate2AddressError(#[from] Instantiate2AddressError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("MintingPaused")]
    MintingPaused {},

    #[error("Invalid reply ID")]
    InvalidReplyID {},

    #[error("Invalid name")]
    InvalidAccount {},

    #[error("Account too short")]
    AccountTooShort {},

    #[error("Account too long")]
    AccountTooLong {},

    #[error("Incorrect delegation.  got: {got}, expected {expected}")]
    IncorrectDelegation { got: u128, expected: u128 },

    #[error("Incorrect payment, got: {got}, expected {expected}")]
    IncorrectPayment { got: u128, expected: u128 },

    #[error("InvalidTradingStartTime {0} < {1}")]
    InvalidTradingStartTime(Timestamp, Timestamp),

    #[error("MintingNotStarted")]
    MintingNotStarted {},

    #[error("Reply error")]
    ReplyOnSuccess {},

    #[error("Invalid Whitelist Type")]
    InvalidWhitelistType {},

    #[error("{0}")]
    Hook(#[from] HookError),

    #[error("AlreadySetup")]
    AlreadySetup {},

    #[error("CannotFinalizeBid")]
    CannotFinalizeBid {},

    #[error("NotApproved")]
    NotApproved {},

    #[error("UnauthorizedMinter")]
    UnauthorizedMinter {},

    #[error("InsufficientRenewalFunds: expected {expected}, actual {actual}")]
    InsufficientRenewalFunds { expected: Coin, actual: Coin },

    #[error("UnauthorizedOwner")]
    UnauthorizedOwner {},

    #[error("UnauthorizedOperator")]
    UnauthorizedOperator {},

    #[error("InvalidPrice")]
    InvalidPrice {},

    #[error("InvalidDuration")]
    InvalidDuration {},

    #[error("NoRenewalFund")]
    NoRenewalFund {},

    #[error("AskUnchanged")]
    AskUnchanged {},

    #[error("AskNotFound")]
    AskNotFound {},

    #[error("CannotProcessFutureRenewal")]
    CannotProcessFutureRenewal {},

    #[error("Cannot remove ask with existing bids")]
    ExistingBids {},

    #[error("PriceTooSmall: {0}")]
    PriceTooSmall(Uint128),

    #[error("InvalidListingFee: {0}")]
    InvalidListingFee(Uint128),

    #[error("Invalid finders fee bps: {0}")]
    InvalidTradingFeeBps(u64),

    #[error("Contract got an unexpected Reply")]
    UnexpectedReply(),
}
