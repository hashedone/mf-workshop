use cw_multi_test::{App, ContractWrapper};

use crate::{execute, instantiate, query};

pub struct CodeId(u64);

impl CodeId {
    pub fn store_code(app: &mut App) -> Self {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Self(app.store_code(Box::new(contract)))
    }
}

#[test]
fn flow() {
    let mut app = App::default();

    let code_id = CodeId::store_code(&mut app);
}
