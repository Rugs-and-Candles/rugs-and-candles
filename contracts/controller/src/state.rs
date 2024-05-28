use abstract_adapter::objects::AccountId;
use cosmwasm_std::Addr;
use cw_storage_plus::{ Item, Map};

#[cosmwasm_schema::cw_serde]
pub struct Config {}

pub type Position = u32;
// default config
pub const CONFIG: Item<Config> = Item::new("config");
pub const STATUS: Map<&AccountId, String> = Map::new("status");

// game controller state
pub const OWNER: Item<&Addr> = Item::new("owner");

pub const PARTICIPANTS: Map<&Addr, Position> = Map::new("participants");

pub const BOARD_IDS: Map<&AccountId, String> = Map::new("board_ids");

