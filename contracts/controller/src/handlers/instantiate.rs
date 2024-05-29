use crate::{
    contract::{AdapterResult},
    state::{Config, CONFIG},
};

use common::controller::{Controller, ControllerInstantiateMsg};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

pub fn instantiate_handler(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _adapter: Controller,
    _msg: ControllerInstantiateMsg,
) -> AdapterResult {
    let config: Config = Config {};

    CONFIG.save(deps.storage, &config)?;

    // Example instantiation that doesn't do anything
    Ok(Response::new())
}
