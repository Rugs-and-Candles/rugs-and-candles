
use abstract_adapter::{objects::AccountId, AdapterContract};
use cosmwasm_schema::QueryResponses;

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
pub enum ControllerQueryMsg {
    #[returns(StatusResponse)]
    Status { account_id: AccountId },
    #[returns(ConfigResponse)]
    Config {},
}

#[cosmwasm_schema::cw_serde]
pub struct ConfigResponse {}

#[cosmwasm_schema::cw_serde]
pub struct StatusResponse {
    pub status: Option<String>,
}
