use std::collections::HashMap;

use crate::{
    contract::BoardResult,
    state::{Config, CONFIG, TILES},
};

use abstract_adapter::sdk::AbstractResponse;
use common::{
    board::{BoardAdapter, BoardInstantiateMsg, TileAction},
    game::Chains,
};
use cosmwasm_std::{DepsMut, Env, MessageInfo};

pub fn instantiate_handler(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    adapter: BoardAdapter,
    msg: BoardInstantiateMsg,
) -> BoardResult {
    let current_chain = Chains::try_from(msg.chain.as_str())?;
    let config: Config = Config { current_chain };

    CONFIG.save(deps.storage, &config)?;

    let tiles_map: HashMap<_, _> = msg.tiles_actions.into_iter().collect();

    // TODO: add check on allowed action types for the chain.
    //
    // To be consistent with board games, the first tile is the number 1, not 0.
    for i in 1..msg.tiles_number + 1 {
        match tiles_map.get(&i) {
            Some(action) => TILES.save(deps.storage, i, action)?,
            None => TILES.save(deps.storage, i, &TileAction::Action { action: None })?,
        }
    }
    Ok(adapter.response("instantiate"))
}
