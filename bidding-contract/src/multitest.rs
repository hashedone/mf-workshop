use cosmwasm_std::{Addr, Decimal, StdResult, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor};

use crate::msg::{BidInfo, InstantiateMsg, QueryMsg, TotalBidResp, WinnerResp};
use crate::{execute, instantiate, query};

pub struct CodeId(u64);

impl CodeId {
    pub fn store_code(app: &mut App) -> Self {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Self(app.store_code(Box::new(contract)))
    }

    #[track_caller]
    pub fn instantiate<'a>(
        &self,
        app: &mut App,
        sender: &str,
        token: &str,
        owner: impl Into<Option<&'a str>>,
        comission: Decimal,
    ) -> StdResult<Contract> {
        let owner = owner.into().map(str::to_owned);

        app.instantiate_contract(
            self.0,
            Addr::unchecked(sender),
            &InstantiateMsg {
                token: token.to_owned(),
                owner,
                comission,
            },
            &[],
            "Bidding contract",
            None,
        )
        .map_err(|e| e.downcast().unwrap())
        .map(Contract)
    }
}

pub struct Contract(Addr);

impl Contract {
    pub fn winner(&self, app: &App) -> StdResult<WinnerResp> {
        app.wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::Winner {})
    }

    pub fn highest_bid(&self, app: &App) -> StdResult<BidInfo> {
        app.wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::HighestBid {})
    }

    pub fn total_bid(&self, app: &App, addr: &str) -> StdResult<TotalBidResp> {
        app.wrap().query_wasm_smart(
            self.0.clone(),
            &QueryMsg::TotalBid {
                addr: addr.to_owned(),
            },
        )
    }
}

const STAR: &str = "star";

#[test]
fn flow() {
    let owner = "onwer";
    let bidder1 = "bidder1";
    let bidder2 = "bidder2";

    let mut app = App::default();

    let code_id = CodeId::store_code(&mut app);
    let contract = code_id
        .instantiate(&mut app, owner, STAR, None, Decimal::percent(5))
        .unwrap();

    assert_eq!(contract.winner(&app).unwrap(), WinnerResp { winner: None });

    assert_eq!(
        contract.highest_bid(&app).unwrap(),
        BidInfo {
            addr: Addr::unchecked(owner),
            amount: Uint128::zero(),
        }
    );

    assert_eq!(
        contract.total_bid(&app, bidder1).unwrap(),
        TotalBidResp {
            amount: Uint128::zero(),
        }
    );

    assert_eq!(
        contract.total_bid(&app, bidder2).unwrap(),
        TotalBidResp {
            amount: Uint128::zero(),
        }
    );
}
