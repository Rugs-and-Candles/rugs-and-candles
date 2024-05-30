use abstract_adapter::{objects::AccountId, AdapterContract};
use cosmwasm_schema::QueryResponses;
use cosmwasm_std::Addr;

use crate::errors::ControllerError;

pub type Controller = AdapterContract<
    ControllerError,
    ControllerInstantiateMsg,
    ControllerExecuteMsg,
    ControllerQueryMsg,
>;

abstract_adapter::adapter_msg_types!(Controller, ControllerExecuteMsg, ControllerQueryMsg);

/// Adapter instantiate message
#[cosmwasm_schema::cw_serde]
pub struct ControllerInstantiateMsg {}

/// Adapter execute messages
#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::ExecuteFns)]
#[impl_into(ExecuteMsg)]
pub enum ControllerExecuteMsg {
    /// Set status of your account
    SetStatus {
        status: String,
    },
    /// Admin method: Update the configuration of the adapter
    UpdateConfig {},
    Join {},
    RollDice {},
}

/// Adapter query messages
#[cosmwasm_schema::cw_serde]
#[derive(QueryResponses, cw_orch::QueryFns)]
#[impl_into(QueryMsg)]
pub enum ControllerQueryMsg {
    #[returns(StatusResponse)]
    Status { account_id: AccountId },
    #[returns(ConfigResponse)]
    Config {},
    #[returns(UserPositionResponse)]
    UserPosition { user_address: String },
    #[returns(ParticipantsResponse)]
    Participants { }
}

#[cosmwasm_schema::cw_serde]
pub struct ConfigResponse {}

#[cosmwasm_schema::cw_serde]
pub struct StatusResponse {
    pub status: Option<String>,
}

#[cosmwasm_schema::cw_serde]
pub struct UserPositionResponse {
    pub position: Option<u32>,
}

#[cosmwasm_schema::cw_serde]
pub struct ParticipantsResponse {
    pub participants: Vec<(Addr, u32)>,
}

#[cosmwasm_schema::cw_serde]
pub enum ControllerIbcMsg {
    ProceedUser {
        client_user_address: String,
        tile_number: Option<u32>,
    },
}
// TODO: Build handler for this message

