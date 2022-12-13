use cosmwasm_std::{coins, ensure, BankMsg, DepsMut, MessageInfo, Response};
use cw_utils::must_pay;

use crate::error::ContractError;
use crate::state::{BIDS, COMISSION, HIGHEST_BID, IS_CLOSED, OWNER, TOKEN};

pub fn bid(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    ensure!(!IS_CLOSED.load(deps.storage)?, ContractError::BiddingClosed);

    let denom = TOKEN.load(deps.storage)?;

    let bid = must_pay(&info, &denom)?;
    let comission = COMISSION.load(deps.storage)?;
    let comission = bid * comission;
    let bid = bid - comission;

    let total = BIDS
        .may_load(deps.storage, &info.sender)?
        .unwrap_or_default();

    let (_, highest) = HIGHEST_BID.load(deps.storage)?;

    ensure!(
        total + bid > highest,
        ContractError::BidTooLow { highest, total }
    );

    let total = total + bid;

    BIDS.save(deps.storage, &info.sender, &total)?;
    HIGHEST_BID.save(deps.storage, &(info.sender.clone(), total))?;

    let owner = OWNER.load(deps.storage)?;
    let commission_msg = BankMsg::Send {
        to_address: owner.into(),
        amount: coins(comission.u128(), denom),
    };

    let resp = Response::new()
        .add_attribute("action", "bid")
        .add_attribute("sender", info.sender.as_str())
        .add_attribute("total_bid", total)
        .add_message(commission_msg);

    Ok(resp)
}

pub fn close(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let owner = OWNER.load(deps.storage)?;
    ensure!(info.sender == owner, ContractError::Unauthorized);

    IS_CLOSED.save(deps.storage, &true)?;

    let (winner, bid) = HIGHEST_BID.load(deps.storage)?;
    let denom = TOKEN.load(deps.storage)?;

    BIDS.remove(deps.storage, &winner);

    let transfer_msg = BankMsg::Send {
        to_address: owner.into(),
        amount: coins(bid.u128(), denom),
    };

    let resp = Response::new()
        .add_attribute("action", "close")
        .add_attribute("sender", info.sender.as_str())
        .add_attribute("winner", winner)
        .add_message(transfer_msg);

    Ok(resp)
}

pub fn retract(
    deps: DepsMut,
    info: MessageInfo,
    receiver: Option<String>,
) -> Result<Response, ContractError> {
    ensure!(IS_CLOSED.load(deps.storage)?, ContractError::BiddingOpen);

    let total = BIDS
        .may_load(deps.storage, &info.sender)?
        .unwrap_or_default();

    BIDS.remove(deps.storage, &info.sender);

    let receiver = receiver
        .map(|addr| deps.api.addr_validate(&addr))
        .transpose()?
        .unwrap_or_else(|| info.sender.clone());

    let denom = TOKEN.load(deps.storage)?;
    let transfer_msg = BankMsg::Send {
        to_address: receiver.into(),
        amount: coins(total.u128(), denom),
    };

    let resp = Response::new()
        .add_attribute("action", "retract")
        .add_attribute("sender", info.sender.as_str())
        .add_message(transfer_msg);

    Ok(resp)
}
