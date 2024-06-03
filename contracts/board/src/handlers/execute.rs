use crate::{
    contract::BoardResult,
    state::{TileId, CONFIG, ONGOING_ACTIONS, STATUS, TEMP_USER, TILES},
    BoardError,
};
use abstract_money_market_adapter::{
    msg::{GenerateMessagesResponse, MoneyMarketExecuteMsg, MoneyMarketQueryMsg},
    MONEY_MARKET_ADAPTER_ID,
};
use log::info;

use abstract_adapter::traits::AbstractNameService;
use abstract_adapter::{
    objects::{module::ModuleInfo, namespace::Namespace},
    sdk::{AccountVerification, IbcInterface, ModuleRegistryInterface},
    traits::AbstractResponse,
};
use abstract_money_market_standard::ans_action::MoneyMarketAnsAction;
use common::{
    board::{ActionType, BoardAdapter, BoardExecuteMsg, TileAction},
    module_ids::{CONTROLLER_ID, RUGS_N_CANDLES_NAMESPACE},
};
use cosmwasm_std::{ensure_eq, Addr, Coin, Deps, DepsMut, Env, MessageInfo, StdError, SubMsg};
use cw_asset::AssetBase;

pub fn execute_handler(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    adapter: BoardAdapter,
    msg: BoardExecuteMsg,
) -> BoardResult {
    use BoardExecuteMsg::*;
    match msg {
        PerformAction {} => perform_action(deps, info, env, adapter),
        UpdateConfig {} => update_config(deps, info, adapter),
        SetStatus { status } => set_status(deps, adapter, status),
    }
}

/// Handles an IBC message from the Controller contract to
/// register an action for a user.
pub(crate) fn register_action(
    deps: DepsMut,
    env: Env,
    adapter: BoardAdapter,
    user_addr: Addr,
    tile_id: TileId,
) -> BoardResult {
    info!("Registering action");
    let tile_action = TILES.load(deps.storage, tile_id)?;

    // TODO: add other actions to message.
    let sub_msgs = match tile_action {
        TileAction::Candle { n_tile: _ } | TileAction::Rugg { n_tile: _ } => {
            match_tile_action_to_message(
                tile_action,
                &deps,
                &adapter,
                &user_addr,
                vec![],
                tile_id,
                env,
            )?
        }
        _ => {
            vec![]
        }
    };

    ONGOING_ACTIONS.save(deps.storage, &user_addr, &tile_id)?;
    Ok(adapter
        .response("register_action")
        .add_submessages(sub_msgs)
        .add_attributes(vec![
            ("tile_id", tile_id.to_string().as_str()),
            ("user", user_addr.to_string().as_str()),
        ])) // GOOD
}

pub fn match_tile_action_to_message(
    tile_action: TileAction,
    deps: &DepsMut<cw_orch::prelude::Empty>,
    adapter: &BoardAdapter,
    sender: &Addr,
    info_funds: Vec<Coin>,
    user_tile: u32,
    env: Env,
) -> Result<Vec<SubMsg>, BoardError> {
    let msgs: Vec<SubMsg> = match tile_action {
        TileAction::Candle { n_tile } => create_ibc_proceed_user(
            deps.as_ref(),
            adapter,
            sender,
            Some(user_tile + u32::from(n_tile)),
        )?,
        TileAction::Rugg { n_tile } => create_ibc_proceed_user(
            deps.as_ref(),
            adapter,
            sender,
            Some(user_tile - u32::from(n_tile)),
        )?,
        TileAction::Action { action } => {
            if let Some(action) = action {
                let required_funds = action.required_funds;
                let action_type = action.actions.first().unwrap();

                match action_type {
                    ActionType::Lend => create_lending_message(
                        deps.as_ref(),
                        env,
                        adapter,
                        sender,
                        required_funds,
                        info_funds,
                    )?,
                }
            } else {
                create_ibc_proceed_user(deps.as_ref(), adapter, sender, None)?
            }
        }
    };
    Ok(msgs)
}

fn perform_action(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    adapter: BoardAdapter,
) -> BoardResult {
    info!("Performing action");

    let user_tile = ONGOING_ACTIONS.load(deps.storage, &info.sender)?;
    let tile_action = TILES.load(deps.storage, user_tile)?;
    let sender = info.sender.clone();
    let funds = info.funds.clone();

    let msgs =
        match_tile_action_to_message(tile_action, &deps, &adapter, &sender, funds, user_tile, env)?;

    TEMP_USER.save(deps.storage, &info.sender)?;
    ONGOING_ACTIONS.remove(deps.storage, &info.sender);

    Ok(adapter
        .response("finish_action")
        .add_attribute("new_status", "finished")
        .add_attribute(
            "removed_user_from_tile",
            format!("{} from tile {}", info.sender, user_tile),
        )
        .add_attribute("tile_id", user_tile.to_string())
        .add_submessages(msgs))
}

fn create_ibc_proceed_user(
    deps: Deps,
    adapter: &BoardAdapter,
    user: &Addr,
    n_tiles: Option<u32>,
) -> Result<Vec<SubMsg>, BoardError> {
    let message = adapter.ibc_client(deps).module_ibc_action(
        "neutron".to_string(),
        ModuleInfo::from_id_latest(CONTROLLER_ID)?,
        &common::controller::ControllerIbcMsg::ProceedUser {
            client_user_address: user.to_string(),
            tile_number: n_tiles,
        },
        None,
    )?;
    Ok([SubMsg::new(message)].to_vec())
}

fn create_lending_message(
    deps: Deps,
    env: Env,
    adapter: &BoardAdapter,
    _user: &Addr,
    required_funds: Vec<Coin>,
    added_funds: Vec<Coin>,
) -> Result<Vec<SubMsg>, BoardError> {
    if required_funds.len() > 1 || required_funds.is_empty() {
        return Err(BoardError::Std(StdError::generic_err(
            "Only one fund is supported",
        )));
    }

    if required_funds.ne(&added_funds) {
        return Err(BoardError::Std(StdError::generic_err(
            "Wrong funds added (The required funds are already added)",
        )));
    }

    let first_fund = required_funds.first().unwrap().clone();

    let asset = AssetBase::native(first_fund.denom, first_fund.amount);
    let ans_asset = adapter.name_service(deps).query(&asset)?;

    let money_market_name = "ghost".to_string();

    // Creating the money market adapter message (fix because we're not using accounts)
    let adapter_query: abstract_adapter::std::adapter::QueryMsg<_> =
        MoneyMarketQueryMsg::GenerateMessages {
            message: MoneyMarketExecuteMsg::AnsAction {
                money_market: money_market_name,
                action: MoneyMarketAnsAction::Deposit {
                    lending_asset: ans_asset,
                },
            },
            addr_as_sender: env.contract.address.to_string(),
        }
        .into();
    let mm_address = adapter
        .module_registry(deps)?
        .query_module(ModuleInfo::from_id_latest(MONEY_MARKET_ADAPTER_ID)?)?
        .reference
        .unwrap_adapter()?;
    let deposit_msg: GenerateMessagesResponse =
        deps.querier.query_wasm_smart(mm_address, &adapter_query)?;

    let sub_msg = deposit_msg
        .messages
        .into_iter()
        .map(|m| SubMsg::reply_on_success(m, 0))
        .collect();

    Ok(sub_msg)
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
