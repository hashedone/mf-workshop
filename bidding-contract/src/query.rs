use cosmwasm_std::{Addr, Deps, StdResult};

use crate::msg::{BidInfo, IsClosedResp, TotalBidResp, WinnerResp};
use crate::state::{BIDS, HIGHEST_BID, IS_CLOSED};

pub fn is_closed(deps: Deps) -> StdResult<IsClosedResp> {
    let is_closed = IS_CLOSED.load(deps.storage)?;
    Ok(IsClosedResp { is_closed })
}

pub fn highest_bid(deps: Deps) -> StdResult<BidInfo> {
    let (addr, amount) = HIGHEST_BID.load(deps.storage)?;
    Ok(BidInfo { addr, amount })
}

pub fn winner(deps: Deps) -> StdResult<WinnerResp> {
    let winner = match IS_CLOSED.load(deps.storage)? {
        true => None,
        false => HIGHEST_BID.load(deps.storage)?.0.into(),
    };

    Ok(WinnerResp { winner })
}

pub fn total_bid(deps: Deps, addr: String) -> StdResult<TotalBidResp> {
    let addr = Addr::unchecked(addr);
    let amount = BIDS.load(deps.storage, &addr)?;
    Ok(TotalBidResp { amount })
}
