use abstract_adapter::{objects::AccountId, AdapterContract};
use cosmwasm_schema::QueryResponses;
use cosmwasm_std::{Addr, Coin};

use crate::errors::BoardError;

/// The type of the adapter that is used to build your Adapter and access the Abstract SDK features.
pub type BoardAdapter =
    AdapterContract<BoardError, BoardInstantiateMsg, BoardExecuteMsg, BoardQueryMsg>;

abstract_adapter::adapter_msg_types!(BoardAdapter, BoardExecuteMsg, BoardQueryMsg);

#[cosmwasm_schema::cw_serde]
pub struct RequiredAction {
    required_funds: Vec<Coin>,
    action_msgs: Vec<String>,
}

#[cosmwasm_schema::cw_serde]
pub enum TileAction {
    Rugg { n_tile: u8 },
    Candle { n_tile: u8 },
    Action { action: Option<RequiredAction> },
}

/// Adapter instantiate message
#[cosmwasm_schema::cw_serde]
pub struct BoardInstantiateMsg {
    /// The name of the chain in which this contract
    /// is instantiated.
    pub chain: String,
    /// A vector containing the index of the tile and
    /// the action to perform.
    pub tiles_actions: Vec<(u32, TileAction)>,
    /// Number of tiles associated with this chain.
    pub tiles_number: u32,
    /// Controller addres
    pub controller_address: String,
}

/// Adapter execute messages
#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::ExecuteFns)]
#[impl_into(ExecuteMsg)]
pub enum BoardExecuteMsg {
    /// Set status of your account
    SetStatus {
        status: String,
    },
    /// Admin method: Update the configuration of the adapter
    UpdateConfig {},
    RegisterAction {
        user: String,
        tile_number: u32,
    },
    PerformAction {},
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

#[cosmwasm_schema::cw_serde]
pub enum BoardIbcMsg {
    RegisterAction { user: Addr, tile_number: u32 },
}

