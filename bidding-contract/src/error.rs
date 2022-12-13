use cosmwasm_std::{StdError, Uint128};
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("{0}")]
    Payment(#[from] PaymentError),
    #[error("Bid not high enough. Highest bid: {highest}, senders total bid: {total}")]
    BidTooLow { total: Uint128, highest: Uint128 },
    #[error("Bidding is closed")]
    BiddingClosed,
    #[error("Unauthorized")]
    Unauthorized,
}
