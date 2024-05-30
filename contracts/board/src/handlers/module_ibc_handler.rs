use std::vec;

use abstract_adapter::std::ibc::ModuleIbcMsg;
use common::board::{BoardAdapter, BoardIbcMsg, TileAction};
use common::module_ids::CONTROLLER_ID;
use cosmwasm_std::{from_json, Addr, DepsMut, Env, Response};

use crate::contract::BoardResult;
use crate::state::{TileId, ONGOING_ACTIONS,  TILES};
use crate::BoardError;

use super::execute::match_tile_action_to_message;

pub fn module_ibc_handler(
    deps: DepsMut,
    env: Env,
    module: BoardAdapter,
    ibc_msg: ModuleIbcMsg,
) -> BoardResult<Response> {
    if ibc_msg.source_module.id().ne(CONTROLLER_ID) {
        return Err(BoardError::Unauthorized {});
    }

    let server_msg: BoardIbcMsg = from_json(&ibc_msg.msg)?;
    match server_msg {
        BoardIbcMsg::RegisterAction { user, tile_number } => {
            let tile_id: TileId = tile_number;
            let user_addr = deps.api.addr_humanize(&user)?;

            handle_register_action(deps, env, module, user_addr, tile_id)
        }
    }
}

fn handle_register_action(
    deps: DepsMut,
    env: Env,
    adapter: BoardAdapter,
    user_addr: Addr,
    tile_id: TileId,
) -> BoardResult {
    let tile_action = TILES.load(deps.storage, tile_id)?;

    let sub_msgs = match tile_action {
        TileAction::Candle { n_tile: _ } => match_tile_action_to_message(
            tile_action,
            &deps,
            &adapter,
            &user_addr,
            vec![],
            tile_id,
            env,
        )?,
        TileAction::Rugg { n_tile: _ } => match_tile_action_to_message(
            tile_action,
            &deps,
            &adapter,
            &user_addr,
            vec![],
            tile_id,
            env,
        )?,

        _ => {
            vec![]
        },
    };

    ONGOING_ACTIONS.save(deps.storage, &user_addr, &tile_id)?;
    Ok(Response::new()
        .add_submessages(sub_msgs)
        .add_attributes(vec![
            ("action", "register_action"),
            ("tile_id", tile_id.to_string().as_str()),
            ("user", user_addr.to_string().as_str()),
        ])) // GOOD
}
