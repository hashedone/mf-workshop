use cosmwasm_std::{Addr, Decimal, StdResult};
use cw_multi_test::{App, ContractWrapper, Executor};

use crate::msg::{ExecuteMsg, InstantiateMsg};
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

const STAR: &str = "star";

#[test]
fn flow() {
    let mut app = App::default();

    let owner = "onwer";

    let code_id = CodeId::store_code(&mut app);
    let contract = code_id
        .instantiate(&mut app, owner, STAR, None, Decimal::percent(5))
        .unwrap();
}
