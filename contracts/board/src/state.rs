use abstract_adapter::objects::AccountId;
use common::{board::TileAction, game::Chains};
use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

#[cosmwasm_schema::cw_serde]
pub struct Config {
    pub current_chain: Chains,
}

pub type TileId = u32;


// default config
pub const CONFIG: Item<Config> = Item::new("config");
pub const STATUS: Map<&AccountId, String> = Map::new("status");

// game state
pub const TILES: Map<TileId, TileAction> = Map::new("tiles");
pub const CONTROLLER_ID: Item<AccountId> = Item::new("controller_id");
pub const ONGOING_ACTIONS: Map<&Addr, TileId> = Map::new("ongoing_actions");
pub const TEMP_USER: Item<Addr> = Item::new("temp_user");
