use abstract_adapter::std::ibc::ModuleIbcMsg;
use common::board::{BoardAdapter, BoardIbcMsg};
use common::module_ids::CONTROLLER_ID;
use cosmwasm_std::{from_json, Addr, DepsMut, Env, Response};

use crate::contract::BoardResult;
use crate::state::{TileId, ONGOING_ACTIONS};
use crate::BoardError;

pub fn module_ibc_handler(
    deps: DepsMut,
    _env: Env,
    _module: BoardAdapter,
    ibc_msg: ModuleIbcMsg,
) -> BoardResult<Response> {
    if ibc_msg.source_module.id().ne(CONTROLLER_ID) {
        return Err(BoardError::Unauthorized {});
    }

    let server_msg: BoardIbcMsg = from_json(&ibc_msg.msg)?;

    match server_msg {
        BoardIbcMsg::RegisterAction { user, tile_number } => {
            let tile_id: TileId = tile_number;
            let user_addr = Addr::unchecked(user);
            return handle_register_action(deps, user_addr, tile_id);
        }
    }
}

fn handle_register_action(deps: DepsMut, user_addr: Addr, tile_id: TileId) -> BoardResult {
    // switch
    //     case RUG:
    //         new ibc rug message

    ONGOING_ACTIONS.save(deps.storage, &user_addr, &tile_id)?;
    Ok(Response::new()) // GOOD
}
