use abstract_adapter::objects::{chain_name::ChainName, AccountId};
use cosmwasm_std::Addr;
use cw_storage_plus::{ Item, Map, IndexedMap,};

#[cosmwasm_schema::cw_serde]
pub struct Config {}

pub type Position = u32;
// default config
pub const CONFIG: Item<Config> = Item::new("config");
pub const STATUS: Map<&AccountId, String> = Map::new("status");

// game controller state
pub const OWNER: Item<&Addr> = Item::new("owner");

pub const PARTICIPANTS: Map<&Addr, Position> = Map::new("participants");

/// A inclusive range of positions on the board
#[cosmwasm_schema::cw_serde]
pub struct PositionRange (Position, Position);

pub type BoardId = ChainName;

pub const BOARD_IDS: Map<&BoardId, PositionRange> = Map::new("board_ids");


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

pub fn board_for_position(position: Position) -> Option<BoardId> {
    BOARD_IDS
        .iter()
        .find(|(_, range)| range.positions().contains(&position))
        .map(|(board_id, _)| board_id)
}

