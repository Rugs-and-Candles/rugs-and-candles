use crate::{
    contract::ControllerResult,
    state::{BoardId, Config, PositionRange, BOARD_IDS, CONFIG},
};

use common::controller::{Controller, ControllerInstantiateMsg};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

pub fn instantiate_handler(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _adapter: Controller,
    _msg: ControllerInstantiateMsg,
) -> ControllerResult {
    println!("Instantiate Controller");
    let config: Config = Config {};

    CONFIG.save(deps.storage, &config)?;

    let board_id: BoardId = "kujira".to_string();
    let board_range = PositionRange::new(1, 10);
    println!("Init vars");

    BOARD_IDS.save(deps.storage, &board_id, &board_range)?;
    println!("Completed Controller instantiation");
    Ok(Response::new())
}
