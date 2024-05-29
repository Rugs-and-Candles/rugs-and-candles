use crate::contract::Controller;

use abstract_adapter::objects::AccountId;
use cosmwasm_schema::QueryResponses;

// This is used for type safety and re-exporting the contract endpoint structs.
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
#[impl_into(QueryMsg)]
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
