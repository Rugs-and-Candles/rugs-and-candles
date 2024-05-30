use crate::{
    contract::ControllerResult,
    state::{CONFIG, PARTICIPANTS, STATUS},
};

use abstract_adapter::objects::AccountId;
use common::controller::{
    ConfigResponse, Controller, ControllerQueryMsg, ParticipantsResponse, StatusResponse,
    UserPositionResponse,
};
use cosmwasm_std::{to_json_binary, Addr, Binary, Deps, Env, Order, StdResult};

pub fn query_handler(
    deps: Deps,
    _env: Env,
    _adapter: &Controller,
    msg: ControllerQueryMsg,
) -> ControllerResult<Binary> {
    use ControllerQueryMsg::*;
    match msg {
        Config {} => to_json_binary(&query_config(deps)?),
        Status { account_id } => to_json_binary(&query_status(deps, account_id)?),
        UserPosition { user_address } => to_json_binary(&query_user_position(deps, user_address)?),
        Participants {} => to_json_binary(&query_participants(deps)?),
    }
    .map_err(Into::into)
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let _config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {})
}

fn query_status(deps: Deps, account_id: AccountId) -> StdResult<StatusResponse> {
    let status = STATUS.may_load(deps.storage, &account_id)?;
    Ok(StatusResponse { status })
}

/// Retrieve user position in the board. If the user is not in the board, None is returned.
fn query_user_position(deps: Deps, user: String) -> StdResult<UserPositionResponse> {
    let user_address = Addr::unchecked(user);
    let position = PARTICIPANTS.may_load(deps.storage, &user_address)?;
    Ok(UserPositionResponse { position })
}

fn query_participants(deps: Deps) -> StdResult<ParticipantsResponse> {
    let participants = PARTICIPANTS
        .range(deps.storage, None, None, Order::Ascending)
        .collect::<StdResult<Vec<(Addr, u32)>>>()?;
    Ok(ParticipantsResponse { participants })
}
