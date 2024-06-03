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

/// Controller instantiate message.
#[cosmwasm_schema::cw_serde]
pub struct ControllerInstantiateMsg {
    /// A vector representing the board range in
    /// every chain participating in the game.
    /// [("juno", (1, 10)), ("kujira", (11, 20)), ..]
    pub boards: Vec<(String, PositionRange)>,
}

/// Represent the position of a user in the board.
pub type Position = u32;

/// A inclusive range of positions on the board
#[cosmwasm_schema::cw_serde]
pub struct PositionRange {
    pub start: Position,
    pub end: Position,
}

impl PositionRange {
    pub fn new(start: Position, end: Position) -> Self {
        PositionRange { start, end }
    }

    pub fn start(&self) -> Position {
        self.start
    }

    pub fn end(&self) -> Position {
        self.end
    }

    /// Create a position vector with all numbers from
    /// the start to the end.
    pub fn positions(&self) -> Vec<Position> {
        (self.start..=self.end).collect()
    }
}

#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::ExecuteFns)]
#[impl_into(ExecuteMsg)]
pub enum ControllerExecuteMsg {
    /// Allow a user to join the game and start from the first tile in the board.
    Join {},
    /// Roll the dice for the sender moving it into another tile in the board.
    RollDice {},
    // TODO: can be removed?
    /// Set status of your account
    SetStatus { status: String },
    /// Admin method: Update the configuration of the adapter
    UpdateConfig {},
}

#[cosmwasm_schema::cw_serde]
#[derive(QueryResponses, cw_orch::QueryFns)]
#[impl_into(QueryMsg)]
pub enum ControllerQueryMsg {
    #[returns(UserPositionResponse)]
    UserPosition { user_address: String },
    #[returns(ParticipantsResponse)]
    Participants {},
    // TODO: can be removed?
    #[returns(StatusResponse)]
    Status { account_id: AccountId },
    #[returns(ConfigResponse)]
    Config {},
}

/// Returns the position of a single user in the board.
#[cosmwasm_schema::cw_serde]
pub struct UserPositionResponse {
    pub position: Option<u32>,
}

/// Returns each participant position in the board.
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

#[cosmwasm_schema::cw_serde]
pub struct ConfigResponse {}

#[cosmwasm_schema::cw_serde]
pub struct StatusResponse {
    pub status: Option<String>,
}
