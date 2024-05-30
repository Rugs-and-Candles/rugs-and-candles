use crate::{
    contract::ControllerResult,
    state::{Config,  BOARD_IDS, CONFIG},
};

use common::{
    config::controller_boards,
    controller::{Controller, ControllerInstantiateMsg, PositionRange},
};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

pub fn instantiate_handler(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _adapter: Controller,
    msg: ControllerInstantiateMsg,
) -> ControllerResult {
    println!("Instantiate Controller");
    let config: Config = Config {};

    CONFIG.save(deps.storage, &config)?;

    let controller_boards = msg.boards;
    for (board_id, board_range) in controller_boards {
        let range = PositionRange::new(board_range.start, board_range.end);
        BOARD_IDS.save(deps.storage, &board_id, &range)?;
    }

    println!("Completed Controller instantiation");
    Ok(Response::new())
}
