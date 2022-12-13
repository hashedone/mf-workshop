use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    token: String,
    owner: Option<String>,
    commission: Decimal,
}

#[cw_serde]
pub enum ExecuteMsg {
    Bid {},
    Close {},
    Retract { receiver: Option<String> },
}

#[cw_serde]
pub enum QueryMsg {
    IsClosed {},
    HighestBid {},
    Winner {},
    TotalBid {
        addr: String,
    },
    AllBids {
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

#[cw_serde]
pub struct IsClosedResp {
    is_closed: bool,
}

#[cw_serde]
pub struct TotalBidResp {
    amount: Uint128,
}

#[cw_serde]
pub struct WinnerResp {
    winner: Option<Addr>,
}

#[cw_serde]
pub struct BidInfo {
    addr: Addr,
    amount: Uint128,
}

#[cw_serde]
pub struct AllBidsResp {
    bids: Vec<BidInfo>,
}