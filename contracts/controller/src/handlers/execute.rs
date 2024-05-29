use crate::{
    contract::{AdapterResult, Controller},
    error::ControllerError,
    msg::ControllerExecuteMsg,
    state::{PositionRange, BOARD_IDS, CONFIG, STATUS},
    CONTROLLER_NAMESPACE,
};

use common::module_ids::BOARD_ID;
use abstract_adapter::{
    objects::{module::ModuleInfo, namespace::Namespace},
    sdk::{AccountVerification, IbcInterface, ModuleRegistryInterface},
    traits::AbstractResponse,
};
use cosmwasm_std::{ensure_eq, Addr, DepsMut, Env, MessageInfo, Order, StdError};

pub fn execute_handler(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    adapter: Controller,
    msg: ControllerExecuteMsg,
) -> AdapterResult {
    use ControllerExecuteMsg::*;
    match msg {
        UpdateConfig {} => update_config(deps, info, adapter),
        SetStatus { status } => set_status(deps, adapter, status),
        RollDice {} => unimplemented!(),
        Join {} => join(deps, adapter, info.sender),
    }
}

/// Update the configuration of the adapter
fn update_config(deps: DepsMut, _msg_info: MessageInfo, adapter: Controller) -> AdapterResult {
    // Only admin(namespace owner) can change recipient address
    let namespace = adapter
        .module_registry(deps.as_ref())?
        .query_namespace(Namespace::new(CONTROLLER_NAMESPACE)?)?;

    // unwrap namespace, since it's unlikely to have unclaimed namespace as this adapter installed
    let namespace_info = namespace.unwrap();
    ensure_eq!(
        namespace_info.account_base,
        adapter.target_account.clone().unwrap(),
        ControllerError::Unauthorized {}
    );
    let mut _config = CONFIG.load(deps.storage)?;

    Ok(adapter.response("update_config"))
}

fn set_status(deps: DepsMut, adapter: Controller, status: String) -> AdapterResult {
    let account_registry = adapter.account_registry(deps.as_ref())?;

    let account_id = account_registry.account_id(adapter.target()?)?;
    STATUS.save(deps.storage, &account_id, &status)?;

    Ok(adapter
        .response("set_status")
        .add_attribute("new_status", &status)
        .add_attribute("account_id", account_id.to_string()))
}

fn join(deps: DepsMut, adapter: Controller, sender: Addr) -> AdapterResult {
    let account_registry = adapter.account_registry(deps.as_ref())?;
    let account_id = account_registry.account_id(adapter.target()?)?;


    let board_id_ranges = BOARD_IDS.range(deps.storage, None, None,Order::Ascending).next().ok_or(StdError::generic_err("No board found"))??;

    let start_tile_id = board_id_ranges.1.start();




    let message = adapter.ibc_client(deps.as_ref()).module_ibc_action(
        board_id_ranges.0.to_string(), 
        ModuleInfo::from_id_latest(BOARD_ID)?,

    );


    Ok(adapter
        .response("join")
        .add_attribute("account_id", account_id.to_string()))
}
