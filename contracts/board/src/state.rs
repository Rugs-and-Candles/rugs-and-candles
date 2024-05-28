use abstract_adapter::objects::AccountId;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cosmwasm_schema::cw_serde]
pub struct Config {}

pub type TileId = u32;

pub struct Tile {} // TODO: implement Tile struct

pub struct OngoingAction {} // TODO: implement OngoingAction struct

// default config
pub const CONFIG: Item<Config> = Item::new("config");
pub const STATUS: Map<&AccountId, String> = Map::new("status");

// game state
pub const TILES: Map<TileId, Tile> = Map::new("tiles");
pub const MANAGER_ID: Item<AccountId> = Item::new("manager_id");

pub const ONGOING_ACTIONS: Map<&Addr, OngoingAction> = Map::new("ongoing_actions");

