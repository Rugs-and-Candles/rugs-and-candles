use crate::contract::BoardAdapter;
use crate::state::TileAction;

use abstract_adapter::objects::AccountId;
use cosmwasm_schema::QueryResponses;

// This is used for type safety and re-exporting the contract endpoint structs.
abstract_adapter::adapter_msg_types!(BoardAdapter, BoardExecuteMsg, BoardQueryMsg);

/// Adapter instantiate message
#[cosmwasm_schema::cw_serde]
pub struct BoardInstantiateMsg {
    /// The name of the chain in which this contract
    /// is instantiated.
    chain: String,
    /// A vector containing the index of the tile and
    /// the action to perform.
    tiles_actions: Vec<(u64, String)>,
    /// Number of tiles associated with this chain.
    tiles_number: u64,
    /// Controller addres
    controller_address: String,
}

/// Adapter execute messages
#[cosmwasm_schema::cw_serde]
pub enum BoardExecuteMsg {
    /// Set status of your account
    SetStatus {
        status: String,
    },
    /// Admin method: Update the configuration of the adapter
    UpdateConfig {},
    StartAction {},
    FinishAction {},
}

/// Adapter query messages
#[cosmwasm_schema::cw_serde]
#[derive(QueryResponses, cw_orch::QueryFns)]
#[impl_into(QueryMsg)]
pub enum BoardQueryMsg {
    #[returns(StatusResponse)]
    Status { account_id: AccountId },
    #[returns(ConfigResponse)]
    Config {},
    #[returns(OngoingActionResponse)]
    OngoingAction {},
}

#[cosmwasm_schema::cw_serde]
pub struct ConfigResponse {}

#[cosmwasm_schema::cw_serde]
pub struct StatusResponse {
    pub status: Option<String>,
}

#[cosmwasm_schema::cw_serde]
pub struct OngoingActionResponse {}
