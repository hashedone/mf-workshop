use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_storage_plus::{Item, Map};

pub const OWNER: Item<Addr> = Item::new("owner");
pub const TOKEN: Item<String> = Item::new("token");
pub const COMISSION: Item<Decimal> = Item::new("comission");

pub const BIDS: Map<&Addr, Uint128> = Map::new("bids");
pub const HIGHEST_BID: Item<(Addr, Uint128)> = Item::new("highest_bid");

pub const IS_CLOSED: Item<bool> = Item::new("is_closed");
