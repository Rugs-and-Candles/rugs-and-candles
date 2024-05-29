use crate::{
    contract::{AdapterResult, BoardAdapter},
    msg::BoardInstantiateMsg,
    state::{Config, CONFIG, TILES},
    BoardError,
};

use abstract_adapter::AdapterError;
use common::game::Chains;
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

    let tiles_map: HashMap<_, _> = vec.into_iter().map(|(key, value)| (key, value)).collect();

    // for i in 0..msg.tiles_number {
    //     if tiles_map.contains
    //     TILES.save(deps.storage,i, )
    // }
    Ok(Response::new())
}
