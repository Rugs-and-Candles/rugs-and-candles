use abstract_adapter::objects::module::ModuleInfo;
use abstract_adapter::sdk::IbcInterface;
use abstract_adapter::std::ibc::ModuleIbcMsg;
use common::controller::{Controller, ControllerIbcMsg};
use common::errors::ControllerError;
use common::module_ids::BOARD_ID;
use cosmwasm_std::{from_json, Addr, CosmosMsg, DepsMut, Env, Order, Response, StdResult};

use crate::contract::ControllerResult;
use crate::state::{PositionRange, BOARD_IDS, PARTICIPANTS};

pub fn module_ibc_handler(
    deps: DepsMut,
    _env: Env,
    adapter: Controller,
    ibc_msg: ModuleIbcMsg,
) -> ControllerResult<Response> {
    println!("Answer received back");
    if ibc_msg.source_module.id().ne(BOARD_ID) {
        return Err(ControllerError::Unauthorized {});
    }

    let server_msg: ControllerIbcMsg = from_json(&ibc_msg.msg)?;
    let mut msgs: Vec<CosmosMsg> = vec![];
    match server_msg {
        ControllerIbcMsg::ProceedUser {
            client_user_address,
            tile_number,
        } => {
            // TODO: should be fixed
            let user_address = Addr::unchecked(client_user_address.clone());

            // get current user position
            let mut user_position = PARTICIPANTS.load(deps.storage, &user_address)?;

            // add the tiles depending on the message received
            if let Some(number) = tile_number {
                // Update position and then call board
                user_position += number;
            } else {
                // random number and then call board
                let serious_random_number = 4;
                user_position += serious_random_number;
            };

            // load to board to see where to send the next message.
            let boards = BOARD_IDS
                .range(deps.storage, None, None, Order::Ascending)
                .collect::<StdResult<Vec<(String, PositionRange)>>>()?;

            let cannonical_sender = deps.api.addr_canonicalize(&client_user_address)?;
            // TODO: fix this, the user can be outside the board and next_chain will be uninint.
            for (chain_name, position_range) in boards {
                if position_range.start() <= user_position && position_range.end() >= user_position
                {
                    let message = adapter.ibc_client(deps.as_ref()).module_ibc_action(
                        chain_name.to_string(),
                        ModuleInfo::from_id_latest(BOARD_ID)?,
                        &common::board::BoardIbcMsg::RegisterAction {
                            user: cannonical_sender.clone(),
                            tile_number: user_position,
                        },
                        None,
                    )?;
                    msgs.push(message)
                }
            }
        }
    }
    Ok(Response::new().add_messages(msgs))
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
