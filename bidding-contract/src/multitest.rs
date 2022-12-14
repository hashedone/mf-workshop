use cosmwasm_std::{coin, coins, Addr, Coin, Decimal, StdResult, Uint128};
use cw_multi_test::{App, ContractWrapper, Executor};

use crate::error::ContractError;
use crate::msg::{BidInfo, ExecuteMsg, InstantiateMsg, QueryMsg, TotalBidResp, WinnerResp};
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
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    #[track_caller]
    pub fn bid(&self, app: &mut App, sender: &str, funds: &[Coin]) -> Result<(), ContractError> {
        app.execute_contract(
            Addr::unchecked(sender),
            self.0.clone(),
            &ExecuteMsg::Bid {},
            funds,
        )
        .map_err(|e| e.downcast().unwrap())
        .map(|_| ())
    }

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
    let owner = "owner";
    let bidder1 = "bidder1";
    let bidder2 = "bidder2";

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &Addr::unchecked(bidder1), coins(100, STAR))
            .unwrap();

        router
            .bank
            .init_balance(storage, &Addr::unchecked(bidder2), coins(100, STAR))
            .unwrap();
    });

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

    contract.bid(&mut app, bidder1, &coins(100, STAR)).unwrap();

    assert_eq!(
        contract.total_bid(&app, bidder1).unwrap(),
        TotalBidResp {
            amount: Uint128::from(95u128),
        }
    );

    assert_eq!(
        contract.total_bid(&app, bidder2).unwrap(),
        TotalBidResp {
            amount: Uint128::zero(),
        }
    );

    assert_eq!(
        app.wrap().query_balance(&contract.addr(), STAR).unwrap(),
        coin(95, STAR)
    );

    assert_eq!(
        app.wrap()
            .query_balance(&Addr::unchecked(owner), STAR)
            .unwrap(),
        coin(5, STAR)
    );

    let err = contract
        .bid(&mut app, bidder2, &coins(50, STAR))
        .unwrap_err();

    assert_eq!(
        err,
        ContractError::BidTooLow {
            total: Uint128::zero(),
            highest: Uint128::from(95u128),
        }
    );
}
