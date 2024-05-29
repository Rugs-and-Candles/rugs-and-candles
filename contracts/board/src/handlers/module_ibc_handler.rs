use abstract_adapter::{sdk::AbstractResponse, std::ibc::ModuleIbcMsg};
use abstract_client::Namespace;
use common::board::BoardAdapter;
use cosmwasm_std::{DepsMut, Env, Response};

use crate::contract::{BoardAdapter, BoardResult};
use crate::msg::BoardIbcMsg;
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
            return handle_register_action(deps, user, tile_id);
        }
    }
}

fn handle_register_action(deps: DepsMut, user_addr: Addr, tile_id: TileId) -> BoardResult {
    ONGOING_ACTIONS.save(deps.storage, &user_addr, &tile_id)?;
    Ok(Response::new())
}
