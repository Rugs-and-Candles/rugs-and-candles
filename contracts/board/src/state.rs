use abstract_adapter::objects::AccountId;
use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

#[cosmwasm_schema::cw_serde]
pub struct Config {}

pub type TileId = u32;

#[cosmwasm_schema::cw_serde]
pub struct TileAction {
    pub required_funds: Vec<Coin>,
    pub action_msgs: Vec<String>,
}

// default config
pub const CONFIG: Item<Config> = Item::new("config");
pub const STATUS: Map<&AccountId, String> = Map::new("status");

// game state
pub const TILES: Map<TileId, TileAction> = Map::new("tiles");
pub const MANAGER_ID: Item<AccountId> = Item::new("manager_id");
pub const ONGOING_ACTIONS: Map<&Addr, TileId> = Map::new("ongoing_actions");
