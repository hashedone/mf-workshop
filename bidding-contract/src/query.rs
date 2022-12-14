use cosmwasm_std::{Addr, Deps, Order, StdResult};
use cw_storage_plus::Bounder;

use crate::msg::{AllBidsResp, BidInfo, IsClosedResp, TotalBidResp, WinnerResp};
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
        false => None,
        true => HIGHEST_BID.load(deps.storage)?.0.into(),
    };

    Ok(WinnerResp { winner })
}

pub fn total_bid(deps: Deps, addr: String) -> StdResult<TotalBidResp> {
    let addr = Addr::unchecked(addr);
    let amount = BIDS.may_load(deps.storage, &addr)?.unwrap_or_default();
    Ok(TotalBidResp { amount })
}

pub fn all_bids(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<AllBidsResp> {
    let limit = limit.unwrap_or(10).min(30);

    let start_after = start_after.map(Addr::unchecked);
    let start_after = start_after.as_ref().and_then(Bounder::exclusive_bound);

    let bids = BIDS
        .range(deps.storage, start_after, None, Order::Ascending)
        .map(|item| {
            let (addr, amount) = item?;
            Ok(BidInfo { addr, amount })
        })
        .take(limit as usize)
        .collect::<StdResult<_>>()?;

    Ok(AllBidsResp { bids })
}
