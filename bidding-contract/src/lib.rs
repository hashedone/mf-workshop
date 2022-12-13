#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use state::{COMISSION, HIGHEST_BID, IS_CLOSED, OWNER, TOKEN};

pub mod msg;
pub mod query;
pub mod state;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    HIGHEST_BID.save(deps.storage, &(info.sender.clone(), Uint128::zero()))?;

    let owner = msg
        .owner
        .map(|addr| deps.api.addr_validate(&addr))
        .transpose()?
        .unwrap_or(info.sender);

    OWNER.save(deps.storage, &owner)?;
    TOKEN.save(deps.storage, &msg.token)?;
    COMISSION.save(deps.storage, &msg.commission)?;

    IS_CLOSED.save(deps.storage, &false)?;

    let resp = Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("owner", &owner);

    Ok(resp)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        IsClosed {} => to_binary(&query::is_closed(deps)?),
        HighestBid {} => to_binary(&query::highest_bid(deps)?),
        Winner {} => to_binary(&query::winner(deps)?),
        TotalBid { addr } => to_binary(&query::total_bid(deps, addr)?),
        AllBids { .. } => unimplemented!(),
    }
}


