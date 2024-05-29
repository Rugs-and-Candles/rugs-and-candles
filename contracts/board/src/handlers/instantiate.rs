use std::collections::HashMap;

use crate::{
    contract::AdapterResult,
    state::{Config, CONFIG, TILES},
    BoardError,
};

use abstract_adapter::AdapterError;
use common::{board::{BoardAdapter, BoardInstantiateMsg, TileAction}, game::Chains};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

pub fn instantiate_handler(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _adapter: BoardAdapter,
    msg: BoardInstantiateMsg,
) -> AdapterResult {
    let current_chain = Chains::from(&msg.chain);
    if current_chain == Chains::NonSupported {
        return Err(BoardError::InvalidChain(msg.chain));
    }

    let config: Config = Config { current_chain };

    CONFIG.save(deps.storage, &config)?;
    // TODO: save the controller id. How?

    let tiles_map: HashMap<_, _> = msg
        .tiles_actions
        .into_iter()
        .map(|(key, value)| (key, value))
        .collect();

    for i in 0..msg.tiles_number {
        match tiles_map.get(&i) {
            Some(action) => TILES.save(deps.storage, i, &action)?,
            None => TILES.save(deps.storage, i, &TileAction::Action { action: None })?,
        }
    }

    Ok(Response::new())
}
