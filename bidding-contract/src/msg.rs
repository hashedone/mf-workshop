use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub token: String,
    pub owner: Option<String>,
    pub commission: Decimal,
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
    pub is_closed: bool,
}

#[cw_serde]
pub struct TotalBidResp {
    pub amount: Uint128,
}

#[cw_serde]
pub struct WinnerResp {
    pub winner: Option<Addr>,
}

#[cw_serde]
pub struct BidInfo {
    pub addr: Addr,
    pub amount: Uint128,
}

#[cw_serde]
pub struct AllBidsResp {
    pub bids: Vec<BidInfo>,
}
