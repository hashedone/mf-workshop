use cosmwasm_schema::cw_serde;
use cosmwasm_schema::QueryResponses;
use cosmwasm_std::{Addr, Decimal, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub token: String,
    pub owner: Option<String>,
    pub comission: Decimal,
}

#[cw_serde]
pub enum ExecuteMsg {
    Bid {},
    Close {},
    Retract { receiver: Option<String> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(IsClosedResp)]
    IsClosed {},
    #[returns(BidInfo)]
    HighestBid {},
    #[returns(WinnerResp)]
    Winner {},
    #[returns(TotalBidResp)]
    TotalBid { addr: String },
    #[returns(AllBidsResp)]
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
