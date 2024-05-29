use abstract_adapter::objects::{chain_name::ChainName, AccountId};
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

/// General configuration of the controller contract.
#[cosmwasm_schema::cw_serde]
pub struct Config {}

/// Represent the position of a user in the board.
pub type Position = u32;

/// Store the information of the contract configuration.
pub const CONFIG: Item<Config> = Item::new("config");
// TODO: whut?
pub const STATUS: Map<&AccountId, String> = Map::new("status");

/// Store the information of the owner of the contracts.
pub const OWNER: Item<&Addr> = Item::new("owner");

/// Store the information of the position of every participant in the board.
pub const PARTICIPANTS: Map<&Addr, Position> = Map::new("participants");

/// A inclusive range of positions on the board
#[cosmwasm_schema::cw_serde]
pub struct PositionRange(Position, Position);

/// The id of the board is represented by the name of the chain.
pub type BoardId = ChainName;

/// Stores the tiles associated to every chain in which a board is present.
pub const BOARD_IDS: Map<&BoardId, PositionRange> = Map::new("board_ids");

/// Create a new position range.
impl PositionRange {
    pub fn new(start: Position, end: Position) -> Self {
        Self(start, end)
    }

    pub fn start(&self) -> Position {
        self.0
    }

    pub fn end(&self) -> Position {
        self.1
    }

    pub fn positions(&self) -> Vec<Position> {
        (self.0..=self.1).collect()
    }
}
