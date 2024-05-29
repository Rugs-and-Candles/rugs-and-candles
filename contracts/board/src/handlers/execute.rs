use crate::{
    contract::BoardResult,
    state::{TileId, CONFIG, ONGOING_ACTIONS, STATUS, TILES},
    BoardError,
};

use abstract_adapter::{
    objects::namespace::Namespace,
    sdk::{AccountVerification, ModuleInterface, ModuleRegistryInterface},
    std::IBC_CLIENT,
    traits::AbstractResponse,
};
use common::{
    board::{BoardAdapter, BoardExecuteMsg},
    module_ids::RUGS_N_CANDLES_NAMESPACE,
};
use cosmwasm_std::{ensure_eq, Addr, DepsMut, Env, MessageInfo};

pub fn execute_handler(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    adapter: BoardAdapter,
    msg: BoardExecuteMsg,
) -> BoardResult {
    use BoardExecuteMsg::*;
    match msg {
        UpdateConfig {} => update_config(deps, info, adapter),
        SetStatus { status } => set_status(deps, adapter, status),
        PerformAction {} => perform_action(deps, info, adapter),
    }
}

/// Update the configuration of the adapter
fn update_config(deps: DepsMut, _msg_info: MessageInfo, adapter: BoardAdapter) -> BoardResult {
    // Only admin(namespace owner) can change recipient address
    let namespace = adapter
        .module_registry(deps.as_ref())?
        .query_namespace(Namespace::new(RUGS_N_CANDLES_NAMESPACE)?)?;

    // unwrap namespace, since it's unlikely to have unclaimed namespace as this adapter installed
    let namespace_info = namespace.unwrap();
    ensure_eq!(
        namespace_info.account_base,
        adapter.target_account.clone().unwrap(),
        BoardError::Unauthorized {}
    );
    let mut _config = CONFIG.load(deps.storage)?;

    Ok(adapter.response("update_config"))
}

fn set_status(deps: DepsMut, adapter: BoardAdapter, status: String) -> BoardResult {
    let account_registry = adapter.account_registry(deps.as_ref())?;

    let account_id = account_registry.account_id(adapter.target()?)?;
    STATUS.save(deps.storage, &account_id, &status)?;

    Ok(adapter
        .response("set_status")
        .add_attribute("new_status", &status)
        .add_attribute("account_id", account_id.to_string()))
}

fn perform_action(deps: DepsMut, info: MessageInfo, adapter: BoardAdapter) -> BoardResult {
    let account_registry = adapter.account_registry(deps.as_ref())?;

    let account_id = account_registry.account_id(adapter.target()?)?;
    STATUS.save(deps.storage, &account_id, &"finished".to_string())?;

    let user_tile = ONGOING_ACTIONS.load(deps.storage, &info.sender)?;
    let user_action = TILES.load(deps.storage, user_tile)?;
    // TODO: clear the user from the tiles

    Ok(adapter
        .response("finish_action")
        .add_attribute("new_status", "finished")
        // TODO add IBC call to Controller to inform that the action is started or finished
        // .add_message()
        .add_attribute("account_id", account_id.to_string()))
}
