use abstract_adapter::objects::module::ModuleInfo;
use abstract_adapter::sdk::IbcInterface;
use abstract_adapter::std::ibc::ModuleIbcMsg;
use common::controller::{Controller, ControllerIbcMsg};
use common::errors::ControllerError;
use common::module_ids::BOARD_ID;
use cosmwasm_std::{from_json, Addr, DepsMut, Env, Order, Response, StdResult};

use crate::contract::ControllerResult;
use crate::state::{PositionRange, BOARD_IDS, PARTICIPANTS};

pub fn module_ibc_handler(
    deps: DepsMut,
    _env: Env,
    adapter: Controller,
    ibc_msg: ModuleIbcMsg,
) -> ControllerResult<Response> {
    if ibc_msg.source_module.id().ne(BOARD_ID) {
        return Err(ControllerError::Unauthorized {});
    }

    let server_msg: ControllerIbcMsg = from_json(&ibc_msg.msg)?;
    match server_msg {
        ControllerIbcMsg::ProceedUser {
            client_user_address,
            tile_number,
        } => {
            // TODO: should be fixed
            let user_address = Addr::unchecked(client_user_address);

            let mut user_position = PARTICIPANTS.load(deps.storage, &user_address)?;
            if let Some(number) = tile_number {
                // Update position and then call board
                user_position += number;
            } else {
                // random number and then call board
                let serious_random_number = 4;
                user_position += serious_random_number;
            };

            let boards = BOARD_IDS
                .range(deps.storage, None, None, Order::Ascending)
                .collect::<StdResult<Vec<(String, PositionRange)>>>()?;

            let mut next_chain: String;
            for (chain_name, position_range) in boards {
                if &position_range.start() <= &user_position
                    && &position_range.end() >= &user_position
                {
                    next_chain = chain_name;
                }
            }

            handle_action_completion(adapter, client_user_address)
        }
    }
    Ok(Response::new())
}

fn handle_action_completion(
    adapter: Controller,
    chain_name: String,
    client_user_address: String,
    deps: DepsMut,
) -> ControllerResult {
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
}

pub enum CallbackIds {
    RegisterConfirm,
}

impl From<CallbackIds> for String {
    fn from(val: CallbackIds) -> Self {
        match val {
            CallbackIds::RegisterConfirm => "register_confirm".to_string(),
        }
    }
}
