use abstract_adapter::std::ibc::ModuleIbcMsg;
use abstract_client::Namespace;
use cosmwasm_std::{DepsMut, Env, Response};
use cw_orch::mock::cw_multi_test::error::Error;

use crate::BoardError;

pub fn module_ibc_handler<Module, Error>(
    _deps: DepsMut,
    _env: Env,
    _module: Module,
    msg: ModuleIbcMsg,
) -> Result<Response, BoardError> {

    if msg.source_module.name != "" || msg.source_module.namespace != Namespace::new("").unwrap() {
        return Err(BoardError::Unauthorized {  })
    }

    // let server_msg: ServerIbcMessage = from_json(&ibc_msg.msg)?;
    // let user_addr = Addr::unchecked(user);
    // let tile_id: TileId = tile_number;
    // ONGOING_ACTIONS.save(deps.storage, &user_addr, &tile_id)?;

    Ok(
        adapter
            .response("start_action")
            .add_attribute("new_status", "started"), // TODO: add success IBC aknowledgement to inform controller
    )
    unimplemented!()
}
