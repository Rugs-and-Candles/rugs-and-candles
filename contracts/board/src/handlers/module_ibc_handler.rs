use std::vec;

use abstract_adapter::std::ibc::ModuleIbcMsg;
use common::board::{BoardAdapter, BoardExecuteMsg, BoardIbcMsg, TileAction};
use common::module_ids::CONTROLLER_ID;
use cosmwasm_std::{from_json, Addr, DepsMut, Env, Response};

use crate::contract::BoardResult;
use crate::state::{TileId, ONGOING_ACTIONS, TILES};
use crate::BoardError;

use super::execute::register_action;

/// This function is used as a router for IBC messagesm to the
/// proper `ExecuteMsg` variant.
pub fn module_ibc_handler(
    deps: DepsMut,
    env: Env,
    module: BoardAdapter,
    ibc_msg: ModuleIbcMsg,
) -> BoardResult<Response> {
    // Check that sender is the proper controller.
    if ibc_msg.source_module.id().ne(CONTROLLER_ID) {
        return Err(BoardError::Unauthorized {});
    }

    let ibc_msg_decoded: BoardIbcMsg = from_json(&ibc_msg.msg)?;
    match ibc_msg_decoded {
        BoardIbcMsg::RegisterAction { user, tile_number } => {
            // The address receive is raw bytes and now we convert it into
            // the specific chain bech32 format.
            let user_addr = deps.api.addr_humanize(&user)?;
            let tile_id: TileId = tile_number;
            register_action(deps, env, module, user_addr, tile_id)
        }
    }
}
