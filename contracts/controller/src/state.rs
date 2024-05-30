use abstract_adapter::objects::AccountId;
use common::controller::{Position, PositionRange};
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

/// General configuration of the controller contract.
#[cosmwasm_schema::cw_serde]
pub struct Config {}

/// Represent the position of a user in the board.

/// Store the information of the contract configuration.
pub const CONFIG: Item<Config> = Item::new("config");
pub const STATUS: Map<&AccountId, String> = Map::new("status");

/// Store the information of the owner of the contracts.
pub const OWNER: Item<&Addr> = Item::new("owner");

/// Store the information of the position of every participant in the board.
pub const PARTICIPANTS: Map<&Addr, Position> = Map::new("participants");


/// The id of the board is represented by the name of the chain.
pub type BoardId = String;

/// Stores the tiles associated to every chain in which a board is present.
// pub const BOARD_IDS: Map<&BoardId, PositionRange> = Map::new("board_ids");
pub const BOARD_IDS: Map<&str, PositionRange> = Map::new("board_ids");


