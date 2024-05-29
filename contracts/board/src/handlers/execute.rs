use crate::{
    contract::BoardResult,
    state::{CONFIG, ONGOING_ACTIONS, STATUS, TILES},
    BoardError,
};

use abstract_adapter::{
    objects::{module::ModuleInfo, namespace::Namespace},
    sdk::{AccountVerification, IbcInterface, ModuleRegistryInterface},
    traits::AbstractResponse,
};
use common::{
    board::{BoardAdapter, BoardExecuteMsg, TileAction},
    module_ids::{CONTROLLER_ID, RUGS_N_CANDLES_NAMESPACE},
};
use cosmwasm_std::{ensure_eq, Addr, CosmosMsg, Deps, DepsMut, Env, MessageInfo};

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
    let action = TILES.load(deps.storage, user_tile)?;

    let msgs: Vec<CosmosMsg> = match action {
        TileAction::Candle { n_tile } => create_ibc_proceed_user(
            deps.as_ref(),
            &adapter,
            &info.sender,
            Some(user_tile + u32::from(n_tile)),
        )?,
        TileAction::Rugg { n_tile } => create_ibc_proceed_user(
            deps.as_ref(),
            &adapter,
            &info.sender,
            Some(user_tile - u32::from(n_tile)),
        )?,
        TileAction::Action { action } => {
            if let Some(_action) = action {
                // action.
                // for msg in action.action_msgs {
                //     msgs.push(adapter.execute_message(msg)?);
                // }
                // msgs
                todo!()
            } else {
                create_ibc_proceed_user(deps.as_ref(), &adapter, &info.sender, None)?
            }
        }
    };

    ONGOING_ACTIONS.remove(deps.storage, &info.sender);

    Ok(adapter
        .response("finish_action")
        .add_attribute("new_status", "finished")
        .add_attribute(
            "removed_user_from_tile",
            format!("{} form{}", info.sender, user_tile),
        )
        .add_attribute("tile_id", user_tile.to_string())
        .add_messages(msgs)
        // TODO add IBC call to Controller to inform that the action is started or finished
        // .add_message()
        .add_attribute("account_id", account_id.to_string()))
}

fn create_ibc_proceed_user(
    deps: Deps,
    adapter: &BoardAdapter,
    user: &Addr,
    n_tiles: Option<u32>,
) -> Result<Vec<CosmosMsg>, BoardError> {
    let message = adapter.ibc_client(deps).module_ibc_action(
        "neutron".to_string(),
        ModuleInfo::from_id_latest(CONTROLLER_ID)?,
        &common::controller::ControllerIbcMsg::ProceedUser {
            client_user_address: user.to_string(),
            tile_number: n_tiles,
        },
        None,
        // Some(CallbackInfo {id: CallbackIds::RegisterConfirm.into(), msg: None})
    )?;

    Ok([message].to_vec())
}

