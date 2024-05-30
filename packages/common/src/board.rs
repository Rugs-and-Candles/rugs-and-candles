use abstract_adapter::{objects::AccountId, AdapterContract};
use cosmwasm_schema::QueryResponses;
use cosmwasm_std::{Addr, CanonicalAddr, Coin};

use crate::errors::BoardError;

/// The type of the adapter that is used to build your Adapter and access the Abstract SDK features.
pub type BoardAdapter =
    AdapterContract<BoardError, BoardInstantiateMsg, BoardExecuteMsg, BoardQueryMsg>;

abstract_adapter::adapter_msg_types!(BoardAdapter, BoardExecuteMsg, BoardQueryMsg);

#[cosmwasm_schema::cw_serde]
pub struct RequiredAction {
    pub required_funds: Vec<Coin>,
    pub actions: Vec<ActionType>,
}

/// The action that can be performed on a tile
#[cosmwasm_schema::cw_serde]
pub enum ActionType {
    Lend,
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
}

/// Adapter execute messages
#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::ExecuteFns)]
#[impl_into(ExecuteMsg)]
pub enum BoardExecuteMsg {
    /// Set status of your account
    SetStatus { status: String },
    /// Admin method: Update the configuration of the adapter
    UpdateConfig {},
    // RegisterAction {
    //     user: String,
    //     tile_number: u32,
    // },
    #[payable]
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
    OngoingAction { addr: Addr },
    #[returns(OngoingActionResponse)]
    OngoingActionFromCanonical { addr: CanonicalAddr },
    #[returns(TileActionResponse)]
    TileAction { tile_id: u32 },
}

#[cosmwasm_schema::cw_serde]
pub struct ConfigResponse {}

#[cosmwasm_schema::cw_serde]
pub struct StatusResponse {
    pub status: Option<String>,
}

#[cosmwasm_schema::cw_serde]
pub struct OngoingActionResponse {
    pub tile_id: u32,
    pub action: TileAction,
}

#[cosmwasm_schema::cw_serde]
pub struct TileActionResponse {
    pub action: TileAction,
}

#[cosmwasm_schema::cw_serde]
pub enum BoardIbcMsg {
    RegisterAction {
        user: CanonicalAddr,
        tile_number: u32,
    },
}

#[cosmwasm_schema::cw_serde]
pub enum BoardReplyMsg {
    Success { action: ActionType, addr: Addr },
}
