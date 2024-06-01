use abstract_adapter::objects::AccountId;
use common::{board::TileAction, game::Chains};
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cosmwasm_schema::cw_serde]
pub struct Config {
    /// Chain in which this board is deployed.
    pub current_chain: Chains,
}

pub type TileId = u32;

/// Board contract configuration.
pub const CONFIG: Item<Config> = Item::new("config");

/// Store the action associated with a tile.
pub const TILES: Map<TileId, TileAction> = Map::new("tiles");
/// Identifier of the controller contract. The controller contract is the only
/// address authorized to perform certain actions on the board.
pub const CONTROLLER_ID: Item<AccountId> = Item::new("controller_id");
/// Stores action that users has to execute before advancing to the next
/// move.
pub const ONGOING_ACTIONS: Map<&Addr, TileId> = Map::new("ongoing_actions");

// TODO: can remove?
pub const STATUS: Map<&AccountId, String> = Map::new("status");
pub const TEMP_USER: Item<Addr> = Item::new("temp_user");
