use crate::{
    contract::ControllerResult,
<<<<<<< HEAD
    state::{BOARD_IDS, CONFIG, STATUS},
=======
    state::{PositionRange, BOARD_IDS, CONFIG, PARTICIPANTS, STATUS},
>>>>>>> 1a15662 (bbbb)
};

use abstract_adapter::{
    objects::{module::ModuleInfo, namespace::Namespace},
    sdk::{AccountVerification, IbcInterface, ModuleRegistryInterface},
    traits::AbstractResponse,
};
use common::{
    controller::{Controller, ControllerExecuteMsg},
    errors::ControllerError,
    module_ids::{BOARD_ID, RUGS_N_CANDLES_NAMESPACE},
};
use cosmwasm_std::{ensure_eq, Addr, DepsMut, Env, MessageInfo, Order, StdError};

/// Main handler of the execute messages supported by the contract.
pub fn execute_handler(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    adapter: Controller,
    msg: ControllerExecuteMsg,
) -> ControllerResult {
    use ControllerExecuteMsg::*;
    match msg {
        RollDice {} => unimplemented!(),
        Join {} => join(deps, adapter, info.sender),
        UpdateConfig {} => update_config(deps, info, adapter),
        SetStatus { status } => set_status(deps, adapter, status),
    }
}

/// Join allows a user to participate to the rugs and candles game. This function returns an IBC
/// message relayed to the board adapter to start the game.
fn join(deps: DepsMut, adapter: Controller, sender: Addr) -> ControllerResult {
    let (chain_name, position_range) = BOARD_IDS
        .range(deps.storage, None, None, Order::Ascending)
        .next()
        .ok_or(StdError::generic_err("No board found"))??;
    let start_tile_id = position_range.start();

    PARTICIPANTS.save(deps.storage, &sender, &start_tile_id)?;

    let cannonical_sender = deps.api.addr_canonicalize(sender.as_str())?;

    let message = adapter.ibc_client(deps.as_ref()).module_ibc_action(
        chain_name.to_string(),
        ModuleInfo::from_id_latest(BOARD_ID)?,
        &common::board::BoardIbcMsg::RegisterAction {
            user: cannonical_sender.clone(),
            tile_number: 0,
        },
        None,
    )?;

    Ok(adapter
        .response("join")
        .add_attribute("start_tile_id", start_tile_id.to_string())
        .add_attribute("target_bech32_sender", cannonical_sender.to_string())
        .add_message(message))
}

/// BOILERPLATE
///
/// Update the configuration of the adapter
fn update_config(deps: DepsMut, _msg_info: MessageInfo, adapter: Controller) -> ControllerResult {
    // Only admin(namespace owner) can change recipient address
    let namespace = adapter
        .module_registry(deps.as_ref())?
        .query_namespace(Namespace::new(RUGS_N_CANDLES_NAMESPACE)?)?;

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

fn set_status(deps: DepsMut, adapter: Controller, status: String) -> ControllerResult {
    let account_registry = adapter.account_registry(deps.as_ref())?;

    let account_id = account_registry.account_id(adapter.target()?)?;
    STATUS.save(deps.storage, &account_id, &status)?;

    Ok(adapter
        .response("set_status")
        .add_attribute("new_status", &status)
        .add_attribute("account_id", account_id.to_string()))
}
