use crate::{
    contract::BoardResult,
    state::{CONFIG, ONGOING_ACTIONS, STATUS, TILES},
};

use abstract_adapter::objects::AccountId;
use common::board::{
    BoardAdapter, BoardQueryMsg, ConfigResponse, OngoingActionResponse, StatusResponse,
    TileActionResponse,
};
use cosmwasm_std::{to_json_binary, Addr, Binary, Deps, Env, StdResult};

pub fn query_handler(
    deps: Deps,
    _env: Env,
    _adapter: &BoardAdapter,
    msg: BoardQueryMsg,
) -> BoardResult<Binary> {
    use BoardQueryMsg::*;
    match msg {
        Config {} => to_json_binary(&query_config(deps)?),
        Status { account_id } => to_json_binary(&query_status(deps, account_id)?),
        OngoingAction { addr } => to_json_binary(&query_ongoing_action(deps, addr)?),
        OngoingActionFromCanonical { addr } => {
            to_json_binary(&query_ongoing_action_from_canonical(deps, addr)?)
        }
        TileAction { tile_id } => to_json_binary(&query_tile_action(deps, tile_id)?),
    }
    .map_err(Into::into)
}

fn query_ongoing_action_from_canonical(
    deps: Deps<cw_orch::prelude::Empty>,
    addr: cosmwasm_std::CanonicalAddr,
) -> StdResult<OngoingActionResponse> {
    let addr = deps.api.addr_humanize(&addr)?;
    query_ongoing_action(deps, addr)
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let _config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {})
}

fn query_status(deps: Deps, account_id: AccountId) -> StdResult<StatusResponse> {
    let status = STATUS.may_load(deps.storage, &account_id)?;
    Ok(StatusResponse { status })
}

fn query_ongoing_action(deps: Deps, addr: Addr) -> StdResult<OngoingActionResponse> {
    let tile_id = ONGOING_ACTIONS.load(deps.storage, &addr)?;
    let action = TILES.load(deps.storage, tile_id)?;

    Ok(OngoingActionResponse { tile_id, action })
}

fn query_tile_action(deps: Deps, tile_id: u32) -> StdResult<TileActionResponse> {
    let action = TILES.load(deps.storage, tile_id)?;
    Ok(TileActionResponse { action })
}
