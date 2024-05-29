use crate::{
    contract::AdapterResult,
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
        RegisterAction { user, tile_number } => register_action(deps, adapter, user, tile_number),
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

/// Allows the controller to start an action for a user.
fn register_action(
    deps: DepsMut,
    adapter: BoardAdapter,
    user: String,
    tile_number: u32,
) -> BoardResult {
    // TODO: only controller

    adapter.modules(deps.as_ref()).module_address(IBC_CLIENT);

    let user_addr = Addr::unchecked(user);
    let tile_id: TileId = tile_number;
    ONGOING_ACTIONS.save(deps.storage, &user_addr, &tile_id)?;

    Ok(
        adapter
            .response("start_action")
            .add_attribute("new_status", "started"), // TODO: add success IBC aknowledgement to inform controller
    )
}

fn perform_action(deps: DepsMut, info: MessageInfo, adapter: BoardAdapter) -> BoardResult {
    let account_registry = adapter.account_registry(deps.as_ref())?;

    let account_id = account_registry.account_id(adapter.target()?)?;
    STATUS.save(deps.storage, &account_id, &"finished".to_string())?;

    let user_tile = ONGOING_ACTIONS.load(deps.storage, &info.sender)?;
    let user_action = TILES.load(deps.storage, user_tile)?;

    Ok(adapter
        .response("finish_action")
        .add_attribute("new_status", "finished")
        // TODO add IBC call to Controller to inform that the action is started or finished
        .add_attribute("account_id", account_id.to_string()))
}
