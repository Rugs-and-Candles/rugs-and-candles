use abstract_adapter::std::ibc::ModuleIbcMsg;
use cosmwasm_std::{DepsMut, Env, Response};

pub fn module_ibc_handler<Module, Error>(
    _deps: DepsMut,
    _env: Env,
    _module: Module,
    _msg: ModuleIbcMsg,
) -> Result<Response, Error> {
    unimplemented!()
}

pub enum CallbackIds {
    RegisterConfirm,
}

impl From<CallbackIds> for String {
    fn from(val: CallbackIds) -> Self {
        match val {
            CallbackIds::RegisterConfirm => "register_confirm".to_string(),
        }
    }
}
