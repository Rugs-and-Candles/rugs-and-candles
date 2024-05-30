use abstract_adapter::sdk::AbstractResponse;
use common::{board::BoardAdapter, errors::BoardError};
use cosmwasm_std::{BankMsg, DepsMut, Env, Reply, StdError};

use crate::{contract::BoardResult, state::TEMP_USER};

pub fn reply_handler(deps: DepsMut, env: Env, adapter: BoardAdapter, reply: Reply) -> BoardResult {
    match reply.result {
        cosmwasm_std::SubMsgResult::Err(_) => {
            Err(BoardError::Std(StdError::generic_err("submessage error")))
        }
        _ => Ok(()),
    }?;
    let temp_user = TEMP_USER.load(deps.storage)?;
    let balances = deps.querier.query_all_balances(env.contract.address)?;
    let send_msg = BankMsg::Send {
        to_address: temp_user.to_string(),
        amount: balances,
    };

    Ok(adapter.response("success").add_message(send_msg))
}
